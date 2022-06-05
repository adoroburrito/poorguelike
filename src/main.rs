use raylib::color::Color;
use raylib::prelude::*;

mod domain;
use domain::map::*;

mod world;
use world::generators::*;

fn main() {
    let window_height = 640.0;
    let window_width = 640.0;

    let (mut rl, thread) = raylib::init()
        .size(window_height as i32, window_width as i32)
        .title("Poorguelike")
        .build();

    rl.set_target_fps(60);

    let world = prepare_world(2);

    println!(">>>DEBUG<<<");

    for (idx, room) in world.iter().enumerate() {
        println!("Room {}", idx + 1);
        for (tile_idx, tile) in room.tiles.iter().enumerate() {
            println!("\tTile with index #{} ->", tile_idx);
            println!(
                "\t\tHas stairs? {}",
                match tile.stairs {
                    None => "None",
                    Some(stairs_direction) => match *stairs_direction {
                        StairsDirection::Up => "UP",
                        StairsDirection::Down => "DOWN",
                    },
                }
            );
        }
    }

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
