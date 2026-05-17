use sdl2::event::Event;
use sdl2::image;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::instant;

mod components;
mod entities;
mod game;
mod rendering;
mod systems;
mod traits;

use entities::ships::Fighter;
use game::game::Game;
use rendering::assets::Assets;
use systems::movement::apply_velocity;
use systems::movement::rotated_velocity;
use traits::renderable::Renderable;

fn main() -> Result<(), String> {
    let screen_width = 1920;
    let screen_height = 1080;

    //Initilize sdl2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    //Create window
    let window = video_subsystem
        .window("Rust!", screen_width, screen_height)
        .fullscreen_desktop()
        .build()
        .unwrap();

    //Create canvas
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas
        .set_logical_size(screen_width, screen_height)
        .unwrap();

    let screen_area = Rect::new(0, 0, screen_width, screen_height);
    let clear_color = Color::RGB(64, 192, 255);
    canvas.set_draw_color(clear_color);

    //texture creator
    let _image_context = image::init(image::InitFlag::PNG)?;
    let texture_creator = canvas.texture_creator();

    //making assets
    let mut assets = Assets::new();

    //loading the textures for my assets
    assets.load_texture(&texture_creator, "fighter", "assets/fighter.png")?;

    //making an asset
    let mut fighter = Fighter::new(
        (screen_width / 2) as f32,
        (screen_height / 2) as f32,
        0.0,
        2.0,
        2.0,
        "fighter",
    );

    //setup for making the loop run at a consistent rate
    let mut last_frame = Instant::now();

    //Event pump
    let mut event_queue = sdl_context.event_pump().unwrap();

    //Main loop
    let mut running = true;
    while running {
        let current_frame = Instant::now();

        let dt = current_frame.duration_since(last_frame).as_secs_f32();

        last_frame = current_frame;

        //one off inputs and quiting properly
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }

                _ => {}
            }
        }
        //inputs
        let keyboard = event_queue.keyboard_state();

        if keyboard.is_scancode_pressed(Scancode::W) || keyboard.is_scancode_pressed(Scancode::Up) {
            rotated_velocity(
                &mut fighter.velocity,
                &fighter.transform.rotation,
                fighter.thrust.speed,
                dt,
            );
        }

        if keyboard.is_scancode_pressed(Scancode::A) || keyboard.is_scancode_pressed(Scancode::Left)
        {
            fighter.transform.rotation -= fighter.turn_rate.speed * dt;
        }

        if keyboard.is_scancode_pressed(Scancode::S) || keyboard.is_scancode_pressed(Scancode::Down)
        {
            rotated_velocity(
                &mut fighter.velocity,
                &fighter.transform.rotation,
                -fighter.thrust.speed,
                dt,
            );
        }

        if keyboard.is_scancode_pressed(Scancode::D)
            || keyboard.is_scancode_pressed(Scancode::Right)
        {
            fighter.transform.rotation += fighter.turn_rate.speed * dt;
        }

        apply_velocity(&mut fighter.transform, &fighter.velocity, dt);

        canvas.clear();

        let _ = canvas.fill_rect(screen_area);

        fighter.render(&mut canvas, &assets)?;

        canvas.present();
    }

    Ok(())
}
