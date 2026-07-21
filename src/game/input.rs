use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::Keycode;

use crate::entity::Entity;


pub fn player_input(
    keyboard_state: &KeyboardState,
    player: &mut Entity
) {
    let player_ship = player.ship_component.as_mut().unwrap();

    player_ship.input.zero();

    if keyboard_state.is_scancode_pressed(Scancode::W) 
        || keyboard_state.is_scancode_pressed(Scancode::Up) {
        player_ship.input.thrust += 1.0;
    }

    if keyboard_state.is_scancode_pressed(Scancode::A) 
        || keyboard_state.is_scancode_pressed(Scancode::Left)
    {
        player_ship.input.turn -= 1.0;
    }

    if keyboard_state.is_scancode_pressed(Scancode::S) 
        || keyboard_state.is_scancode_pressed(Scancode::Down)
    {
        player_ship.input.thrust -= 0.1;
    }

    if keyboard_state.is_scancode_pressed(Scancode::D)
        || keyboard_state.is_scancode_pressed(Scancode::Right)
    {
        player_ship.input.turn += 1.0;
    }

    if keyboard_state.is_scancode_pressed(Scancode::Space) 
    {
        player_ship.input.primary_fire = true;
    }
}

pub fn exiting_debug_input(
    event_pump: &mut EventPump,
    debug_mode: &mut bool,
    running: &mut bool
) {

    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown { scancode: Some(Scancode::F2), .. } => {
                *debug_mode = !*debug_mode;
            }

            Event::Quit { .. } => *running = false,

            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => *running = false,

            _ => {}
        }
    }
}
