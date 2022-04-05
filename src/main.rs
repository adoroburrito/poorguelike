use crate::draw as DrawGame;
use crate::state::core as StateCore;
use crate::state::domain::{Entity, Game, Settings};
use crate::state::helpers as StateHelpers;
use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::prelude::RaylibDraw;
use std::collections::HashMap;

mod state;

mod draw;

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

    let tileset_entities = rl.load_texture(&thread, "./tileset_entities.png").unwrap();
    let tileset_terrain = rl.load_texture(&thread, "./tileset_terrain.png").unwrap();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let interaction = StateHelpers::user_interacted(&mut d);
        let interacted = match interaction {
            Some(key) => true,
            None => false,
        };

        let key_pressed = match interaction {
            Some(key) => key,
            None => KeyboardKey::KEY_NULL,
        };

        if interacted {
            let updated_game: Game =
                StateCore::update_game_states(&mut d, &game, &settings, key_pressed);

            DrawGame::frame(
                &mut d,
                &updated_game,
                &settings,
                &tileset_terrain,
                &tileset_entities,
            );

            game = updated_game;

            debug(&game)
        } else {
            DrawGame::frame(
                &mut d,
                &game,
                &settings,
                &tileset_terrain,
                &tileset_entities,
            );
        }
    }
}
