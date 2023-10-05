use std::time::Duration;

use game_of_life::{Cell, Screen};

fn main() {
    let (width, height) = crossterm::terminal::size().unwrap();

    let mut screen = Screen::new(width, height);

    screen[5][5] = Cell::Alive;
    screen[5][4] = Cell::Alive;
    screen[4][3] = Cell::Alive;
    screen[4][5] = Cell::Alive;
    screen[3][5] = Cell::Alive;

    for _ in screen {
        std::thread::sleep(Duration::from_millis(200));
    }
}
