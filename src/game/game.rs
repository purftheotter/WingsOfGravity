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

use crate::entity::factory::spawn_player;
use crate::entity::Entity;
use crate::math::shapes::Circle;
use crate::math::shapes::Polygon;
use crate::rendering::assets::Assets;
use crate::rendering::debug_render::render_circle;
use crate::rendering::debug_render::render_polygon;
use crate::systems::movement::apply_velocity;
use crate::systems::movement::apply_acceleration;
use crate::traits::renderable::Renderable;
use crate::math::vec2math::Vec2;

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

            self.handle_input(dt);

            self.update(dt);

            self.render(&assets);

            self.debug_render();

            self.canvas.present();
        }

        Ok(())
    }

    pub fn load_world(&mut self) {
        self.entities.push(spawn_player(Vec2::new(self.screen_width as f32 /2.0, self.screen_height as f32 /2.0),&self.ship_database,ShipClass::Scout));
        self.player_index = Some(self.entities.len() -1);
    }

    pub fn handle_input(&mut self, dt: f32) {


        if let Some(player_index) = self.player_index {

            //inputs
            let keyboard = self.event_pump.keyboard_state();

            let player = &mut self.entities[player_index];

            let player_ship = player.ship_component.as_ref().unwrap();
            let player_ship_stats = self.ship_database.get(player_ship.class);

            if keyboard.is_scancode_pressed(Scancode::W) || keyboard.is_scancode_pressed(Scancode::Up) {
                apply_acceleration(
                    &mut player.velocity,
                    &player.transform.rotation,
                    &player_ship_stats.thrust,
                    dt,
                );
            }

            if keyboard.is_scancode_pressed(Scancode::A) || keyboard.is_scancode_pressed(Scancode::Left)
            {
                player.velocity.angular -= player_ship_stats.angular_thrust * dt;
            }

            if keyboard.is_scancode_pressed(Scancode::S) || keyboard.is_scancode_pressed(Scancode::Down)
            {
                apply_acceleration(
                    &mut player.velocity,
                    &player.transform.rotation,
                    &-player_ship_stats.thrust,
                    dt,
                );
            }

            if keyboard.is_scancode_pressed(Scancode::D)
                || keyboard.is_scancode_pressed(Scancode::Right)
            {
                player.velocity.angular += player_ship_stats.thrust * dt;
            }
        }

        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { scancode: Some(Scancode::F2), .. } => {
                    self.debug_mode = !self.debug_mode;
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

    pub fn update(&mut self, dt: f32) {

        if let Some(player_index) = self.player_index {
            let player = &mut self.entities[player_index];
            let player_ship = player.ship_component.as_ref().unwrap();
            let player_ship_stats = self.ship_database.get(player_ship.class);

            apply_velocity(&mut player.transform, &player.velocity, dt);

            player.velocity.angular *= (-player_ship_stats.angular_velocity_dampener * dt).exp();

            player.velocity.angular = player.velocity.angular.clamp(-500.0, 500.0);

        }
    }

    pub fn render(&mut self, assets: &Assets) {
        self.canvas.set_draw_color(Color::RGB(64, 192, 255));

        self.canvas.clear();

        let _ = self
            .canvas
            .fill_rect(Rect::new(0, 0, self.screen_width, self.screen_height));


        if let Some(player_index) = self.player_index {

            let player = &mut self.entities[player_index];

            player
                .render(&mut self.canvas, assets)
                .expect("Player Render Failed");
        }
    }

    pub fn debug_render(&mut self) {
        if !self.debug_mode {return;}

        self.canvas.set_draw_color(Color::RGB(100, 0, 100));

        if let Some(player_index) = self.player_index {
            let player = &mut self.entities[player_index];

            match &player.hitbox {
                Hitbox::Circle { radius } => {
                    render_circle(
                        &mut self.canvas,
                        &player.transform,
                        &Circle::new(*radius),
                    ).unwrap();
                }

                Hitbox::Polygon { polygon } => {
                    render_polygon(
                        &mut self.canvas,
                        &player.transform,
                        &polygon,
                    ).unwrap();
                }
            }

        }

    }
}
