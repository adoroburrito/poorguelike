use crate::domain::map::*;

pub fn log_rooms(world: Vec<Room>) {
    println!(">>>DEBUG<<<");
    let mut count_x = 0;
    for (idx, room) in world.iter().enumerate() {
        println!("Room {}", idx + 1);
        print!("\t");
        for (_, tile) in room.tiles.iter().enumerate() {
            let face = match tile.stairs {
                None => match *tile.terrain {
                    TerrainVariant::BrickWall1(_) => "#",
                    TerrainVariant::BrickWall2(_) => "#",
                    TerrainVariant::Dirt1(_) => ".",
                    TerrainVariant::Dirt2(_) => ",",
                    TerrainVariant::Grass1(_) => "'",
                    TerrainVariant::Grass2(_) => "\"",
                },
                Some(stairs_direction) => match *stairs_direction {
                    StairsDirection::Up => "▲",
                    StairsDirection::Down => "▼",
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
}
