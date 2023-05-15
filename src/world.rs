use std::println;

use crate::{life::Organism, position::Position};

pub struct World {
    width: usize,
    height: usize,
    buffer: Vec<Vec<Organism>>,
    cells: Vec<Vec<Organism>>,
}

impl World {
    pub fn new(w: usize, h: usize, alive: Vec<Organism>) -> Self {
        let mut initial_cells = vec![];

        for x in 0..w {
            let mut temp: Vec<Organism> = vec![];

            for y in 0..h {
                temp.push(Organism::Dead);
            }

            initial_cells.push(temp);
        }

        for organism in alive {
            match organism {
                Organism::Dead => {}
                Organism::Alive(life) => initial_cells[life.x][life.y] = organism,
            }
        }

        World {
            width: w,
            height: h,
            buffer: vec![],
            cells: initial_cells,
        }
    }

    fn collect_alive(&self) -> usize {
        let collected: Vec<Organism> = self
            .cells
            .iter()
            .flat_map(|row| row.iter().cloned().filter(|cell| cell.is_alive()))
            .collect();

        collected.len()
    }

    fn is_valid(&self, x: usize, y: usize) -> bool {
        y < self.cells.len() && x < self.cells[0].len()
    }

    fn calculate_neighbours(&self, x: usize, y: usize) -> usize {
        let position_offset: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, 1),
            (1, -1),
            (1, 0),
        ];
        let mut neighbours = 0;

        for offset in position_offset.iter() {
            let delta_x = x as isize + offset.0;
            let delta_y = y as isize + offset.1;

            if delta_x < 0 {
                continue;
            } else if delta_y < 0 {
                continue;
            } else {
                if self.is_valid(delta_x as usize, delta_y as usize) {
                    match self.cells[delta_x as usize][delta_y as usize] {
                        Organism::Dead => {}
                        Organism::Alive(_) => neighbours += 1,
                    }
                }
            }
        }

        neighbours as usize
    }

    pub fn dead(&self) -> bool {
        self.collect_alive() > 0
    }

    pub fn next(&mut self) {
        self.buffer = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {
                let neighbours = self.calculate_neighbours(x, y);

                match self.cells[x][y] {
                    Organism::Dead => {
                        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                        if neighbours == 3 {
                            self.buffer[x][y] = Organism::Alive(Position::new(x, y));
                            continue;
                        }
                    }
                    Organism::Alive(_) => {
                        // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.

                        if neighbours < 2 {
                            self.buffer[x][y] = Organism::Dead;
                            continue;
                        }

                        // Any live cell with two or three live neighbours lives on to the next generation.

                        if neighbours == 2 || neighbours == 3 {
                            continue;
                        }

                        // Any live cell with more than three live neighbours dies, as if by overpopulation.

                        if neighbours > 3 {
                            self.buffer[x][y] = Organism::Dead;
                            continue;
                        }
                    }
                }
            }
        }

        std::mem::swap(&mut self.cells, &mut self.buffer);
    }

    pub fn draw(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                match self.cells[x][y] {
                    Organism::Dead => print!(" ▢ "),
                    Organism::Alive(_) => print!(" ◉ "),
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {

    mod neighbours {
        use crate::{life::Organism, world::World};

        #[test]
        fn collect_alive_neighbours() {
            let world = World::new(
                3,
                3,
                vec![
                    Organism::Alive(crate::position::Position { x: 1, y: 1 }),
                    Organism::Alive(crate::position::Position { x: 0, y: 0 }),
                    Organism::Alive(crate::position::Position { x: 0, y: 2 }),
                ],
            );

            let collected = world.collect_alive();

            assert_eq!(collected, 3);
        }

        #[test]
        fn calculate_neighbours() {
            let world = World::new(
                3,
                3,
                vec![
                    Organism::Alive(crate::position::Position { x: 1, y: 1 }),
                    Organism::Alive(crate::position::Position { x: 0, y: 0 }),
                    Organism::Alive(crate::position::Position { x: 0, y: 2 }),
                ],
            );

            let neighbours = world.calculate_neighbours(1, 1);

            assert_eq!(neighbours, 2);
        }
    }
}
