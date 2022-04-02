use crate::draw as DrawGame;
use crate::state::core as StateCore;
use crate::state::domain::{Entity, Game, Settings};
use crate::state::helpers as StateHelpers;
use raylib::color::Color;
use raylib::prelude::RaylibDraw;
use std::collections::HashMap;

mod state {

    pub mod domain {
        use crate::draw::TileNames;
        use raylib::color::Color;
        use uuid::Uuid;

        #[derive(Debug, Clone, Copy)]
        pub enum EntityRelationship {
            Foe,
            Friendly,
            Neutral,
            None,
        }

        #[derive(Debug, Clone, Copy)]
        pub enum EntityMode {
            Building,
            Player,
            NPC,
            Mob,
        }

        #[derive(Debug, Clone)]
        pub struct EntityCharacteristics {
            pub face: TileNames,
            pub color: Color,
        }

        pub struct TilePosition {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone)]
        pub struct Position {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone)]
        pub struct Entity {
            pub id: Uuid,
            pub mode: EntityMode,
            pub relationship: EntityRelationship,
            pub characteristics: EntityCharacteristics,
            pub pos: Position,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct GraphicSettings {
            pub tile_height: i32,
            pub tile_width: i32,
            pub font_offset_x: i32,
            pub font_offset_y: i32,
            pub font_size: i32,
            pub rows: i32,
            pub columns: i32,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct WindowSettings {
            pub height: i32,
            pub width: i32,
        }

        #[derive(Debug, Clone, Copy)]
        pub struct Settings {
            pub window: WindowSettings,
            pub graphic: GraphicSettings,
        }

        #[derive(Debug, Clone)]
        pub struct Game {
            pub entities: Vec<Entity>,
            pub buildings: Vec<Entity>,
            pub settings: Settings,
        }
    }

    pub mod helpers {
        use crate::state::domain::{
            Entity, EntityCharacteristics, EntityMode, EntityRelationship, Game, GraphicSettings,
            Position, Settings, WindowSettings,
        };

        use crate::draw::TileNames;
        use rand::Rng;
        use raylib::color::Color;
        use raylib::drawing::RaylibDrawHandle;
        use raylib::prelude::KeyboardKey;
        use uuid::Uuid;

        pub fn gen_random_position(
            entities: &Vec<Entity>,
            max_cols: i32,
            max_rows: i32,
        ) -> Position {
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

        pub fn gen_entity(
            entity_mode: EntityMode,
            entity_relationship: EntityRelationship,
            position: Position,
        ) -> Entity {
            let entity_characteristics = EntityCharacteristics {
                face: match entity_mode {
                    EntityMode::Building => TileNames::BrickWall,
                    EntityMode::Player => TileNames::NakedPlayer,
                    EntityMode::NPC => TileNames::NPC,
                    EntityMode::Mob => TileNames::Outline,
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
                    if game_entity.pos.x == (entity.pos.x - 1) && game_entity.pos.y == entity.pos.y
                    {
                        return false;
                    }
                }

                // is there any building that is directly to the left of this entity?
                for game_building in game.buildings.iter() {
                    if game_building.pos.x == (entity.pos.x - 1)
                        && game_building.pos.y == entity.pos.y
                    {
                        return false;
                    }
                }
            }

            if side == "RIGHT" {
                // is there any entity that is directly to the right of this entity?
                for game_entity in game.entities.iter() {
                    if game_entity.pos.x == (entity.pos.x + 1) && game_entity.pos.y == entity.pos.y
                    {
                        return false;
                    }
                }

                // is there any building that is directly to the right of this entity?
                for game_building in game.buildings.iter() {
                    if game_building.pos.x == (entity.pos.x + 1)
                        && game_building.pos.y == entity.pos.y
                    {
                        return false;
                    }
                }
            }

            if side == "UP" {
                // is there any entity that is directly to the up of this entity?
                for game_entity in game.entities.iter() {
                    if game_entity.pos.x == entity.pos.x && game_entity.pos.y == (entity.pos.y - 1)
                    {
                        return false;
                    }
                }

                // is there any building that is directly to the up of this entity?
                for game_building in game.buildings.iter() {
                    if game_building.pos.x == entity.pos.x
                        && game_building.pos.y == (entity.pos.y - 1)
                    {
                        return false;
                    }
                }
            }

            if side == "DOWN" {
                // is there any entity that is directly to the down of this entity?
                for game_entity in game.entities.iter() {
                    if game_entity.pos.x == entity.pos.x && game_entity.pos.y == (entity.pos.y + 1)
                    {
                        return false;
                    }
                }

                // is there any entity that is directly to the down of this entity?
                for game_building in game.buildings.iter() {
                    if game_building.pos.x == entity.pos.x
                        && game_building.pos.y == (entity.pos.y + 1)
                    {
                        return false;
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

        pub fn user_interacted(d: &RaylibDrawHandle) -> bool {
            if d.is_key_pressed(KeyboardKey::KEY_H)
                || d.is_key_pressed(KeyboardKey::KEY_J)
                || d.is_key_pressed(KeyboardKey::KEY_K)
                || d.is_key_pressed(KeyboardKey::KEY_L)
            {
                return true;
            }

            false
        }
    }

    pub mod core {
        use raylib::drawing::RaylibDrawHandle;

        use crate::state::domain::{
            Entity, EntityMode, EntityRelationship, Game, Position, Settings,
        };

        use crate::state::helpers as StateHelpers;

        pub fn get_starting_entities(settings: &Settings) -> Vec<Entity> {
            let mut to_return: Vec<Entity> = Vec::new();

            let position: Position = Position { x: 1, y: 1 };

            let player: Entity =
                StateHelpers::gen_entity(EntityMode::Player, EntityRelationship::Neutral, position);

            to_return.push(player);

            for _ in 0..5 {
                let npc: Entity = StateHelpers::gen_entity(
                    EntityMode::NPC,
                    EntityRelationship::Friendly,
                    StateHelpers::gen_random_position(
                        &to_return,
                        settings.graphic.columns - 1,
                        settings.graphic.rows - 1,
                    ),
                );

                to_return.push(npc);
            }

            for _ in 0..5 {
                let npc: Entity = StateHelpers::gen_entity(
                    EntityMode::Mob,
                    EntityRelationship::Foe,
                    StateHelpers::gen_random_position(
                        &to_return,
                        settings.graphic.columns - 1,
                        settings.graphic.rows - 1,
                    ),
                );

                to_return.push(npc);
            }

            to_return
        }

        pub fn get_starting_buildings(settings: &Settings) -> Vec<Entity> {
            let mut buildings: Vec<Entity> = Vec::new();

            for n in 0..settings.graphic.columns {
                buildings.push(StateHelpers::gen_entity(
                    EntityMode::Building,
                    EntityRelationship::None,
                    Position { x: n, y: 0 },
                ));

                buildings.push(StateHelpers::gen_entity(
                    EntityMode::Building,
                    EntityRelationship::None,
                    Position {
                        x: n,
                        y: settings.graphic.columns - 1,
                    },
                ));
            }

            for n in 0..settings.graphic.rows {
                buildings.push(StateHelpers::gen_entity(
                    EntityMode::Building,
                    EntityRelationship::None,
                    Position { x: 0, y: n },
                ));

                buildings.push(StateHelpers::gen_entity(
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

        pub fn update_game_states(
            d: &mut RaylibDrawHandle,
            game: &Game,
            settings: &Settings,
        ) -> Game {
            let mut new_entities = game.entities.to_vec();

            for entity in new_entities.iter_mut() {
                match entity.mode {
                    EntityMode::Player => StateHelpers::update_player(entity, d, game),
                    EntityMode::NPC => StateHelpers::move_random(entity, game),
                    EntityMode::Mob => StateHelpers::move_random(entity, game),
                    _ => (),
                }
            }

            return Game {
                entities: new_entities,
                settings: settings.to_owned(),
                buildings: game.buildings.to_owned(),
            };
        }
    }
}

mod draw {
    use crate::state::domain::{EntityMode, Game, Settings, TilePosition};
    use crate::HashMap;
    use raylib::color::Color;
    use raylib::drawing::RaylibDrawHandle;
    use raylib::math::{Rectangle, Vector2};
    use raylib::misc::AsF32;
    use raylib::prelude::{RaylibDraw, Texture2D};

    #[derive(PartialEq, Eq, Hash, Debug, Clone)]
    pub enum TileNames {
        Dirt1,
        Dirt2,
        Dirt3,
        Dirt4,
        Grass1,
        Grass2,
        Grass3,
        BrickWall,
        NakedPlayer,
        NPC,
        Outline,
    }

    pub fn grid(d: &mut RaylibDrawHandle, settings: &Settings) {
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

    pub fn entities(
        d: &mut RaylibDrawHandle,
        game: &Game,
        settings: &Settings,
        tileset: &Texture2D,
    ) {
        let mut tile_map: HashMap<TileNames, TilePosition> = HashMap::new();

        tile_map.insert(TileNames::Dirt1, TilePosition { x: 1.0, y: 0.0 });
        tile_map.insert(TileNames::Dirt2, TilePosition { x: 2.0, y: 0.0 });
        tile_map.insert(TileNames::Dirt3, TilePosition { x: 3.0, y: 0.0 });
        tile_map.insert(TileNames::Dirt4, TilePosition { x: 4.0, y: 0.0 });
        tile_map.insert(TileNames::Grass1, TilePosition { x: 6.0, y: 0.0 });
        tile_map.insert(TileNames::Grass2, TilePosition { x: 7.0, y: 0.0 });
        tile_map.insert(TileNames::Grass3, TilePosition { x: 8.0, y: 0.0 });
        tile_map.insert(TileNames::BrickWall, TilePosition { x: 7.0, y: 15.0 });

        tile_map.insert(TileNames::NakedPlayer, TilePosition { x: 25.0, y: 0.0 });
        tile_map.insert(TileNames::NPC, TilePosition { x: 25.0, y: 9.0 });
        tile_map.insert(TileNames::Outline, TilePosition { x: 24.0, y: 7.0 });

        let tile_side_size = 16.0;
        let scale = 2.5;

        for building in game.buildings.iter() {
            // d.draw_text(
            //     &building.characteristics.face.to_string(),
            //     (building.pos.x * settings.graphic.tile_width) + settings.graphic.font_offset_x,
            //     (building.pos.y * settings.graphic.tile_height) + settings.graphic.font_offset_y,
            //     settings.graphic.font_size,
            //     building.characteristics.color,
            // )
            let source_rec = Rectangle::new(
                tile_side_size * tile_map.get(&building.characteristics.face).unwrap().x,
                tile_side_size * tile_map.get(&building.characteristics.face).unwrap().y,
                16.0,
                16.0,
            );
            let dest_rec = Rectangle::new(
                building.pos.x.as_f32() * (tile_side_size * scale),
                building.pos.y.as_f32() * (tile_side_size * scale),
                tile_side_size * scale,
                tile_side_size * scale,
            );
            let origin = Vector2::new(0.0, 0.0);
            d.draw_texture_tiled(
                tileset,
                source_rec,
                dest_rec,
                origin,
                0.0,
                scale,
                Color::WHITE,
            );
        }

        for entity in game.entities.iter() {
            // d.draw_text(
            //     &entity.characteristics.face.to_string(),
            //     (entity.pos.x * settings.graphic.tile_width) + settings.graphic.font_offset_x,
            //     (entity.pos.y * settings.graphic.tile_height) + settings.graphic.font_offset_y,
            //     settings.graphic.font_size,
            //     entity.characteristics.color,
            // );
            let source_rec = Rectangle::new(
                tile_side_size * tile_map.get(&entity.characteristics.face).unwrap().x,
                tile_side_size * tile_map.get(&entity.characteristics.face).unwrap().y,
                16.0,
                16.0,
            );
            let dest_rec = Rectangle::new(
                entity.pos.x.as_f32() * (tile_side_size * scale),
                entity.pos.y.as_f32() * (tile_side_size * scale),
                tile_side_size * scale,
                tile_side_size * scale,
            );
            let origin = Vector2::new(0.0, 0.0);
            d.draw_texture_tiled(
                tileset,
                source_rec,
                dest_rec,
                origin,
                0.0,
                scale,
                match entity.mode {
                    EntityMode::Mob => Color::RED,
                    EntityMode::NPC => Color::SKYBLUE,
                    _ => Color::WHITE,
                },
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

    pub fn frame(d: &mut RaylibDrawHandle, game: &Game, settings: &Settings, tileset: &Texture2D) {
        grid(d, &settings);
        entities(d, game, &settings, tileset);
    }
}

fn debug(game: &Game) {
    println!("Entities:");

    for entity in game.entities.iter() {
        print!("\t- Entity {}\n", entity.id);
        print!("\t\t- Mode {:#?}\n", entity.mode);
        print!("\t\t- Relationship {:#?}\n", entity.relationship);
        print!("\t\t- Characteristics\n");
        print!("\t\t\t- Face {:#?}\n", entity.characteristics.face);
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
    let settings: Settings = StateHelpers::get_settings(800, 800, 20, 20, 26);
    let entities: Vec<Entity> = StateCore::get_starting_entities(&settings);
    let buildings: Vec<Entity> = StateCore::get_starting_buildings(&settings);

    let mut game = Game {
        entities: entities,
        settings: settings,
        buildings: buildings,
    };

    let (mut rl, thread) = raylib::init()
        .size(settings.window.width, settings.window.height)
        .title("Poorguelike")
        .build();

    let tileset = rl.load_texture(&thread, "./tileset.png").unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if StateHelpers::user_interacted(&mut d) {
            let updated_game: Game = StateCore::update_game_states(&mut d, &game, &settings);

            DrawGame::frame(&mut d, &updated_game, &settings, &tileset);

            game = updated_game;

            debug(&game)
        } else {
            DrawGame::frame(&mut d, &game, &settings, &tileset);
        }
    }
}
