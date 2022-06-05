use crate::domain;
use crate::domain::entities::{Actor, Entity};
use crate::domain::map::{
    Room, RoomTile, StairsDirection, TerrainName, TerrainVariant, UVec2D, ALL_TERRAINS,
};
use rand::prelude::*;
use std::cmp::Ordering;

pub fn prepare_world(rooms: usize) -> Vec<Room<'static>> {
    if rooms.cmp(&2) == Ordering::Less {
        panic!("Not enough rooms! Need at least 2 - Start + Finish");
    }

    println!("Generating world with {} rooms...", rooms);
    let mut world: Vec<Room> = Vec::new();
    let last_room: usize = rooms - 1;

    for n in 0..rooms {
        println!("Generating room n #{}", n + 1);
        let mut room = make_room(UVec2D { x: 10, y: 10 });

        println!("Placing staircases for room n #{}", n + 1);
        match n.cmp(&last_room) {
            Ordering::Equal => place_staircases(&mut room, true, false),
            _ => match n.cmp(&0) {
                Ordering::Equal => place_staircases(&mut room, false, true),
                _ => place_staircases(&mut room, true, true),
            },
        }

        world.push(room);
    }

    world
}

fn place_staircases(room: &mut Room, entrance: bool, exit: bool) {
    let x = room.size.x;
    let y = room.size.y;

    let max_x = x - 1;
    let max_y = y - 1;

    let mut entrance_position = UVec2D { x: 0, y: 0 };
    let mut exit_position = UVec2D { x: 0, y: 0 };

    //place entrance
    match entrance {
        true => {
            // get a random position in the room
            let mut rng = thread_rng();
            let mut finished = false;

            while !finished {
                entrance_position.x = rng.gen_range(0..=max_x);
                entrance_position.y = rng.gen_range(0..=max_y);

                finished = !matches!(
                    *room.tiles[entrance_position.x * entrance_position.y].terrain,
                    domain::map::TerrainVariant::BrickWall1(_)
                        | domain::map::TerrainVariant::BrickWall2(_)
                )
            }

            room.tiles[entrance_position.x * entrance_position.y].stairs =
                Some(&StairsDirection::Up);
        }
        false => (),
    }

    //place exit
    match exit {
        true => {
            // get a random position in the room
            let mut rng = thread_rng();

            let mut finished = false;

            while !finished {
                exit_position.x = rng.gen_range(0..=max_x);
                exit_position.y = rng.gen_range(0..=max_y);

                let target_tile = &room.tiles[exit_position.x * exit_position.y];

                let has_wall = matches!(
                    *target_tile.terrain,
                    domain::map::TerrainVariant::BrickWall1(_)
                        | domain::map::TerrainVariant::BrickWall2(_)
                );

                let has_stairs = target_tile.stairs.is_some();

                finished = !has_wall && !has_stairs
            }

            room.tiles[exit_position.x * exit_position.y].stairs = Some(&StairsDirection::Down);
        }
        false => (),
    }
}

fn make_room(size: UVec2D) -> Room<'static> {
    let size_x = size.x;
    let size_y = size.y;

    let mut room_tiles: Vec<RoomTile> = Vec::new();

    // generate tiles for simple square room with random terrain
    for x in 1..=size_x {
        for y in 1..=size_y {
            let wall_x = match x.cmp(&size_x) {
                Ordering::Equal => true,
                _ => matches!(x.cmp(&1), Ordering::Equal),
            };

            let wall_y = match y.cmp(&size_y) {
                Ordering::Equal => true,
                _ => matches!(y.cmp(&1), Ordering::Equal),
            };

            let wall = wall_x || wall_y;

            let mut rng = rand::thread_rng();

            let variant_name: &TerrainName = match wall {
                true => {
                    // TO-DO: for now there are only 2 walls, this needs to be incremented for when there
                    // are more types of walls/more variants
                    let wall_rng: i32 = rng.gen_range(0..2);

                    match wall_rng {
                        0 => &TerrainName::BrickWall1,
                        1 => &TerrainName::BrickWall2,
                        _ => unreachable!(),
                    }
                }
                false => {
                    // TO-DO: for now there are only 4 ground variants, this needs to be incremented for when there
                    // are more variants
                    let ground_rng: i32 = rng.gen_range(0..4);

                    match ground_rng {
                        0 => &TerrainName::Dirt1,
                        1 => &TerrainName::Dirt2,
                        2 => &TerrainName::Grass1,
                        3 => &TerrainName::Grass2,
                        _ => unreachable!(),
                    }
                }
            };

            let terrain: &TerrainVariant = match *variant_name {
                TerrainName::Dirt1 => &ALL_TERRAINS[0],
                TerrainName::Dirt2 => &ALL_TERRAINS[1],
                TerrainName::Grass1 => &ALL_TERRAINS[2],
                TerrainName::Grass2 => &ALL_TERRAINS[3],
                TerrainName::BrickWall1 => &ALL_TERRAINS[4],
                TerrainName::BrickWall2 => &ALL_TERRAINS[5],
            };

            let entities: Vec<&Entity> = Vec::new();
            let actors: Vec<&Actor> = Vec::new();

            room_tiles.push(RoomTile {
                stairs: None,
                terrain,
                entities,
                actors,
            })
        }
    }

    Room {
        tiles: room_tiles,
        size: UVec2D {
            x: size_x,
            y: size_y,
        },
    }
}
