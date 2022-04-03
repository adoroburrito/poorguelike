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
    BrickWall,
    Ground,
    Player,
    NPC,
    Mob,
}

#[derive(Debug, Clone)]
pub struct EntityCharacteristics {
    pub face: TileNames,
    pub color: Color,
    pub walkable: bool,
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
