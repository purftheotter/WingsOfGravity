use rand::random;
use rapier2d::prelude::*;

use std::time::Instant;

use sdl2::EventPump;

use crate::render_context::RenderContext;
use crate::rendering::*;
use crate::rendering::debug::*;
use crate::physics_world::PhysicsWorld;
use crate::assets::ShipDatabase;
use crate::game::input::*;
use crate::entity::*;
use crate::systems::ship_movement::*;
use crate::components::*;
use crate::core::*;



pub struct Game {
    pub render_context: RenderContext,
    pub event_pump: EventPump,
    pub running: bool,
    pub debug_mode: bool,
    pub physics_world: PhysicsWorld,
    pub entities: Vec<Entity>,
    pub slots: Vec<Slot>,
    pub free_slots: Vec<u32>,
    pub player_id: Option<EntityId>,
    pub ship_database: ShipDatabase,
    pub game_seed: u32,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let physics_world = PhysicsWorld::new(Vec2::ZERO);

        let screen_width = 1366;
        let screen_height = 768;

        //Initilize sdl2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        //Create window
        let window = video_subsystem
            .window("WOG", screen_width, screen_height)
            .fullscreen()
            .build()
            .unwrap();

        //Create canvas
        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .unwrap();

        let assets = Assets::new();

        //create ttf_context
        let ttf_context = sdl2::ttf::init().unwrap();

        let render_context = 
            RenderContext::new(
                canvas,
                assets,
                ttf_context,
                screen_width,
                screen_height,
                16.0,
                );

        //Event pump
        let event_pump = sdl_context.event_pump().unwrap();

        //make ship database
        let ship_database = ShipDatabase::load().expect("Failing while trying to load ShipDatabase");

        //entities
        let entities = vec![];

        let game_seed:u32 = random();

        let slots = vec![];

        let free_slots = vec![];

