use raylib::drawing::RaylibDrawHandle;

use crate::state::domain::{Entity, EntityMode, EntityRelationship, Game, Position, Settings};

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
            EntityMode::Wall,
            EntityRelationship::None,
            Position { x: n, y: 0 },
        ));

        buildings.push(StateHelpers::gen_entity(
            EntityMode::Wall,
            EntityRelationship::None,
            Position {
                x: n,
                y: settings.graphic.columns - 1,
            },
        ));
    }

    for n in 0..settings.graphic.rows {
        buildings.push(StateHelpers::gen_entity(
            EntityMode::Wall,
            EntityRelationship::None,
            Position { x: 0, y: n },
        ));

        buildings.push(StateHelpers::gen_entity(
            EntityMode::Wall,
            EntityRelationship::None,
            Position {
                x: settings.graphic.rows - 1,
                y: n,
            },
        ));
    }

    for y in 1..settings.graphic.rows - 1 {
        for x in 1..settings.graphic.columns - 1 {
            buildings.push(StateHelpers::gen_entity(
                EntityMode::Ground,
                EntityRelationship::None,
                Position { x, y },
            ));
        }
    }

    return buildings;
}

pub fn update_game_states(d: &mut RaylibDrawHandle, game: &Game, settings: &Settings) -> Game {
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
