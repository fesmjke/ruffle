use std::io::{Write, Stdout};

static MENU: &'static str = "
╔══════════════════════════╗
║ Conway's Game of Life    ║
╠══════════════════════════╣
║ [r] - - - - - - - Resize ║
║ [m] - - - - - - - Modify ║
║ [d] - - - - - - - Draw   ║
║ [q] - - - - - - - Exit   ║
╠══════════════════════════╣
";

static INFO: &'static str = "
║ Alive cells -     #      ║
╠══════════════════════════╣
║ World size -      ?      ║
╚══════════════════════════╝
";

pub struct Menu;

impl Menu {
    pub fn draw_menu(stdout : &mut Stdout) {
        write!(stdout, "{}", termion::clear::All);
        for (index,line) in MENU.trim().lines().enumerate() {
            let dr = 1 + index as u16;

            write!(stdout, "{} {}", termion::cursor::Goto(1,dr), line);
        }
    }

    pub fn draw_info(stdout : &mut Stdout, alive: usize, size: usize) {
        let offset = MENU.lines().count() as u16;

        for (index,line) in INFO.trim().lines().enumerate() {
            let temp = line.clone();

            let dr = offset + index as u16;
            
            if line.contains("#") {
                let lives = format!("{}", alive);
                
                if alive > 10 {
                    write!(stdout, "{} {}", termion::cursor::Goto(1,dr), temp.replace("# ", lives.as_str()));
                } else {
                    write!(stdout, "{} {}", termion::cursor::Goto(1,dr), temp.replace("#", lives.as_str()));
                } 
            } else if line.contains("?") {
                let sz = format!("{}x{}", size, size);

                if sz.len() < 5 {
                    write!(stdout, "{} {}", termion::cursor::Goto(1,dr), temp.replace(" ? ",  sz.as_str()));
                } else {
                    write!(stdout, "{} {}", termion::cursor::Goto(1,dr), temp.replace("  ?  ",  sz.as_str()));
                }
            } else {
                write!(stdout, "{} {}", termion::cursor::Goto(1,dr), line);
            }
        }
    }
}