#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Face {}

#[derive(Debug)]
pub struct Entity {
    pos_x: usize,
    pos_y: usize,
    face: Face,
}

#[derive(Debug)]
pub struct Actor {
    pos_x: usize,
    pos_y: usize,
    face: Face,
}
