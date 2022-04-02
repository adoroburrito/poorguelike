use crate::state::domain::{EntityMode, Game, Position, Settings, TilePosition};
use crate::state::helpers as StateHelpers;
use crate::HashMap;
use rand::Rng;
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

pub fn get_tilenames() -> HashMap<TileNames, TilePosition> {
    let mut tile_map: HashMap<TileNames, TilePosition> = HashMap::new();

    tile_map.insert(TileNames::Dirt1, TilePosition { x: 1.0, y: 0.0 });
    tile_map.insert(TileNames::Dirt2, TilePosition { x: 2.0, y: 0.0 });
    tile_map.insert(TileNames::Dirt3, TilePosition { x: 3.0, y: 0.0 });
    tile_map.insert(TileNames::Dirt4, TilePosition { x: 4.0, y: 0.0 });
    tile_map.insert(TileNames::Grass1, TilePosition { x: 5.0, y: 0.0 });
    tile_map.insert(TileNames::Grass2, TilePosition { x: 6.0, y: 0.0 });
    tile_map.insert(TileNames::Grass3, TilePosition { x: 7.0, y: 0.0 });
    tile_map.insert(TileNames::BrickWall, TilePosition { x: 7.0, y: 15.0 });

    tile_map.insert(TileNames::NakedPlayer, TilePosition { x: 25.0, y: 0.0 });
    tile_map.insert(TileNames::NPC, TilePosition { x: 25.0, y: 9.0 });
    tile_map.insert(TileNames::Outline, TilePosition { x: 24.0, y: 7.0 });

    tile_map
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
    tileset_terrain: &Texture2D,
    tileset_entities: &Texture2D,
) {
    let mut tile_map: HashMap<TileNames, TilePosition> = get_tilenames();

    let tile_side_size = 16.0;
    let scale = 2.5;

    for building in game.buildings.iter() {
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
            tileset_terrain,
            source_rec,
            dest_rec,
            origin,
            0.0,
            scale,
            match building.mode {
                EntityMode::Wall => Color::WHITE,
                EntityMode::Ground => Color::from_hex("AAAAAA").unwrap(),
                _ => unreachable!(),
            },
        );
    }

    for entity in game.entities.iter() {
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
            tileset_entities,
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
    }
}

pub fn mouse(
    d: &mut RaylibDrawHandle,
    game: &Game,
    settings: &Settings,
    tileset_terrain: &Texture2D,
    tileset_entities: &Texture2D,
) {
    let mouse_x = d.get_mouse_x();
    let mouse_y = d.get_mouse_y();

    let text = format!("x: {}, y: {}", mouse_x, mouse_y);
    d.draw_text(&text, mouse_x, mouse_y, 20, Color::WHITE);
}

pub fn frame(
    d: &mut RaylibDrawHandle,
    game: &Game,
    settings: &Settings,
    tileset_terrain: &Texture2D,
    tileset_entities: &Texture2D,
) {
    grid(d, &settings);
    entities(d, game, &settings, tileset_terrain, tileset_entities);
    mouse(d, game, &settings, tileset_terrain, tileset_entities);
}
