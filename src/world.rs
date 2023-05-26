use termion::{raw::IntoRawMode, input::TermRead, event::Key, cursor::DetectCursorPos, async_stdin, AsyncReader};
use crate::{life::Organism, position::{Position, self}};
use crate::menu::Menu;
use std::{io::{Write,Read,Stdin,Stdout,stdout, stdin}, thread};

enum State {
    Draw,
    Resize,
    Modify,
    Exit
}

pub struct World {
    width: usize,
    height: usize,
    stdout: Stdout,
    stdin: AsyncReader,
    state : State,
    buffer: Vec<Vec<Organism>>,
    cells: Vec<Vec<Organism>>,
    message: String
}

enum Vary {
    Increase,
    Decrease
}

const WORLD_MAX_LIMIT : usize = 20;
const WORLD_MIN_LIMIT : usize = 5;

impl World {
    pub fn new(w: usize, h: usize, stdout: Stdout,
        stdin: AsyncReader , alive: Vec<Organism>) -> Self {
        let mut initial_cells = vec![];

        for _ in 0..w {
            let mut temp: Vec<Organism> = vec![];

            for _ in 0..h {
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
            stdout,
            stdin,
            state: State::Draw,
            buffer: vec![],
            cells: initial_cells,
            message : String::new()
        }
    }

    fn collect_alive(&self) -> Vec<Organism> {
        let collected: Vec<Organism> = self
            .cells
            .iter()
            .flat_map(|row| row.iter().cloned().filter(|cell| cell.is_alive()))
            .collect();

        collected
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

    fn change_world(&mut self, direction : Vary){
        match direction {
            Vary::Increase => {
                if self.width == WORLD_MAX_LIMIT {
                    self.message = "Unable to increase world size, because its reaches a world max limit!".to_string();
                } else {
                    self.width += 1;
                    self.height += 1;
                }
            },
            Vary::Decrease => {
                if self.width == WORLD_MIN_LIMIT {
                    self.message = "Unable to decrease world size, because its reaches a world min limit!".to_string();
                } else {
                    self.width -= 1;
                    self.height -= 1;
                }
            },
        }

        for row in self.cells.iter_mut() {
            row.resize(self.width, Organism::Dead);
        }

        self.cells.resize(self.width, vec![Organism::Dead;self.width]);

        // todo ? 
        // reload alive cells to new vectors
    }
    
    fn status(&mut self) {
        write!(self.stdout, "{}World size is {}x{}", termion::cursor::Goto(1,1), self.width, self.height);
        if self.message.len() > 0 {
            write!(self.stdout, "{} {}", termion::cursor::Goto(1,2), self.message);
            self.message.clear();
        }
    }

    fn dead(&self) -> bool {
        self.collect_alive().len() > 0
    }

    fn next(&mut self) {
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

    fn get_term_size(&mut self) -> (u16,u16) {
        let mut terminal: (u16,u16) = (0,0);

        match termion::terminal_size() {
            Ok(n) => {
                terminal = n;
            },
            Err(e) => {
                write!(self.stdout, "Error during call a terminal size");
            }
        }

        terminal
    }

    fn draw(&mut self) {
        let terminal = self.get_term_size();

        let center_col = terminal.0 / 2;
        let center_row = terminal.1 / 2;

        for x in 0..self.width {
            let mut row = String::new();
            
            for y in 0..self.height {
                match self.cells[x][y] {
                    Organism::Dead => row.push_str(" ▢ "),
                    Organism::Alive(_) => row.push_str(" ◉ "),
                }
            }

            let dc = center_col - (row.len() as u16 / 2) + self.width as u16;
            let dr = center_row + x as u16 - (self.height as u16 / 2);

            write!(self.stdout, "{} {}", termion::cursor::Goto(dc, dr), row);

            self.stdout.flush();
        }
    }

    fn modify(&mut self) {
        let rstdout = stdout().into_raw_mode().unwrap();

        let terminal = self.get_term_size();

        let mut cursor_position : (u16,u16) = (0,0);
        let mut cell_position = Position::new(0,0);
        
        let row_width = (self.width * 3) as u16; // width * len(" ▢ ")
        
        let center_col = terminal.0 / 2;
        let center_row = terminal.1 / 2;

        let mut initial_col = center_col - row_width / 2 + 2; // +2 - offset
        let mut initial_row = center_row - (self.height as u16 / 2);

        let offset_lr = 3; // left-right
        let offset_td = 1; // top-down

        // moves cursor to first element in field
        cursor_position.0 = initial_col; 
        cursor_position.1 = initial_row;

        self.draw();

        write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));

        self.stdout.flush();

        loop {
            let mut b: [u8; 1] = [0];

            if self.stdin.read(&mut b).is_ok() {
                match b[0] {
                    b'q' => {break},
                    b'w' => {
                        if cell_position.x == 0 {
                            write!(self.stdout, "{} Cannot move up anymore!", termion::cursor::Goto(1,1));
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        } else {
                            cell_position.x -= 1;
                            cursor_position.1 -= offset_td;
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        }
                    },
                    b's' => {
                        if cell_position.x == self.width - 1 {
                            write!(self.stdout, "{} Cannot move down anymore!", termion::cursor::Goto(1,1));
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        } else {
                            cell_position.x += 1;
                            cursor_position.1 += offset_td;
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        }
                    },
                    b'a' => {
                        if cell_position.y == 0 {
                            write!(self.stdout, "{} Cannot move left anymore!", termion::cursor::Goto(1,1));
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        } else {
                            cell_position.y -= 1;
                            cursor_position.0 -= offset_lr;
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        }
                    },
                    b'd' => {
                        if cell_position.y == self.height - 1{
                            write!(self.stdout, "{} Cannot move right anymore!", termion::cursor::Goto(1,1));
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        } else {
                            cell_position.y += 1;
                            cursor_position.0 += offset_lr;
                            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));
                        }
                    },
                    b'i' => {
                        match self.cells[cell_position.x][cell_position.y] {
                            Organism::Dead => {
                                self.cells[cell_position.x][cell_position.y] = Organism::Alive(Position { x: cell_position.x, y: cell_position.y })
                            },
                            Organism::Alive(_) => {
                                write!(self.stdout, "{} Cannot add new alive organism, because this cell already has alive organism!", termion::cursor::Goto(1,1));
                            },
                        }
                    },
                    b'o' => {
                        match self.cells[cell_position.x][cell_position.y] {
                            Organism::Dead => {
                                write!(self.stdout, "{} Cannot delete alive organism, because this cell already has dead organism!", termion::cursor::Goto(1,1));
                            },
                            Organism::Alive(_) => {
                                self.cells[cell_position.x][cell_position.y] = Organism::Dead;
                            },
                        }
                    }
                    _ => {}
                }
            } 

            self.draw();

            write!(self.stdout, "{}", termion::cursor::Goto(cursor_position.0, cursor_position.1));

            self.stdout.flush();

            thread::sleep(std::time::Duration::from_millis(500));
        }
    }
    
    fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All);
    }

    fn resize(&mut self) {
        let rstdout = stdout().into_raw_mode().unwrap();

        self.draw();

        loop {
            let mut b: [u8; 1] = [0];

            if self.stdin.read(&mut b).is_ok() {
                match b[0] {
                    b'q' => {break},
                    b'+' => self.change_world(Vary::Increase),
                    b'-' => self.change_world(Vary::Decrease),
                    _ => {}
                }
            }
            self.clear();
            self.status();
            self.draw();

            self.stdout.flush();
            thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    pub fn process(&mut self) {                
        let rstdout = stdout().into_raw_mode().unwrap();
        
        loop {
            let mut b: [u8; 1] = [0];

            let alive = self.collect_alive().len();
            let size = self.width.clone();

            Menu::draw_menu(&mut self.stdout);
            Menu::draw_info(&mut self.stdout, alive, size);

            if self.stdin.read(&mut b).is_ok() {
                match b[0] {
                    b'q' => {
                        self.state = State::Exit;
                    },
                    b'r' => self.state = State::Resize,
                    b'd' => self.state = State::Draw,
                    b'm' => self.state = State::Modify,
                    _ => {},
                }
            }

            match self.state {
                State::Draw => {
                    self.draw();
                    self.next(); 
                },
                State::Resize => {
                    self.resize();
                    self.state = State::Draw;
                },
                State::Modify => {
                    self.modify();
                    self.state = State::Draw;
                },
                State::Exit => {
                    write!(self.stdout,"{} {}\n", termion::clear::UntilNewline, "Goodbye!");

                    break;
                }
            }
            
            thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}

#[cfg(test)]
mod tests {

    mod neighbours {
        use crate::{life::Organism, world::World};
        
        #[test]
        fn collect_alive_neighbours() {
            let mut stdin = stdin();
            let mut stdout = stdout();
            
            let world = World::new(
                3,
                3,
                stdin,
                stdout,
                vec![
                    Organism::Alive(crate::position::Position { x: 1, y: 1 }),
                    Organism::Alive(crate::position::Position { x: 0, y: 0 }),
                    Organism::Alive(crate::position::Position { x: 0, y: 2 }),
                ],
            );

            let collected = world.collect_alive();

            assert_eq!(collected.len(), 3);
        }

        #[test]
        fn calculate_neighbours() {
            let mut stdin = stdin();
            let mut stdout = stdout();
            
            let world = World::new(
                3,
                3,
                stdin,
                stdout,
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
