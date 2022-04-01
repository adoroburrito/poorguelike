use rand::prelude::*;
use raylib::prelude::*;

#[derive(Clone)]
struct Entity {
    id: i32,
    face: char,
    pos_x: i32,
    pos_y: i32,
}

#[derive(Clone, Copy)]
struct GraphicSettings {
    tile_height: i32,
    tile_width: i32,
    font_offset_x: i32,
    font_offset_y: i32,
    font_size: i32,
    rows: i32,
    columns: i32,
}

#[derive(Clone, Copy)]
struct WindowSettings {
    height: i32,
    width: i32,
}

#[derive(Clone, Copy)]
struct Settings {
    window: WindowSettings,
    graphic: GraphicSettings,
}

#[derive(Clone)]
struct Game {
    entities: Vec<Entity>,
    buildings: Vec<Entity>,
    settings: Settings,
}

fn update_player(entity: &mut Entity, d: &mut RaylibDrawHandle, game: &Game) {
    if d.is_key_pressed(KeyboardKey::KEY_H) && can_move(entity, "LEFT", game) {
        entity.pos_x -= 1;
    }

    if d.is_key_pressed(KeyboardKey::KEY_L) && can_move(entity, "RIGHT", game) {
        entity.pos_x += 1;
    }

    if d.is_key_pressed(KeyboardKey::KEY_J) && can_move(entity, "DOWN", game) {
        entity.pos_y += 1;
    }

    if d.is_key_pressed(KeyboardKey::KEY_K) && can_move(entity, "UP", game) {
        entity.pos_y -= 1;
    }
}

fn can_move(entity: &Entity, side: &str, game: &Game) -> bool {
    if side == "LEFT" {
        // is there any entity that is directly to the left of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos_x == (entity.pos_x - 1) && game_entity.pos_y == entity.pos_y {
                return false;
            }
        }

        // is there any building that is directly to the left of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos_x == (entity.pos_x - 1) && game_building.pos_y == entity.pos_y {
                return false;
            }
        }
    }

    if side == "RIGHT" {
        // is there any entity that is directly to the right of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos_x == (entity.pos_x + 1) && game_entity.pos_y == entity.pos_y {
                return false;
            }
        }

        // is there any building that is directly to the right of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos_x == (entity.pos_x + 1) && game_building.pos_y == entity.pos_y {
                return false;
            }
        }
    }

    if side == "UP" {
        // is there any entity that is directly to the up of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos_x == entity.pos_x && game_entity.pos_y == (entity.pos_y - 1) {
                return false;
            }
        }

        // is there any building that is directly to the up of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos_x == entity.pos_x && game_building.pos_y == (entity.pos_y - 1) {
                return false;
            }
        }
    }

    if side == "DOWN" {
        // is there any entity that is directly to the down of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos_x == entity.pos_x && game_entity.pos_y == (entity.pos_y + 1) {
                return false;
            }
        }

        // is there any entity that is directly to the down of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos_x == entity.pos_x && game_building.pos_y == (entity.pos_y + 1) {
                return false;
            }
        }
    }

    true
}

fn move_random(entity: &mut Entity, game: &Game) {
    let mut rng = rand::thread_rng();
    let random_move: u8 = rng.gen_range(0..5);

    if random_move == 1 && can_move(&entity, "LEFT", game) {
        entity.pos_x -= 1;
    }

    if random_move == 2 && can_move(&entity, "UP", game) {
        entity.pos_y -= 1;
    }

    if random_move == 3 && can_move(&entity, "DOWN", game) {
        entity.pos_y += 1;
    }

    if random_move == 4 && can_move(&entity, "RIGHT", game) {
        entity.pos_x += 1;
    }
}

fn update_entities(d: &mut RaylibDrawHandle, game: &Game, settings: &Settings) -> Game {
    let mut new_entities = game.entities.to_vec();

    for entity in new_entities.iter_mut() {
        if entity.id == 1 {
            update_player(entity, d, game)
        }

        if entity.id == 2 {
            move_random(entity, game)
        }
    }

    return Game {
        entities: new_entities,
        settings: settings.to_owned(),
        buildings: game.buildings.to_owned(),
    };
}

fn draw_grid(d: &mut RaylibDrawHandle, settings: &Settings) {
    for n in 0..settings.graphic.columns {
        d.draw_line(
            n * settings.graphic.tile_width,
            0,
            n * settings.graphic.tile_width,
            settings.window.height,
            Color::from_hex("222222").unwrap(),
        );
    }

    for n in 0..settings.graphic.rows {
        d.draw_line(
            0,
            n * settings.graphic.tile_height,
            settings.window.width,
            n * settings.graphic.tile_height,
            Color::from_hex("222222").unwrap(),
        );
    }
}

