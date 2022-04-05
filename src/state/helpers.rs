use crate::state::domain::{
    Entity, EntityCharacteristics, EntityMode, EntityRelationship, Game, GraphicSettings, Position,
    Settings, WindowSettings,
};

use crate::draw::TileNames;
use rand::Rng;
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::KeyboardKey;
use uuid::Uuid;

pub fn get_entities_in_tile(game: &Game, tile_pos: &Position) -> Vec<Entity> {
    let mut to_return: Vec<Entity> = Vec::new();

    for building in &game.buildings {
        if building.pos.x == tile_pos.x && building.pos.y == tile_pos.y {
            to_return.push(building.to_owned());
        }
    }

    for entity in &game.entities {
        if entity.pos.x == tile_pos.x && entity.pos.y == tile_pos.y {
            to_return.push(entity.to_owned());
        }
    }

    to_return
}

pub fn gen_random_position(entities: &Vec<Entity>, max_cols: i32, max_rows: i32) -> Position {
    let mut rng = rand::thread_rng();

    let mut found = false;
    let mut final_position = Position { x: 0, y: 0 };

    while found == false {
        let random_x: i32 = rng.gen_range(1..max_cols);
        let random_y: i32 = rng.gen_range(1..max_rows);

        let pos = Position {
            x: random_x,
            y: random_y,
        };

        if position_free(&pos, entities) {
            final_position = pos;
            found = true;
        }
    }

    final_position
}

fn get_random_grass() -> TileNames {
    let mut rng = rand::thread_rng();
    let random_grass: u8 = rng.gen_range(0..=100);
    let grass_texture: &TileNames = match random_grass {
        0..=50 => &TileNames::Grass1,
        51..=75 => &TileNames::Grass2,
        76..=100 => &TileNames::Grass3,
        _ => unreachable!(),
    };

    grass_texture.to_owned()
}

fn get_random_dirt() -> TileNames {
    let mut rng = rand::thread_rng();
    let random_grass: u8 = rng.gen_range(0..=100);
    let grass_texture: &TileNames = match random_grass {
        0..=50 => &TileNames::Dirt0,
        51..=60 => &TileNames::Dirt1,
        61..=80 => &TileNames::Dirt2,
        81..=90 => &TileNames::Dirt3,
        91..=100 => &TileNames::Dirt4,
        _ => unreachable!(),
    };

    grass_texture.to_owned()
}

fn get_random_ground() -> TileNames {
    let mut rng = rand::thread_rng();
    let random_ground: u8 = rng.gen_range(1..=100);

    let grass_texture = match random_ground {
        0..=66 => get_random_dirt(),
        67..=100 => get_random_grass(),
        _ => unreachable!(),
    };

    grass_texture.to_owned()
}

fn get_random_brick_wall() -> TileNames {
    let mut rng = rand::thread_rng();
    let random_brick_wall: u8 = rng.gen_range(1..=100);
    let grass_texture: &TileNames = match random_brick_wall {
        1..=74 => &TileNames::BrickWall1,
        75..=89 => &TileNames::BrickWall2,
        90..=100 => &TileNames::BrickWall3,
        _ => unreachable!(),
    };

    grass_texture.to_owned()
}

pub fn gen_entity(
    entity_mode: EntityMode,
    entity_relationship: EntityRelationship,
    position: Position,
) -> Entity {
    let entity_characteristics = EntityCharacteristics {
        face: match entity_mode {
            EntityMode::BrickWall => get_random_brick_wall(),
            EntityMode::Player => TileNames::NakedPlayer,
            EntityMode::NPC => TileNames::NPC,
            EntityMode::Mob => TileNames::Outline,
            EntityMode::Ground => get_random_ground(),
        },
        color: match entity_relationship {
            EntityRelationship::Foe => Color::RED,
            EntityRelationship::Friendly => Color::GREEN,
            EntityRelationship::Neutral => Color::WHITE,
            EntityRelationship::None => Color::from_hex("333333").unwrap(),
        },
        walkable: match entity_mode {
            EntityMode::BrickWall => false,
            EntityMode::Player => false,
            EntityMode::NPC => false,
            EntityMode::Mob => false,
            EntityMode::Ground => true,
        },
    };

    Entity {
        id: Uuid::new_v4(),
        mode: entity_mode,
        relationship: entity_relationship,
        characteristics: entity_characteristics,
        pos: position,
    }
}

