use rapier2d::prelude::*;

use std::time::Instant;

use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::EventPump;

use crate::render_context::RenderContext;
use crate::rendering::*;
use crate::rendering::debug::*;

use crate::physics_world::PhysicsWorld;

use crate::assets::ShipDatabase;

use crate::game::input::*;

use crate::entity::*;

use crate::systems::ship_movement::*;

use crate::components::ShipClass;

pub struct Game {
    pub render_context: RenderContext,
    pub event_pump: EventPump,
    pub running: bool,
    pub debug_mode: bool,
    pub physics_world: PhysicsWorld,
    pub next_entity_id: usize,
    pub entities: Vec<Entity>,
    pub player_index: Option<usize>,
    pub ship_database: ShipDatabase,
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

        let render_context = 
            RenderContext::new(
                canvas,
                screen_width,
                screen_height,
                16.0,
                );

        //Event pump
        let event_pump = sdl_context.event_pump().unwrap();

        //make ship database
        let ship_database = ShipDatabase::load().expect("Failing while trying to load ShipDatabase");

        //next_entity_id
        let next_entity_id:usize = 0;

        //entities
        let entities = vec![];


        Ok(Self {
            render_context,
            physics_world,
            event_pump,
            next_entity_id,
            entities,
            running: true,
            debug_mode:false,
            player_index: None,
            ship_database,

        })
    }

    pub fn load_textures(
        &mut self,
        assets: &mut Assets,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        assets.load_texture(
            texture_creator,
            "fighter",
            "assets/textures/fighter.png"
        )?;

        assets.load_texture(
            texture_creator,
            "asteroid",
            "assets/textures/asteroid.png"
        )?;

        assets.load_texture(
            texture_creator,
            "green_bullet",
            "assets/textures/green_bullet.png"
        )?;

        Ok(())
    }

    pub fn load_world(
        &mut self,
        ) -> Result<(), String> {

        //spawn_player
        self.entities.push(
            spawn_player(
                self.next_entity_id,
                Pose2::new(
                    Vec2::new(0.0, 0.0),
                    0.0
                ),
                ShipClass::Scout,
                &self.ship_database,
                &mut self.physics_world.rigid_body_set,
                &mut self.physics_world.collider_set,
            )
        );
        self.player_index = Some(self.next_entity_id);

        self.next_entity_id += 1;

        //spawn_asteroid
        self.entities.push(
            spawn_asteroid(
                self.next_entity_id,
                &mut self.physics_world.rigid_body_set,
                &mut self.physics_world.collider_set,
                Pose2::new(
                    Vec2::new(10.0, 10.0),
                    0.0),
                3,
                10.0,
                0.0,
            )
        );

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        //texture creator
        let texture_creator = self.render_context.canvas.texture_creator();

        //making assets
        let mut assets = Assets::new();

        self.load_textures(&mut assets, &texture_creator)?;

        self.load_world()?;

        //setup for making the loop run at a consistent rate
        let mut last_frame = Instant::now();

        while self.running {
            
            let current_frame = Instant::now();

            let dt = current_frame.duration_since(last_frame).as_secs_f32();

            last_frame = current_frame;

            self.handle_input();
            self.update_ships(dt);
            self.update_weapons(dt);
            
            self.physics_world.step(dt);

            let hits = self.check_projectile_impacts();
            
            self.update_asteroids();
            
            self.remove_hit_projectile(hits);

            self.sync_position();

            self.render(&assets).expect("Render Failed");

            self.debug_render()?;

            self.render_context.present();
        }

        Ok(())
    }

    pub fn handle_input(&mut self) {
        if let Some(player_index) = self.player_index {
            let mut player = &mut self.entities[player_index];
            let keyboard_state = &self.event_pump.keyboard_state();
                player_input(&keyboard_state, &mut player);
        }

        exiting_debug_input(
            &mut self.event_pump,
            &mut self.debug_mode,
            &mut self.running
        );
    }

    pub fn update_ships(&mut self, dt:f32) {
        for entity in self.entities.iter_mut() {
            if let Some(ship) = &mut entity.ship_component {

                for weapon in ship.weapons.iter_mut() {
                    weapon.tick(dt);
                }

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
        for entity in self.entities.iter_mut() {
            if let Some(asteroid) = &mut entity.asteroid {
                if !asteroid.update_asteroid(
                    entity.rigid_body_handle,
                    &mut self.physics_world
                ){
                    println!("delete asteroid entity");
                }
                
            }

        }
    }

    pub fn update_weapons(&mut self, dt: f32) {
        let mut new_projectiles:Vec<Entity> = vec![];
        for entity in self.entities.iter_mut() {
            if let Some(ship) = &mut entity.ship_component {
                if ship.input.primary_fire {
                    let weapon = &mut ship.weapons[0];
                    if weapon.fire(dt) {
                        let entity = fire_prjectile(
                            self.next_entity_id,
                            entity.position * weapon.local_offset,
                            Some(weapon.init_vel_mag),
                            weapon,
                            &mut self.physics_world.rigid_body_set,
                            &mut self.physics_world.collider_set
                        );

                        new_projectiles.push(entity);
                        self.next_entity_id += 1;

                    }
                }
            }
        }
        self.entities.append(&mut new_projectiles);
    }

    pub fn check_projectile_impacts(&mut self) -> Vec<usize> {
        let mut hit_indices = vec![];
        let mut newly_exploded = vec![];

        for (i, entity) in self.entities.iter_mut().enumerate() {
            if let Some(projectile) = &mut entity.projectile {

                if projectile.exploded {
                    hit_indices.push(i);
                    continue;
                }

                let has_hit = self.physics_world.narrow_phase
                    .intersection_pairs_with(projectile.collider_handle)
                    .any(|(_, _, intersecting)| intersecting);

                if has_hit {
                    projectile.exploded = true;
                    if let Some(explosion_handle) = 
                        projectile.explosion_handle {
                            newly_exploded.push(
                                explosion_handle
                            );
                        }
                }
            }
        }

        for collider in newly_exploded {
            let collider = 
                self.physics_world.collider_set.get_mut(collider).unwrap();
            collider.set_enabled(true);
        }

        hit_indices
    }

    pub fn remove_hit_projectile(&mut self, hit_indices: Vec<usize>) {
        for &i in hit_indices.iter().rev() {
            let entity = self.entities.remove(i);
            self.physics_world.rigid_body_set.remove(
                entity.rigid_body_handle,
                &mut self.physics_world.island_manager,
                &mut self.physics_world.collider_set,
                &mut self.physics_world.impulse_joint_set,
                &mut self.physics_world.multibody_joint_set,
                true,
            );
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

    pub fn render(
        &mut self,
        assets: &Assets,
        ) -> Result<(), String> {
        self.render_context.clear(Color::RGB(0, 0, 20));

        for entity in self.entities.iter() {
            match entity.entity_type {
                EntityType::Ship => {
                    render_ship(
                        entity,
                        &mut self.render_context,
                        assets,
                        &self.ship_database
                        )?
                }
                EntityType::Asteroid => {}
                EntityType::Projectile => {}
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
}
