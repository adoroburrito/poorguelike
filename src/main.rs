use rand::prelude::*;
use raylib::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
enum EntityRelationship {
    Foe,
    Friendly,
    Neutral,
    None,
}

#[derive(Debug, Clone, Copy)]
enum EntityMode {
    Building,
    Player,
    NPC,
}

#[derive(Debug, Clone)]
struct EntityCharacteristics {
    face: char,
    color: Color,
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Entity {
    id: Uuid,
    mode: EntityMode,
    relationship: EntityRelationship,
    characteristics: EntityCharacteristics,
    pos: Position,
}

#[derive(Debug, Clone, Copy)]
struct GraphicSettings {
    tile_height: i32,
    tile_width: i32,
    font_offset_x: i32,
    font_offset_y: i32,
    font_size: i32,
    rows: i32,
    columns: i32,
}

#[derive(Debug, Clone, Copy)]
struct WindowSettings {
    height: i32,
    width: i32,
}

#[derive(Debug, Clone, Copy)]
struct Settings {
    window: WindowSettings,
    graphic: GraphicSettings,
}

#[derive(Debug, Clone)]
struct Game {
    entities: Vec<Entity>,
    buildings: Vec<Entity>,
    settings: Settings,
}

fn update_player(entity: &mut Entity, d: &mut RaylibDrawHandle, game: &Game) {
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

fn can_move(entity: &Entity, side: &str, game: &Game) -> bool {
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
                return false;
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
                return false;
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
                return false;
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
                return false;
            }
        }
    }

    true
}

fn move_random(entity: &mut Entity, game: &Game) {
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

fn update_entities(d: &mut RaylibDrawHandle, game: &Game, settings: &Settings) -> Game {
    let mut new_entities = game.entities.to_vec();

    for entity in new_entities.iter_mut() {
        match entity.mode {
            EntityMode::Player => update_player(entity, d, game),
            EntityMode::NPC => move_random(entity, game),
            _ => (),
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
            &building.characteristics.face.to_string(),
            (building.pos.x * settings.graphic.tile_width) + settings.graphic.font_offset_x,
            (building.pos.y * settings.graphic.tile_height) + settings.graphic.font_offset_y,
            settings.graphic.font_size,
            building.characteristics.color,
        )
    }

    for entity in game.entities.iter() {
        d.draw_text(
            &entity.characteristics.face.to_string(),
            (entity.pos.x * settings.graphic.tile_width) + settings.graphic.font_offset_x,
            (entity.pos.y * settings.graphic.tile_height) + settings.graphic.font_offset_y,
            settings.graphic.font_size,
            entity.characteristics.color,
        );

        let id_to_draw: String = entity.id.to_string().chars().skip(30).take(6).collect();
        d.draw_text(
            &id_to_draw,
            (entity.pos.x * settings.graphic.tile_width),
            (entity.pos.y * settings.graphic.tile_height) + settings.graphic.font_offset_y + 30,
            8,
            entity.characteristics.color,
        );
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

fn position_free(pos: &Position, entities: &Vec<Entity>) -> bool {
    for game_entity in entities.iter() {
        if game_entity.pos.x == pos.x && game_entity.pos.y == pos.y {
            return false;
        }
    }

    true
}

fn gen_random_position(entities: &Vec<Entity>, max_cols: i32, max_rows: i32) -> Position {
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

fn gen_entity(
    entity_mode: EntityMode,
    entity_relationship: EntityRelationship,
    position: Position,
) -> Entity {
    let entity_characteristics = EntityCharacteristics {
        face: match entity_mode {
            EntityMode::Building => '#',
            EntityMode::Player => '@',
            EntityMode::NPC => '&',
        },
        color: match entity_relationship {
            EntityRelationship::Foe => Color::RED,
            EntityRelationship::Friendly => Color::GREEN,
            EntityRelationship::Neutral => Color::WHITE,
            EntityRelationship::None => Color::from_hex("333333").unwrap(),
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

fn get_starting_entities(settings: &Settings) -> Vec<Entity> {
    let mut to_return: Vec<Entity> = Vec::new();

    let position: Position = Position { x: 1, y: 1 };

    let player: Entity = gen_entity(EntityMode::Player, EntityRelationship::Neutral, position);

    to_return.push(player);

    for _ in 0..5 {
        let npc: Entity = gen_entity(
            EntityMode::NPC,
            EntityRelationship::Friendly,
            gen_random_position(
                &to_return,
                settings.graphic.columns - 1,
                settings.graphic.rows - 1,
            ),
        );

        to_return.push(npc);
    }

    for _ in 0..5 {
        let npc: Entity = gen_entity(
            EntityMode::NPC,
            EntityRelationship::Foe,
            gen_random_position(
                &to_return,
                settings.graphic.columns - 1,
                settings.graphic.rows - 1,
            ),
        );

        to_return.push(npc);
    }

    to_return
}

fn get_starting_buildings(settings: &Settings) -> Vec<Entity> {
    let mut buildings: Vec<Entity> = Vec::new();

    for n in 0..settings.graphic.columns {
        buildings.push(gen_entity(
            EntityMode::Building,
            EntityRelationship::None,
            Position { x: n, y: 0 },
        ));

        buildings.push(gen_entity(
            EntityMode::Building,
            EntityRelationship::None,
            Position {
                x: n,
                y: settings.graphic.columns - 1,
            },
        ));
    }

    for n in 0..settings.graphic.rows {
        buildings.push(gen_entity(
            EntityMode::Building,
            EntityRelationship::None,
            Position { x: 0, y: n },
        ));

        buildings.push(gen_entity(
            EntityMode::Building,
            EntityRelationship::None,
            Position {
                x: settings.graphic.rows - 1,
                y: n,
            },
        ));
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

fn debug(game: &Game) {
    println!("Entities:");

    for entity in game.entities.iter() {
        print!("\t- Entity {}\n", entity.id);
        print!("\t\t- Mode {:#?}\n", entity.mode);
        print!("\t\t- Relationship {:#?}\n", entity.relationship);
        print!("\t\t- Characteristics\n");
        print!("\t\t\t- Face {}\n", entity.characteristics.face);
        print!(
            "\t\t\t- Color: r={} g={} b={}\n",
            entity.characteristics.color.r,
            entity.characteristics.color.g,
            entity.characteristics.color.b
        );
        print!("\t\t- Position: x={} y={}\n", entity.pos.x, entity.pos.y);
        println!();
    }
}

fn main() {
    let settings: Settings = get_settings(768, 768, 20, 20, 26);
    let entities: Vec<Entity> = get_starting_entities(&settings);
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

            debug(&game)
        } else {
            draw_game(&mut d, &game, &settings);
        }
    }
}
