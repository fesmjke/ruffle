extern crate termion;

use std::io::{Write, stdout, stdin};
use std::{thread, time};

use crate::life::Organism;
use crate::world::World;

mod life;
mod position;
mod world;


fn main() {    
    let mut width = 10 as usize;
    let mut height = 10 as usize;
    
    let mut stdin = stdin();
    let mut stdout = stdout();

    let mut world = World::new(
        width,
        height,
        stdout,
        stdin,
        vec![
            Organism::Alive(crate::position::Position { x: 1, y: 1 }),
            Organism::Alive(crate::position::Position { x: 1, y: 0 }),
            Organism::Alive(crate::position::Position { x: 1, y: 2 }),
        ],
    );

    world.resize();

    loop {
        world.draw();
        world.next();

        thread::sleep(time::Duration::from_millis(1000));

        if !world.dead() {
            world.draw();
            break;
        }
    }

    println!("Your world is dead!")
}
