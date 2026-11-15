use std::sync::Arc;

use anyhow::Ok;
use rapier2d::utils::RotationOps;
use winit::{
    event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window
};

use rand::random;

use std::time::Instant;

use rapier2d::prelude::*;

use crate::camera::*;
use crate::sprite_renderer::*;
use crate::texture::*;
use crate::gfx::*;

use crate::system_utils::*;
use crate::physics_world::PhysicsWorld;
use crate::assets::ShipDatabase;
use crate::entity::*;
use crate::systems::ship_movement::*;
use crate::components::*;
use crate::core::*;


pub struct Game {
    pub gfx: Gfx,
    pub camera: CameraGpu,
    pub sprites: SpriteRenderer,
    pub texture_ids: TextureIds,
    pub bg_color: wgpu::Color,
    pub running: bool,
    pub debug_mode: bool,
    pub physics_world: PhysicsWorld,
    pub entities: Vec<Entity>,
    pub slots: Vec<Slot>,
    pub free_slots: Vec<u32>,
    pub player_id: Option<EntityId>,
    pub ship_database: ShipDatabase,
    pub game_seed: u32,
    pub last_frame: Instant,
}

impl Game {
    pub async fn new(window: Arc<Window>, event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        let gfx = Gfx::new(window, event_loop).await?;
        let camera = CameraGpu::new(&gfx);
        let sprites = SpriteRenderer::new(&gfx, &camera);

        let bg_color = wgpu::Color {
            r: 0.0,
            g: 0.1,
            b: 0.2,
            a: 1.0,
        };

        let physics_world = PhysicsWorld::new(Vec2::ZERO);
 
        //make ship database
        let ship_database = ShipDatabase::load().expect("Failing while trying to load ShipDatabase");

        //entities
        let entities = vec![];

        let game_seed:u32 = random();

        let slots = vec![];

        let free_slots = vec![];

        let fps_text = String::new();

        let texture_ids = TextureIds {
            fighter: None,
            green_bullet: None,
            asteroid: None,
        };

        let mut game = Self {
            gfx,
            camera,
            sprites,
            bg_color,
            entities,
            physics_world,
            running: true,
            debug_mode:false,
            player_id: None,
            ship_database,
            game_seed,
            slots,
            free_slots,
            texture_ids,
            last_frame: Instant::now(),
        };

        game.texture_ids = game.load_textures()?;
        game.load_world()?;

        Ok(game)
    }

    pub fn handle_key(
        &self,
        event_loop: &ActiveEventLoop,
        code: KeyCode,
        is_pressed: bool,
    ) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ){
        self.gfx.resize(width, height);
        self.camera.resize(&self.gfx);
    }

    pub fn tick(&mut self) -> anyhow::Result<()> {
        let current_frame = Instant::now();

        let dt = current_frame.duration_since(self.last_frame).as_secs_f32();

        self.last_frame = current_frame;

        self.update_ships();
        self.update_weapons(dt);
        
        self.physics_world.step(dt);

        let hits = self.check_projectile_impacts();
        
        //self.update_asteroids();
        
        self.remove_hit_projectile(hits);

        self.sync_position();

        self.update_camera_pos();

        self.camera.resize(&self.gfx);
        self.queue_sprites();

        self.render()?;

        //self.debug_render()?;


        Ok(())
    }

    pub fn queue_sprites(&mut self) {
        for entity in self.entities.iter() {
            if let Some(ship) = &entity.ship_component {
                self.sprites.queue_instance(
                    ship.sprite.to_queued_instance(
                        entity.position.translation.to_array(),
                        entity.position.rotation.angle(),
                    )
                );
                continue;
            }
            if let Some(projectile) = &entity.projectile {
                self.sprites.queue_instance(
                    projectile.sprite.to_queued_instance(
                        entity.position.translation.to_array(),
                        entity.position.rotation.angle(),
                    )
                );
            }
        }
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        self.gfx.window.request_redraw();

        if !self.gfx.is_surface_configured {
            return Ok(());
        }

        let output = match self.gfx.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                self.gfx.surface.configure(&self.gfx.device, &self.gfx.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost device")
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.gfx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.sprites.prepare(&self.gfx);
        
        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[
                        Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                depth_slice: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                              self.bg_color
                                          ),
                                    store: wgpu::StoreOp::Store,
                                }
                            }
                        )
                    ],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                }
            );

            self.sprites.draw(&mut render_pass, &self.camera);
        }


        self.gfx.queue.submit(std::iter::once(encoder.finish()));
        self.gfx.queue.present(output);

        Ok(())
    }

    pub fn load_textures(
        &mut self,
    ) -> anyhow::Result<TextureIds> {
        let bytes = std::fs::read(asset_path("assets/textures/fighter.png"))?;
        let fighter = self.sprites.load_texture(
            &self.gfx,
            &bytes,
            "fighter"
        )?;

        let bytes = std::fs::read(asset_path("assets/textures/green_bullet.png"))?;
        let green_bullet = self.sprites.load_texture(
            &self.gfx,
            &bytes,
            "green_bullet"
        )?;

        let bytes = std::fs::read(asset_path("assets/textures/asteroid.png"))?;
        let asteroid = self.sprites.load_texture(
            &self.gfx,
            &bytes,
            "asteroid"
        )?;

        Ok(TextureIds { 
            fighter:Some(fighter),
            green_bullet: Some(green_bullet),
            asteroid: Some(asteroid)
        })
    }

    pub fn load_world(
        &mut self,
        ) -> anyhow::Result<()> {
        
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
                &self.sprites,
                &self.texture_ids,
            );

        self.add_entity(
            player,
            true,
        );

        /*
        //spawn_asteroid
        let asteroid = spawn_asteroid(
            &mut self.physics_world.rigid_body_set,
            &mut self.physics_world.collider_set,
            &self.render_context,
            Pose2::new(
                Vec2::new(20.0, 5.0),
                0.0),
            10,
            5.0,
            self.game_seed,
            0.4,
        );

        self.add_entity(asteroid, false);
        */
        Ok(())
    }

    /*
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
    */

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
    /*
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
    */
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

                        let muzzle_vel: Vec2 = spawn_pos.rotation
                            * Vec2::new(0.0, -weapon.init_vel_mag);

                        new_projectiles.push(fire_prjectile(
                            spawn_pos,
                            ship_linvel + muzzle_vel,
                            weapon,
                            &mut self.physics_world.rigid_body_set,
                            &mut self.physics_world.collider_set,
                            &self.sprites,
                            &self.texture_ids,
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

    pub fn update_camera_pos(&mut self) {
        let Some(pos) = self.player_id
            .and_then(|id| self.get_entity(id))
            .map(|e| e.position)
        else {
            return;
        };

        self.camera.camera.center = pos.translation.to_array();
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

        //let key = ((id.slot as u64) << 32) | id.generation as u64;
        //self.render_context.asteroid_textures.remove(key);

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
