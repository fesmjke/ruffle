use crate::life::Organism;

enum Cell {
    Empty,
    Full(Organism)
}

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>
}

impl World {
    pub fn new(w: usize, h: usize, alive: Vec<Organism>) -> Self {
        let mut initial_cells = vec![];
        
        for x in 0..w {
            let mut temp : Vec<Cell> = vec![];
            
            for y in 0..h {
                temp.push(Cell::Empty);
            }

            initial_cells.push(temp);
        }

        for organism in alive {
            match organism {
                Organism::Dead => {},
                Organism::Alive( life ) => initial_cells[life.x][life.y] = Cell::Full(organism),
            }
        }
        
        World { width: w, height: h, cells: initial_cells }
    }

    fn next(&self) {
        todo!();
    }

    pub fn draw(&self) {

        for x in 0..self.width {
            for y in 0..self.height {

                match self.cells[x][y] {
                    Cell::Empty => print!(" ▢ "),
                    Cell::Full(_) => print!(" ◉ "),
                }

            }
            println!("");
        }

    }
}