use std::time::Duration;

use game_of_life::{Screen, Cell, clear_screen};

fn main() {
    let (width, height) = crossterm::terminal::size().unwrap();

    let mut screen = Screen::new(width, height);
    
    let mut i: usize = 1;
    loop {
        clear_screen();
        print!("{}", screen);
        screen[5][i] = Cell::Alive;
        i += 1;
        if i >= width.into() { break; }
        std::thread::sleep(Duration::from_millis(200));
    }
}
