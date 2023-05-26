extern crate termion;

use std::io::{Write, stdout, stdin, Stdout};
use std::{thread, time};

use termion::async_stdin;

use crate::life::Organism;
use crate::world::World;

mod life;
mod position;
mod world;
mod menu;

fn main() {    
    let mut width = 10 as usize;
    let mut height = 10 as usize;
    
    let mut stdin = async_stdin();
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

    world.process();
}
