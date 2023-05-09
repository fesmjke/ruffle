use position::Position;
use crate::world::World;
use crate::life::Organism;

mod life;
mod world;
mod position;

fn main() {
    let mut world = World::new(5,5, vec![Organism::Alive(Position::new(2,2))]);

    world.draw();
}