        Ok(Self {
            render_context,
            physics_world,
            event_pump,
            entities,
            running: true,
            debug_mode:false,
            player_id: None,
            ship_database,
            game_seed,
            slots,
            free_slots,
        })
    }

    pub fn load_textures(
        &mut self,
    ) -> Result<(), String> {

        //texture creator
        let texture_creator = self.render_context.canvas.texture_creator();

        self.render_context.assets.load_texture(
            &texture_creator,
            "background",
            "assets/textures/background.png"
            )?;

        self.render_context.assets.load_texture(
            &texture_creator,
            "fighter",
            "assets/textures/fighter.png"
        )?;

        self.render_context.assets.load_texture(
            &texture_creator,
            "asteroid",
            "assets/textures/asteroid.png"
        )?;

        self.render_context.assets.load_texture(
            &texture_creator,
            "green_bullet",
            "assets/textures/green_bullet.png"
        )?;

        Ok(())
    }

    pub fn load_world(
        &mut self,
        ) -> Result<(), String> {
        
        //spawn_player
        
        let player = spawn_player(
                Pose2::new(
                    Vec2::new(0.0, 0.0),
                    0.0
                ),
                ShipClass::Scout,
                &self.ship_database,
                &mut self.physics_world.rigid_body_set,
                &mut self.physics_world.collider_set,
                &self.render_context.assets
            );

        self.add_entity(
            player,
            true,
        );


        //spawn_asteroid
        let asteroid = spawn_asteroid(
            &mut self.physics_world.rigid_body_set,
            &mut self.physics_world.collider_set,
            &self.render_context.assets,
            Pose2::new(
                Vec2::new(20.0, 5.0),
                0.0),
            4,
            5.0,
            self.game_seed,
            0.4,
        );

        self.add_entity(asteroid, false);

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {

        self.load_textures()?;

        self.load_world()?;

        //setup for making the loop run at a consistent rate
        let mut last_frame = Instant::now();

        while self.running {
            
            let current_frame = Instant::now();

            let dt = current_frame.duration_since(last_frame).as_secs_f32();

            last_frame = current_frame;

            self.handle_input();
            self.update_ships();
            self.update_weapons(dt);
            
            self.physics_world.step(dt);

            let hits = self.check_projectile_impacts();
            
            self.update_asteroids();
            
            self.remove_hit_projectile(hits);

            self.sync_position();

            self.update_camra_pos();

            self.render().expect("Render Failed");

            self.debug_render()?;

            self.render_context.present();

            for entity in self.entities.iter() {
                println!("{:?}, ", entity.id);
            }
            println!("___________");

        }

        Ok(())
    }

    pub fn handle_input(&mut self) {
        if let Some(index) = self.player_id.and_then(|id| self.index_of(id)) {
            let keyboard_state = self.event_pump.keyboard_state();
            player_input(&keyboard_state, &mut self.entities[index]);
        }

        exiting_debug_input(
            &mut self.event_pump,
            &mut self.debug_mode,
            &mut self.running
        );
    }

    pub fn update_ships(&mut self) {
        for entity in self.entities.iter_mut() {
            if let Some(ship) = &mut entity.ship_component {

                let ship_stats = self.ship_database.get(ship.class);
                apply_ship_movements(
                    &mut self.physics_world,
                    entity,
                    ship_stats
                    );
            }
        }
    }

    pub fn update_asteroids(&mut self) {
        let mut entities_to_remove = vec![];
        for entity in self.entities.iter_mut() {
            if let Some(asteroid) = &mut entity.asteroid {
                if asteroid.update_asteroid(
                    entity.rigid_body_handle,
                    &mut self.physics_world
                ){
                    entities_to_remove.push(entity.id);
                }
            }
        }
        for id in entities_to_remove {
            self.remove_entity(id);
        }
    }

    pub fn update_weapons(&mut self, dt: f32) {
        let mut new_projectiles: Vec<Entity> = vec![];

        for entity in self.entities.iter_mut() {
            if let Some(ship) = &mut entity.ship_component {
                for weapon in ship.weapons.iter_mut() {
                    weapon.tick(dt);
                }

                if ship.input.primary_fire {
                    let weapon = &mut ship.weapons[0];

                    if weapon.fire(dt) {
                        let spawn_pos = entity.position * weapon.local_offset;

                        let ship_linvel = self.physics_world.rigid_body_set
                            .get(entity.rigid_body_handle)
                            .unwrap()
                            .linvel();

                        let muzzle_vel = spawn_pos.rotation
                            * Vec2::new(0.0, -weapon.init_vel_mag);

                        new_projectiles.push(fire_prjectile(
                            spawn_pos,
                            ship_linvel + muzzle_vel,
                            weapon,
                            &mut self.physics_world.rigid_body_set,
                            &mut self.physics_world.collider_set,
                            &self.render_context.assets,
                        ));
                    }
                }
            }
        }

        for projectile in new_projectiles {
            self.add_entity(projectile, false);
        }
    }

    pub fn check_projectile_impacts(&mut self) -> Vec<EntityId> {
        let mut hit_ids = vec![];
        let mut newly_exploded = vec![];

        for entity in self.entities.iter_mut() {
            if let Some(projectile) = &mut entity.projectile {
                if projectile.exploded {
                    hit_ids.push(entity.id);
                    continue;
                }

                let has_hit = self.physics_world.narrow_phase
                    .intersection_pairs_with(projectile.collider_handle)
                    .any(|(_, _, intersecting)| intersecting);

                if has_hit {
                    projectile.exploded = true;
                    if let Some(explosion_handle) = projectile.explosion_handle {
                        newly_exploded.push(explosion_handle);
                    }
                }
            }
        }

        for handle in newly_exploded {
            if let Some(collider) = self.physics_world.collider_set.get_mut(handle) {
                collider.set_enabled(true);
            }
        }

        hit_ids
    }

    pub fn remove_hit_projectile(&mut self, hit_ids: Vec<EntityId>) {
        for id in hit_ids {
            self.remove_entity(id);
        }
    }

    pub fn sync_position(&mut self) {
        for entity in self.entities.iter_mut() {
            let rigid_body = 
                self.physics_world.rigid_body_set
                .get(entity.rigid_body_handle)
                .unwrap();
            entity.position = *rigid_body.position();
        }
    }

    pub fn update_camra_pos(&mut self) {
        let Some(pos) = self.player_id
            .and_then(|id| self.get_entity(id))
            .map(|e| e.position)
        else {
            return;
        };

        self.render_context.camera_position = pos;
        self.render_context.camera_position.rotation = Rot2::new(0.0);
    }

    pub fn render(
        &mut self,
        ) -> Result<(), String> {
        self.render_context.render_bg("background", 1.0, 5.0)?;

        for entity in self.entities.iter() {
            match entity.entity_type {
                EntityType::Ship => {
                    render_ship(
                        entity,
                        &mut self.render_context,
                        &self.ship_database
                    )?
                }
                EntityType::Projectile => {
                    render_projectile(entity, &mut self.render_context)?
                }
                EntityType::Asteroid => {}
            }
        }

        Ok(())
    }

    pub fn debug_render(&mut self) -> Result<(), String> {
        if !self.debug_mode {
            return Ok(())
        }

        for entity in self.entities.iter(){
            render_hitbox(
                entity,
                &self.physics_world,
                &mut self.render_context
                )?
        }

        Ok(())  
    }

    pub fn add_entity(
        &mut self,
        mut entity: Entity,
        is_player: bool
    ) -> EntityId {
        let index = self.entities.len();

        let id = if let Some(slot) = self.free_slots.pop() {
            let s = &mut self.slots[slot as usize];
            s.index = Some(index);
            EntityId { slot, generation: s.generation}
        //setup for making the loop run at a consistent rate
        } else {
            let slot = self.slots.len() as u32;
            self.slots.push(Slot { generation: 0, index: Some(index)});
            EntityId { slot, generation: 0 }
        };

        entity.id = id;
        self.entities.push(entity);

        if is_player {
            self.player_id = Some(id);
        }
        id
    }

    fn index_of(&self, id: EntityId) -> Option<usize> {
        let slot = self.slots.get(id.slot as usize)?;
        if slot.generation != id.generation {
            return None;
        }
        slot.index
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(self.index_of(id)?)
    }
    
    pub fn get_mut_entity(
        &mut self,
        id: EntityId
        ) -> Option<&mut Entity> {
        let index = self.index_of(id)?;
        self.entities.get_mut(index)
    }

    pub fn remove_entity(&mut self, id: EntityId) -> Option<Entity> {
        let index = self.index_of(id)?;
        let removed = self.entities.swap_remove(index);

        if index < self.entities.len() {
            let moved = self.entities[index].id;
            self.slots[moved.slot as usize].index = Some(index);
        }

        let slot = &mut self.slots[id.slot as usize];
        slot.index = None;
        slot.generation = slot.generation.wrapping_add(1);
        self.free_slots.push(id.slot);

        self.physics_world.rigid_body_set.remove(
            removed.rigid_body_handle,
            &mut self.physics_world.island_manager,
            &mut self.physics_world.collider_set,
            &mut self.physics_world.impulse_joint_set,
            &mut self.physics_world.multibody_joint_set,
            true,
        );

        if self.player_id == Some(id) {
            self.player_id = None;
        }
        Some(removed)
    }
}
