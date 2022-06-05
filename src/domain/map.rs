use crate::domain::entities::{Actor, Entity, Face};

pub const ALL_TERRAINS: [TerrainVariant; 6] = [
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

#[derive(Debug)]
pub struct UVec2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct RoomTile<'a> {
    pub stairs: Option<&'a StairsDirection>,
    pub terrain: &'a TerrainVariant,
    pub entities: Vec<&'a Entity>,
    pub actors: Vec<&'a Actor>,
}

#[derive(Debug)]
pub struct Room<'a> {
    pub tiles: Vec<RoomTile<'a>>,
    pub size: UVec2D,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum TerrainVariant {
    Dirt1(Terrain),
    Dirt2(Terrain),
    Grass1(Terrain),
    Grass2(Terrain),
    BrickWall1(Terrain),
    BrickWall2(Terrain),
}

#[derive(Debug)]
pub enum StairsDirection {
    Up,
    Down,
}

#[derive(Eq, PartialEq, Hash)]
pub enum TerrainName {
    Dirt1,
    Dirt2,
    Grass1,
    Grass2,
    BrickWall1,
    BrickWall2,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Terrain {
    pub traversable: bool,
    pub face: Face,
}
