mod domain;
mod world;
use world::generators::*;

mod tools;
use tools::debug::*;

fn main() {
    // creates world (vector of rooms, connected by stairs)
    let world = prepare_world(4);
    log_rooms(world);

    /* TO-DO: drawing

    let window_height = 640.0;
    let window_width = 640.0;

    let (mut rl, _thread) = raylib::init()
        .size(window_height as i32, window_width as i32)
        .title("Poorguelike")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
    */
}
