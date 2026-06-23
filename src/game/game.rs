use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Instant;

use crate::assets::ship_database::ShipDatabase;
use crate::components::ShipClass;
use crate::components::hitbox::Hitbox;

use crate::entity::EntityType;
use crate::entity::factory::spawn_asteroid;
use crate::entity::factory::spawn_player;
use crate::entity::Entity;
use crate::rendering::assets::Assets;
use crate::rendering::entities::render_asteriod;
use crate::rendering::primitives::render_circle;
use crate::rendering::primitives::render_polygon;
use crate::rendering::entities::render_ship;
use crate::systems::collision;
use crate::systems::physics::{apply_forward_thrust,apply_torque,apply_velocity};
use crate::math::vec2::Vec2;

pub struct Game {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub running: bool,
    pub screen_width: u32,
    pub screen_height: u32,
    pub debug_mode: bool,
    pub entities: Vec<Entity>,
    pub player_index: Option<usize>,
    pub ship_database: ShipDatabase,
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let screen_width = 1920;
        let screen_height = 1080;

        //Initilize sdl2
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        //Create window
        let window = video_subsystem
            .window("Rust!", screen_width, screen_height)
            .fullscreen()
            .build()
            .unwrap();

        //Create canvas
        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .unwrap();

        let clear_color = Color::RGB(64, 192, 255);
        canvas.set_draw_color(clear_color);

        //Event pump
        let event_pump = sdl_context.event_pump().unwrap();

        //make ship database
        let ship_database = ShipDatabase::load().expect("Failing while trying to load ShipDatabase");

        //entities
        let entities = vec![];


        Ok(Self {
            canvas,
            event_pump,
            entities,
            running: true,
            screen_width,
            screen_height,
            debug_mode:false,
            player_index: None,
            ship_database,

        })
    }

    pub fn run(&mut self) -> Result<(), String> {

        self.load_world();

        //texture creator
        let texture_creator = self.canvas.texture_creator();

        //making assets
        let mut assets = Assets::new();

        //loading the textures for my assets
        assets.load_texture(&texture_creator, "fighter", "assets/textures/fighter.png")?;

        //setup for making the loop run at a consistent rate
        let mut last_frame = Instant::now();

        while self.running {

            let current_frame = Instant::now();

            let dt = current_frame.duration_since(last_frame).as_secs_f32();

            last_frame = current_frame;

            self.handle_input();

            self.update_player_velocity(dt);

            self.update_transform(dt);

            self.update_collisions();

            self.render(&assets).expect("Render Failed");

            self.debug_render()?;

            self.canvas.present();
        }

        Ok(())
            
    }

    pub fn load_world(&mut self) {
        self.entities.push(
            spawn_player(
                Vec2::new(
                    self.screen_width as f32 /2.0,
                    self.screen_height as f32 /2.0
                ),
                ShipClass::Scout,
            )
        );
        self.player_index = Some(self.entities.len() -1);

        self.entities.push(
            spawn_asteroid(
                Vec2::new(400.0, 400.0),
                1.0,
            )
        );
    }

    pub fn handle_input(&mut self) {

        if let Some(player_index) = self.player_index {

            //inputs
            let keyboard = self.event_pump.keyboard_state();

            let player = &mut self.entities[player_index];

            let player_ship = player.ship_component.as_mut().unwrap();

            player_ship.input.zero();

            if keyboard.is_scancode_pressed(Scancode::W) || keyboard.is_scancode_pressed(Scancode::Up) {
                player_ship.input.thrust += 1.0;
            }

            if keyboard.is_scancode_pressed(Scancode::A) || keyboard.is_scancode_pressed(Scancode::Left)
            {
                player_ship.input.turn -= 1.0;
            }

            if keyboard.is_scancode_pressed(Scancode::S) || keyboard.is_scancode_pressed(Scancode::Down)
            {
                player_ship.input.thrust -= 0.1;
            }

            if keyboard.is_scancode_pressed(Scancode::D)
                || keyboard.is_scancode_pressed(Scancode::Right)
            {
                player_ship.input.turn += 1.0;
            }
        }

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { scancode: Some(Scancode::F2), .. } => {
                    self.debug_mode = !self.debug_mode;
                }

                Event::KeyDown { scancode: Some(Scancode::E), ..} => {
                    for entity in self.entities.iter_mut() {
                        if entity.entity_type == EntityType::Asteroid {
                            if let Some(asteroid) = entity.asteroid.as_mut() {
                                asteroid.root.subdivide(asteroid.max_depth);
                            }                            
                        }
                        
                    }
                }

                Event::Quit { .. } => self.running = false,

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.running = false,

                _ => {}
            }
        }
    }

    pub fn update_player_velocity(&mut self, dt: f32) {

        if let Some(player_index) = self.player_index {
            let player = &mut self.entities[player_index];
            let player_ship = player.ship_component.as_ref().unwrap();
            let player_ship_stats = self.ship_database.get(player_ship.class);

            //update velocity
            apply_forward_thrust(
                &mut player.velocity,
                &player.transform.rotation,
                &(player_ship_stats.thrust * player_ship.input.thrust),
                &player_ship_stats.mass,
                &dt
            );
            apply_torque(
                &mut player.velocity,
                &(player_ship_stats.torque * player_ship.input.turn),
                &player_ship_stats.inertia,
                &dt
            );

        }
    }

    pub fn update_transform(&mut self, dt: f32) {
        for entity in self.entities.iter_mut() {
            apply_velocity(&mut entity.transform, &entity.velocity, &dt);
        }
    }

    pub fn update_collisions(&mut self) {
        collision::update_player_collisions(
            &mut self.entities,
            self.player_index.unwrap(),
            &self.ship_database
        );
    }

    pub fn render(&mut self, assets: &Assets) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(64, 192, 255));

        self.canvas.clear();

        let _ = self
            .canvas
            .fill_rect(
                Rect::new(
                    0,
                    0,
                    self.screen_width,
                    self.screen_height
                )
            );


        if let Some(player_index) = self.player_index {

            let player = &self.entities[player_index];

            render_ship(
                player,
                &mut self.canvas,
                assets,
                &self.ship_database
            )?;
        }

        for entity in self.entities.iter() {
            if entity.entity_type == EntityType::Asteroid {
                render_asteriod(entity, &mut self.canvas, assets)?;
            }
        }

        Ok(())
    }

    pub fn debug_render(&mut self) -> Result<(), String> {
        if !self.debug_mode {
            return Ok(())
        }

        self.canvas.set_draw_color(Color::RGB(100, 0, 100));

        if let Some(player_index) = self.player_index {
            let player = &mut self.entities[player_index];

            match &self.ship_database.get(player.ship_component.as_ref().unwrap().class).hitbox {
                Hitbox::Circle { circle } => {
                    render_circle(
                        &mut self.canvas,
                        &player.transform,
                        &circle,
                    )?;
                }

                Hitbox::Polygon { polygon } => {
                    render_polygon(
                        &mut self.canvas,
                        &player.transform,
                        &polygon,
                    )?;
                }

            }

        }

        Ok(())  

    }
}
