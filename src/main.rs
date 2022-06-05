mod domain;
use domain::map::*;

mod world;
use world::generators::*;

fn main() {
    // creates world (vector of rooms, connected by stairs)
    let world = prepare_world(3);

    println!(">>>DEBUG<<<");
    let mut count_x = 0;
    for (idx, room) in world.iter().enumerate() {
        println!("Room {}", idx + 1);
        print!("\t");
        for (tile_idx, tile) in room.tiles.iter().enumerate() {
            let face = match tile.stairs {
                None => "#",
                Some(stairs_direction) => match *stairs_direction {
                    StairsDirection::Up => "▼",
                    StairsDirection::Down => "▲",
                },
            };

            print!("{}", face);
            if count_x == room.size.x - 1 {
                print!("\n\t");
                count_x = 0;

                continue;
            }

            count_x += 1;
        }
        println!();
    }

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
