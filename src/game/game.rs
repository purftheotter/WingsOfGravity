use rapier2d::prelude::*;

use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::EventPump;

use crate::render_context::RenderContext;
use crate::rendering::*;

use crate::physics_world::PhysicsWorld;

use crate::assets::ShipDatabase;

use crate::game::input::*;

use crate::entity::*;

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
            RenderContext::new(canvas, screen_width, screen_height);

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

    pub fn run(&mut self) -> Result<(), String> {
        //texture creator
        let texture_creator = self.render_context.canvas.texture_creator();

        //making assets
        let mut assets = Assets::new();

        self.load_textures(&mut assets, &texture_creator)?;

        self.load_world()?;

        while self.running {

            self.handle_input();

            self.update_player_velocity();

            self.physics_world.step();

            self.render(&assets).expect("Render Failed");

            self.debug_render()?;

            self.render_context.present();
        }

        Ok(())
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
                Vec2::new(
                    0.0,
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

    pub fn update_player_velocity(&mut self) {
    }

    pub fn render(
        &mut self,
        assets: &Assets,
        ) -> Result<(), String> {
        self.render_context.clear(Color::RGB(64, 192, 255));

        for entity in self.entities.iter() {
            match entity.entity_type {
                    
                EntityType::Asteroid => {}
                EntityType::Ship => {}
                EntityType::Projectile => {}
            }
        }

        Ok(())
    }

    pub fn debug_render(&mut self) -> Result<(), String> {
        if !self.debug_mode {
            return Ok(())
        }

        self.render_context.canvas.set_draw_color(Color::RGB(100, 0, 100));

        for entity in self.entities.iter(){
            let entity_rigid_body = 
                self.physics_world.rigid_body_set
                    .get_mut(entity.rigid_body_handle)
                    .unwrap();
            let entity_collider_handles:&[ColliderHandle] = 
                entity_rigid_body.colliders();
            let mut entity_colliders = vec![];
            for handle in entity_collider_handles {
                entity_colliders
                    .push(self.physics_world.collider_set.get(*handle).unwrap());
            }

            for collider in entity_colliders {
                let shape = collider.shape();
                
                match shape.as_typed_shape() {
                    TypedShape::Ball(ball) => {
                        render_ball(
                            &mut self.render_context,
                            entity_rigid_body.position(),
                            ball
                        )?
                    }

                    TypedShape::Triangle(tri) => {
                        render_triangle(
                            &mut self.render_context,
                            entity_rigid_body.position(),
                            tri,
                            )?
                    }

                    _ => {}
                    
                }
            }
        }

        Ok(())  

    }
}