pub fn update_player(entity: &mut Entity, d: &mut RaylibDrawHandle, game: &Game) {
    if d.is_key_pressed(KeyboardKey::KEY_H) && can_move(entity, "LEFT", game) {
        entity.pos.x -= 1;
    }

    if d.is_key_pressed(KeyboardKey::KEY_L) && can_move(entity, "RIGHT", game) {
        entity.pos.x += 1;
    }

    if d.is_key_pressed(KeyboardKey::KEY_J) && can_move(entity, "DOWN", game) {
        entity.pos.y += 1;
    }

    if d.is_key_pressed(KeyboardKey::KEY_K) && can_move(entity, "UP", game) {
        entity.pos.y -= 1;
    }
}

pub fn can_move(entity: &Entity, side: &str, game: &Game) -> bool {
    if side == "LEFT" {
        // is there any entity that is directly to the left of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos.x == (entity.pos.x - 1) && game_entity.pos.y == entity.pos.y {
                return false;
            }
        }

        // is there any building that is directly to the left of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos.x == (entity.pos.x - 1) && game_building.pos.y == entity.pos.y {
                if !game_building.characteristics.walkable {
                    return false;
                }
            }
        }
    }

    if side == "RIGHT" {
        // is there any entity that is directly to the right of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos.x == (entity.pos.x + 1) && game_entity.pos.y == entity.pos.y {
                return false;
            }
        }

        // is there any building that is directly to the right of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos.x == (entity.pos.x + 1) && game_building.pos.y == entity.pos.y {
                if !game_building.characteristics.walkable {
                    return false;
                }
            }
        }
    }

    if side == "UP" {
        // is there any entity that is directly to the up of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos.x == entity.pos.x && game_entity.pos.y == (entity.pos.y - 1) {
                return false;
            }
        }

        // is there any building that is directly to the up of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos.x == entity.pos.x && game_building.pos.y == (entity.pos.y - 1) {
                if !game_building.characteristics.walkable {
                    return false;
                }
            }
        }
    }

    if side == "DOWN" {
        // is there any entity that is directly to the down of this entity?
        for game_entity in game.entities.iter() {
            if game_entity.pos.x == entity.pos.x && game_entity.pos.y == (entity.pos.y + 1) {
                return false;
            }
        }

        // is there any entity that is directly to the down of this entity?
        for game_building in game.buildings.iter() {
            if game_building.pos.x == entity.pos.x && game_building.pos.y == (entity.pos.y + 1) {
                if !game_building.characteristics.walkable {
                    return false;
                }
            }
        }
    }

    true
}

pub fn move_random(entity: &mut Entity, game: &Game) {
    let mut rng = rand::thread_rng();
    let random_move: u8 = rng.gen_range(1..5);
    let actual_move = match random_move {
        1 => "LEFT",
        2 => "UP",
        3 => "DOWN",
        4 => "RIGHT",
        _ => unreachable!(),
    };

    println!(
        "Entity with id {} is trying to move {}",
        entity.id.to_string(),
        actual_move
    );

    if random_move == 1 && can_move(&entity, actual_move, game) {
        println!(
            "Entity with id {} moved {}",
            entity.id.to_string(),
            actual_move
        );
        entity.pos.x -= 1;
    }

    if random_move == 2 && can_move(&entity, actual_move, game) {
        println!(
            "Entity with id {} moved {}",
            entity.id.to_string(),
            actual_move
        );
        entity.pos.y -= 1;
    }

    if random_move == 3 && can_move(&entity, actual_move, game) {
        println!(
            "Entity with id {} moved {}",
            entity.id.to_string(),
            actual_move
        );
        entity.pos.y += 1;
    }

    if random_move == 4 && can_move(&entity, actual_move, game) {
        println!(
            "Entity with id {} moved {}",
            entity.id.to_string(),
            actual_move
        );
        entity.pos.x += 1;
    }
}

