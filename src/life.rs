use crate::position::Position;

#[derive(Clone, Copy)]
pub enum Organism {
    Dead,
    Alive(Position),
}

impl Organism {
    pub fn is_alive(&self) -> bool {
        match self {
            Organism::Dead => false,
            Organism::Alive(_) => true,
        }
    }
}
