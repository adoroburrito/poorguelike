use rand::Rng;
use std::collections::HashMap;

use raylib::color::Color;
use raylib::prelude::*;

#[derive(Debug)]
enum StairsDirection {
    UP,
    DOWN,
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum TerrainVariant {
    Dirt1(Terrain),
    Dirt2(Terrain),
    Grass1(Terrain),
    Grass2(Terrain),
    BrickWall1(Terrain),
    BrickWall2(Terrain),
}

#[derive(Eq, PartialEq, Hash)]
enum TerrainName {
    Dirt1,
    Dirt2,
    Grass1,
    Grass2,
    BrickWall1,
    BrickWall2,
}

struct GraphicSettings {
    tile_size: u32,
}

struct WindowSettings {
    height: f32,
    width: f32,
}
struct Settings {
    graphic: GraphicSettings,
    window: WindowSettings,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Face {}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Terrain {
    traversable: bool,
    face: Face,
}

#[derive(Debug)]
struct Entity {
    pos_x: usize,
    pos_y: usize,
    face: Face,
}

#[derive(Debug)]
struct Actor {
    pos_x: usize,
    pos_y: usize,
    face: Face,
}

#[derive(Debug)]
struct RoomTile<'a> {
    stairs: Option<&'a StairsDirection>,
    terrain: &'a TerrainVariant,
    entities: Vec<&'a Entity>,
    actors: Vec<&'a Actor>,
}

#[derive(Debug)]
struct Room<'a> {
    tiles: Vec<RoomTile<'a>>,
    size: UVec2D,
}

#[derive(Debug)]
struct UVec2D {
    x: usize,
    y: usize,
}

fn make_room(size: UVec2D) -> Room<'static> {
    let size_x = size.x;
    let size_y = size.y;

    let mut room_tiles: Vec<RoomTile> = Vec::new();

    // generate tiles for simple square room with random terrain
    for x in 1..=size_x {
        for y in 1..=size_y {
            let wall_x = match x {
                1 => true,
                size_x => true,
                _ => false,
            };

            let wall_y = match y {
                1 => true,
                size_y => true,
                _ => false,
            };

            let wall = wall_x || wall_y;

            let mut rng = rand::thread_rng();

            let variant_name: &TerrainName = match wall {
                true => {
                    // TO-DO: for now there are only 2 walls, this needs to be incremented for when there
                    // are more types of walls/more variants
                    let wall_rng = rng.gen_range(0..2);

                    match wall_rng {
                        0 => &TerrainName::BrickWall1,
                        1 => &TerrainName::BrickWall2,
                        _ => unreachable!(),
                    }
                }
                false => {
                    // TO-DO: for now there are only 4 ground variants, this needs to be incremented for when there
                    // are more variants
                    let ground_rng = rng.gen_range(0..4);

                    match ground_rng {
                        0 => &TerrainName::Dirt1,
                        1 => &TerrainName::Dirt2,
                        2 => &TerrainName::Grass1,
                        3 => &TerrainName::Grass2,
                        _ => unreachable!(),
                    }
                }
            };

            let terrain: &TerrainVariant = match variant_name {
                &TerrainName::Dirt1 => &ALL_TERRAINS[0],
                &TerrainName::Dirt2 => &ALL_TERRAINS[1],
                &TerrainName::Grass1 => &ALL_TERRAINS[2],
                &TerrainName::Grass2 => &ALL_TERRAINS[3],
                &TerrainName::BrickWall1 => &ALL_TERRAINS[4],
                &TerrainName::BrickWall2 => &ALL_TERRAINS[5],
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

fn place_staircases(room: &mut Room, entrance: bool, exit: bool) {
    let x = room.size.x;
    let y = room.size.y;
}

fn prepare_world(rooms: usize) -> Vec<Room<'static>> {
    println!("Generating world with {} rooms...", rooms);
    let mut world: Vec<Room> = Vec::new();
    let last_room: usize = rooms - 1;

    for n in 0..rooms {
        println!("Generating room n #{}", n + 1);
        let mut room = make_room(UVec2D { x: 1000, y: 1000 });
        match n {
            0 => place_staircases(&mut room, false, true),
            last_room => place_staircases(&mut room, true, false),
            _ => place_staircases(&mut room, true, true),
        }
        world.push(room);
    }

    world
}

const ALL_TERRAINS: [TerrainVariant; 6] = [
    TerrainVariant::Dirt1(Terrain {
        traversable: true,
        face: Face {},
    }),
    TerrainVariant::Dirt2(Terrain {
        traversable: true,
        face: Face {},
    }),
    TerrainVariant::Grass1(Terrain {
        traversable: true,
        face: Face {},
    }),
    TerrainVariant::Grass2(Terrain {
        traversable: true,
        face: Face {},
    }),
    TerrainVariant::BrickWall1(Terrain {
        traversable: false,
        face: Face {},
    }),
    TerrainVariant::BrickWall2(Terrain {
        traversable: false,
        face: Face {},
    }),
];

fn main() {
    let window_height = 640.0;
    let window_width = 640.0;

    let (mut rl, thread) = raylib::init()
        .size(window_height as i32, window_width as i32)
        .title("Poorguelike")
        .build();
    rl.set_target_fps(60);

    let settings = Settings {
        graphic: GraphicSettings { tile_size: 64 },
        window: WindowSettings {
            height: window_height,
            width: window_width,
        },
    };

    let world = prepare_world(10);
    dbg!(&world[0]);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