pub fn get_settings(
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

pub fn position_free(pos: &Position, entities: &Vec<Entity>) -> bool {
    for game_entity in entities.iter() {
        if game_entity.pos.x == pos.x && game_entity.pos.y == pos.y {
            return false;
        }
    }

    true
}

pub fn get_all_keys() -> Vec<KeyboardKey> {
    let mut all_keys: Vec<KeyboardKey> = Vec::new();

    all_keys.push(KeyboardKey::KEY_A);
    all_keys.push(KeyboardKey::KEY_B);
    all_keys.push(KeyboardKey::KEY_C);
    all_keys.push(KeyboardKey::KEY_D);
    all_keys.push(KeyboardKey::KEY_E);
    all_keys.push(KeyboardKey::KEY_F);
    all_keys.push(KeyboardKey::KEY_G);
    all_keys.push(KeyboardKey::KEY_H);
    all_keys.push(KeyboardKey::KEY_I);
    all_keys.push(KeyboardKey::KEY_J);
    all_keys.push(KeyboardKey::KEY_K);
    all_keys.push(KeyboardKey::KEY_L);
    all_keys.push(KeyboardKey::KEY_M);
    all_keys.push(KeyboardKey::KEY_N);
    all_keys.push(KeyboardKey::KEY_O);
    all_keys.push(KeyboardKey::KEY_P);
    all_keys.push(KeyboardKey::KEY_Q);
    all_keys.push(KeyboardKey::KEY_R);
    all_keys.push(KeyboardKey::KEY_S);
    all_keys.push(KeyboardKey::KEY_T);
    all_keys.push(KeyboardKey::KEY_U);
    all_keys.push(KeyboardKey::KEY_V);
    all_keys.push(KeyboardKey::KEY_X);
    all_keys.push(KeyboardKey::KEY_Y);
    all_keys.push(KeyboardKey::KEY_W);
    all_keys.push(KeyboardKey::KEY_Z);
    all_keys.push(KeyboardKey::KEY_ONE);
    all_keys.push(KeyboardKey::KEY_TWO);
    all_keys.push(KeyboardKey::KEY_THREE);
    all_keys.push(KeyboardKey::KEY_FOUR);
    all_keys.push(KeyboardKey::KEY_FIVE);
    all_keys.push(KeyboardKey::KEY_SIX);
    all_keys.push(KeyboardKey::KEY_SEVEN);
    all_keys.push(KeyboardKey::KEY_EIGHT);
    all_keys.push(KeyboardKey::KEY_NINE);
    all_keys.push(KeyboardKey::KEY_ZERO);
    all_keys.push(KeyboardKey::KEY_DOWN);
    all_keys.push(KeyboardKey::KEY_LEFT);
    all_keys.push(KeyboardKey::KEY_RIGHT);
    all_keys.push(KeyboardKey::KEY_UP);
    all_keys.push(KeyboardKey::KEY_APOSTROPHE);
    all_keys.push(KeyboardKey::KEY_BACK);
    all_keys.push(KeyboardKey::KEY_BACKSLASH);
    all_keys.push(KeyboardKey::KEY_BACKSPACE);
    all_keys.push(KeyboardKey::KEY_CAPS_LOCK);
    all_keys.push(KeyboardKey::KEY_COMMA);
    all_keys.push(KeyboardKey::KEY_DELETE);
    all_keys.push(KeyboardKey::KEY_END);
    all_keys.push(KeyboardKey::KEY_ENTER);
    all_keys.push(KeyboardKey::KEY_EQUAL);
    all_keys.push(KeyboardKey::KEY_ESCAPE);
    all_keys.push(KeyboardKey::KEY_F1);
    all_keys.push(KeyboardKey::KEY_F2);
    all_keys.push(KeyboardKey::KEY_F3);
    all_keys.push(KeyboardKey::KEY_F4);
    all_keys.push(KeyboardKey::KEY_F5);
    all_keys.push(KeyboardKey::KEY_F6);
    all_keys.push(KeyboardKey::KEY_F7);
    all_keys.push(KeyboardKey::KEY_F8);
    all_keys.push(KeyboardKey::KEY_F9);
    all_keys.push(KeyboardKey::KEY_F10);
    all_keys.push(KeyboardKey::KEY_F11);
    all_keys.push(KeyboardKey::KEY_F12);
    all_keys.push(KeyboardKey::KEY_GRAVE);
    all_keys.push(KeyboardKey::KEY_HOME);
    all_keys.push(KeyboardKey::KEY_INSERT);
    all_keys.push(KeyboardKey::KEY_KB_MENU);
    all_keys.push(KeyboardKey::KEY_KP_0);
    all_keys.push(KeyboardKey::KEY_KP_1);
    all_keys.push(KeyboardKey::KEY_KP_2);
    all_keys.push(KeyboardKey::KEY_KP_3);
    all_keys.push(KeyboardKey::KEY_KP_4);
    all_keys.push(KeyboardKey::KEY_KP_5);
    all_keys.push(KeyboardKey::KEY_KP_6);
    all_keys.push(KeyboardKey::KEY_KP_7);
    all_keys.push(KeyboardKey::KEY_KP_8);
    all_keys.push(KeyboardKey::KEY_KP_9);
    all_keys.push(KeyboardKey::KEY_KP_ENTER);
    all_keys.push(KeyboardKey::KEY_KP_ADD);
    all_keys.push(KeyboardKey::KEY_KP_DECIMAL);
    all_keys.push(KeyboardKey::KEY_KP_DIVIDE);
    all_keys.push(KeyboardKey::KEY_KP_ENTER);
    all_keys.push(KeyboardKey::KEY_KP_EQUAL);
    all_keys.push(KeyboardKey::KEY_KP_MULTIPLY);
    all_keys.push(KeyboardKey::KEY_KP_SUBTRACT);
    all_keys.push(KeyboardKey::KEY_LEFT_ALT);
    all_keys.push(KeyboardKey::KEY_LEFT_BRACKET);
    all_keys.push(KeyboardKey::KEY_LEFT_CONTROL);
    all_keys.push(KeyboardKey::KEY_LEFT_SHIFT);
    all_keys.push(KeyboardKey::KEY_LEFT_SUPER);
    all_keys.push(KeyboardKey::KEY_MINUS);
    all_keys.push(KeyboardKey::KEY_NULL);
    all_keys.push(KeyboardKey::KEY_NUM_LOCK);
    all_keys.push(KeyboardKey::KEY_PAGE_DOWN);
    all_keys.push(KeyboardKey::KEY_PAGE_UP);
    all_keys.push(KeyboardKey::KEY_PAUSE);
    all_keys.push(KeyboardKey::KEY_PERIOD);
    all_keys.push(KeyboardKey::KEY_PRINT_SCREEN);
    all_keys.push(KeyboardKey::KEY_RIGHT_ALT);
    all_keys.push(KeyboardKey::KEY_RIGHT_BRACKET);
    all_keys.push(KeyboardKey::KEY_RIGHT_CONTROL);
    all_keys.push(KeyboardKey::KEY_RIGHT_SHIFT);
    all_keys.push(KeyboardKey::KEY_RIGHT_SUPER);
    all_keys.push(KeyboardKey::KEY_SCROLL_LOCK);
    all_keys.push(KeyboardKey::KEY_SEMICOLON);
    all_keys.push(KeyboardKey::KEY_SLASH);
    all_keys.push(KeyboardKey::KEY_SPACE);
    all_keys.push(KeyboardKey::KEY_TAB);
    all_keys.push(KeyboardKey::KEY_VOLUME_DOWN);
    all_keys.push(KeyboardKey::KEY_VOLUME_UP);
    all_keys.push(KeyboardKey::KEY_MENU);

    all_keys
}

pub fn user_interacted(d: &RaylibDrawHandle) -> Option<KeyboardKey> {
    let all_keys: Vec<KeyboardKey> = get_all_keys();

    for key in all_keys {
        if d.is_key_pressed(key) {
            return Option::Some(key);
        }
    }

    return Option::None;
}
