use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Instant;

use crate::components::hitbox::Hitbox;

use crate::entity::factory::spawn_player;
use crate::entity::{Entity,EntityType};
use crate::math::shapes::Circle;
use crate::rendering::assets::Assets;
use crate::rendering::debug_render::render_circle;
use crate::rendering::debug_render::render_polygon;
use crate::systems::movement::apply_velocity;
use crate::systems::movement::rotated_velocity;
use crate::traits::renderable::Renderable;
use crate::math::vec2math::Vec2;

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    running: bool,
    screen_width: u32,
    screen_height: u32,
    debug_mode: bool,
    entities: Vec<Entity>
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

        })
    }

    pub fn run(&mut self) -> Result<(), String> {

        self.load_world();

        //texture creator
        let texture_creator = self.canvas.texture_creator();

        //making assets
        let mut assets = Assets::new();

        //loading the textures for my assets
        assets.load_texture(&texture_creator, "fighter", "assets/fighter.png")?;

        //setup for making the loop run at a consistent rate
        let mut last_frame = Instant::now();

        while self.running {

            let player = self.entities.iter_mut().find(|e| e.is_player);

            if let Some(player) = player {

                let current_frame = Instant::now();

                let dt = current_frame.duration_since(last_frame).as_secs_f32();

                last_frame = current_frame;

                self.handle_input(dt);

                self.update(dt);

                self.render(&assets);

                self.debug_render();

                self.canvas.present();
            }
        }

        Ok(())
    }

    pub fn load_world(&mut self) {
        self.entities.push(spawn_player(self.screen_width as f32 /2.0, self.screen_height as f32 /2.0))
    }

    pub fn handle_input(&mut self, dt: f32) {

        let player = self.entities.iter_mut().find(|e| e.is_player);

        if let Some(player) = player {

            //inputs
            let keyboard = self.event_pump.keyboard_state();

            if keyboard.is_scancode_pressed(Scancode::W) || keyboard.is_scancode_pressed(Scancode::Up) {
                rotated_velocity(
                    &mut player.velocity,
                    &player.transform.rotation,
                    &player.ship.as_ref().unwrap().thrust.speed,
                    dt,
                );
            }

            if keyboard.is_scancode_pressed(Scancode::A) || keyboard.is_scancode_pressed(Scancode::Left)
            {
                player.transform.rotation -= player.ship.as_ref().unwrap().turn_rate.speed * dt;
            }

            if keyboard.is_scancode_pressed(Scancode::S) || keyboard.is_scancode_pressed(Scancode::Down)
            {
                rotated_velocity(
                    &mut player.velocity,
                    &player.transform.rotation,
                    &-player.ship.as_ref().unwrap().thrust.speed,
                    dt,
                );
            }

            if keyboard.is_scancode_pressed(Scancode::D)
                || keyboard.is_scancode_pressed(Scancode::Right)
            {
                player.transform.rotation += player.ship.as_ref().unwrap().turn_rate.speed * dt;
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

        let player = self.entities.iter_mut().find(|e| e.is_player);

        if let Some(player) = player {

            apply_velocity(&mut player.transform, &player.velocity, dt);
        }
    }

    pub fn render(&mut self, assets: &Assets) {
        self.canvas.set_draw_color(Color::RGB(64, 192, 255));

        self.canvas.clear();

        let _ = self
            .canvas
            .fill_rect(Rect::new(0, 0, self.screen_width, self.screen_height));


        let player = self.entities.iter_mut().find(|e| e.is_player);

        if let Some(player) = player {


            player
                .render(&mut self.canvas, assets)
                .expect("Player Render Failed");
        }
    }

    pub fn debug_render(&mut self) {
        if !self.debug_mode {return;}

        self.canvas.set_draw_color(Color::RGB(100, 0, 100));


        let player = self.entities.iter_mut().find(|e| e.is_player);

        if let Some(player) = player {
            if let Hitbox::Circle { radius } = player.hitbox {
            render_circle(&mut self.canvas,&player.transform, &Circle::new(radius)).unwrap();
            }
        }

    }
}
