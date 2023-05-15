use std::{println, thread, time};

use crate::life::Organism;
use crate::world::World;

mod life;
mod position;
mod world;

fn main() {
    let mut world = World::new(
        5,
        5,
        vec![
            Organism::Alive(crate::position::Position { x: 1, y: 1 }),
            Organism::Alive(crate::position::Position { x: 1, y: 0 }),
            Organism::Alive(crate::position::Position { x: 1, y: 2 }),
        ],
    );

    loop {
        world.draw();
        world.next();

        thread::sleep(time::Duration::from_millis(1000));

        print!("\x1B[2J\x1B[1;1H");

        if !world.dead() {
            world.draw();
            break;
        }
    }

    println!("Your world is dead!")
}
