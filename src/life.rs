use crate::position::Position;

#[derive(Clone, Copy)]
pub enum Organism {
    Dead,
    Alive(Position)
}