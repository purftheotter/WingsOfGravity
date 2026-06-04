use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::libc::SECCOMP_RET_KILL;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Instant;

use crate::entities::ships::Fighter;
use crate::math::shapes::Circle;
use crate::rendering::assets::Assets;
use crate::rendering::debug_render::render_circle;
use crate::rendering::debug_render::render_polygon;
use crate::systems::movement::apply_velocity;
use crate::systems::movement::rotated_velocity;
use crate::traits::renderable::Renderable;
use crate::math::vec2math::Vec2;
use crate::math::shapes::Polygon;
use crate::systems::collision::two_circle_collision;

pub struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    player: Fighter,
    running: bool,
    screen_width: u32,
    screen_height: u32,
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

        //Player
        let player = Fighter::new(
            (screen_width / 2) as f32,
            (screen_height / 2) as f32,
            0.0,
            2.0,
            2.0,
            "fighter",
        );

        Ok(Self {
            canvas,
            event_pump,
            player,
            running: true,
            screen_width,
            screen_height,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        //texture creator
        let texture_creator = self.canvas.texture_creator();

        //making assets
        let mut assets = Assets::new();

        //loading the textures for my assets
        assets.load_texture(&texture_creator, "fighter", "assets/fighter.png")?;

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

    pub fn handle_input(&mut self, dt: f32) {
        //inputs
        let keyboard = self.event_pump.keyboard_state();

        if keyboard.is_scancode_pressed(Scancode::W) || keyboard.is_scancode_pressed(Scancode::Up) {
            rotated_velocity(
                &mut self.player.velocity,
                &self.player.transform.rotation,
                self.player.thrust.speed,
                dt,
            );
        }

        if keyboard.is_scancode_pressed(Scancode::A) || keyboard.is_scancode_pressed(Scancode::Left)
        {
            self.player.transform.rotation -= self.player.turn_rate.speed * dt;
        }

        if keyboard.is_scancode_pressed(Scancode::S) || keyboard.is_scancode_pressed(Scancode::Down)
        {
            rotated_velocity(
                &mut self.player.velocity,
                &self.player.transform.rotation,
                -self.player.thrust.speed,
                dt,
            );
        }

        if keyboard.is_scancode_pressed(Scancode::D)
            || keyboard.is_scancode_pressed(Scancode::Right)
        {
            self.player.transform.rotation += self.player.turn_rate.speed * dt;
        }

        for event in self.event_pump.poll_iter() {
            match event {
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
        apply_velocity(&mut self.player.transform, &self.player.velocity, dt);
    }

    pub fn render(&mut self, assets: &Assets) {
        self.canvas.set_draw_color(Color::RGB(64, 192, 255));

        self.canvas.clear();

        let _ = self
            .canvas
            .fill_rect(Rect::new(0, 0, self.screen_width, self.screen_height));

        self.player
            .render(&mut self.canvas, assets)
            .expect("Player Render Failed");

    }

    pub fn debug_render(&mut self) {
        let c1 = Circle::new(Vec2::new(20.0, 20.0), 10.0);
        let c2 = Circle::new(Vec2::new(20.0, 30.0), 10.0);

        if two_circle_collision(&c1, &c2) {
            self.canvas.set_draw_color(Color::RGB(255,0,0));
        }else {
            self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        }

        render_circle(&mut self.canvas, &c1).unwrap();
        render_circle(&mut self.canvas, &c2).unwrap();

        

    }
}