fn draw_entities(d: &mut RaylibDrawHandle, game: &Game, settings: &Settings) {
    for building in game.buildings.iter() {
        d.draw_text(
            &building.face.to_string(),
            (building.pos_x * settings.graphic.tile_width) + settings.graphic.font_offset_x,
            (building.pos_y * settings.graphic.tile_height) + settings.graphic.font_offset_y,
            settings.graphic.font_size,
            Color::from_hex("555555").unwrap(),
        )
    }

    for entity in game.entities.iter() {
        d.draw_text(
            &entity.face.to_string(),
            (entity.pos_x * settings.graphic.tile_width) + settings.graphic.font_offset_x,
            (entity.pos_y * settings.graphic.tile_height) + settings.graphic.font_offset_y,
            settings.graphic.font_size,
            Color::WHITE,
        )
    }
}

fn draw_game(d: &mut RaylibDrawHandle, game: &Game, settings: &Settings) {
    draw_grid(d, &settings);
    draw_entities(d, game, &settings);
}

fn get_settings(
    window_height: i32,
    window_width: i32,
    window_rows: i32,
    window_columns: i32,
    font_size: i32,
) -> Settings {
    let window_settings: WindowSettings = WindowSettings {
        height: window_height,
        width: window_width,
    };

    let tile_width = window_settings.width / window_columns;
    let tile_height = window_settings.height / window_rows;

    let graphic_settings: GraphicSettings = GraphicSettings {
        tile_width: tile_width,
        tile_height: tile_height,
        font_offset_x: 10,
        font_offset_y: 5,
        font_size: font_size,
        rows: window_rows,
        columns: window_columns,
    };

    return Settings {
        window: window_settings,
        graphic: graphic_settings,
    };
}

fn get_starting_entities() -> Vec<Entity> {
    let player: Entity = Entity {
        id: 1,
        face: '@',
        pos_x: 1,
        pos_y: 1,
    };

    let npc: Entity = Entity {
        id: 2,
        face: 'L',
        pos_x: 3,
        pos_y: 3,
    };

    return vec![player, npc];
}

fn get_starting_buildings(settings: &Settings) -> Vec<Entity> {
    let mut buildings: Vec<Entity> = Vec::new();
    let mut this_is_dumb = 999;

    for n in 0..settings.graphic.columns {
        buildings.push(Entity {
            id: this_is_dumb,
            face: '#',
            pos_x: n,
            pos_y: 0,
        });

        this_is_dumb += 1;

        buildings.push(Entity {
            id: this_is_dumb,
            face: '#',
            pos_x: n,
            pos_y: settings.graphic.columns - 1,
        });

        this_is_dumb += 1;
    }

    for n in 0..settings.graphic.rows {
        buildings.push(Entity {
            id: this_is_dumb,
            face: '#',
            pos_x: 0,
            pos_y: n,
        });

        this_is_dumb += 1;

        buildings.push(Entity {
            id: this_is_dumb,
            face: '#',
            pos_x: settings.graphic.rows - 1,
            pos_y: n,
        });

        this_is_dumb += 1;
    }

    return buildings;
}

fn user_interacted(d: &RaylibDrawHandle) -> bool {
    if d.is_key_pressed(KeyboardKey::KEY_H)
        || d.is_key_pressed(KeyboardKey::KEY_J)
        || d.is_key_pressed(KeyboardKey::KEY_K)
        || d.is_key_pressed(KeyboardKey::KEY_L)
    {
        return true;
    }

    false
}

fn main() {
    let settings: Settings = get_settings(768, 768, 20, 20, 26);
    let entities: Vec<Entity> = get_starting_entities();
    let buildings: Vec<Entity> = get_starting_buildings(&settings);

    let mut game = Game {
        entities: entities,
        settings: settings,
        buildings: buildings,
    };

    let (mut rl, thread) = raylib::init()
        .size(settings.window.width, settings.window.height)
        .title("Poorguelike")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if user_interacted(&mut d) {
            let updated_game: Game = update_entities(&mut d, &game, &settings);

            draw_game(&mut d, &updated_game, &settings);

            game = updated_game;
        } else {
            draw_game(&mut d, &game, &settings);
        }
    }
}
