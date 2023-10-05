use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    process::Command,
};

pub const DEAD_CHAR: char = ' ';
pub const ALIVE_CHAR: char = 'a';

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Clone, Debug)]
pub struct Screen {
    screen: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn new<T: Into<usize>>(width: T, height: T) -> Screen {
        let (width, height) = (width.into(), height.into());
        Screen {
            screen: vec![vec![Cell::Dead; width]; height],
            width,
            height,
        }
    }

    fn neighbours(&self, x: usize, y: usize) -> u8 {
        let mut counter = 0;
        for i in 0..=2 {
            let y = y as i32;
            let cur_y = y + i - 1;
            if cur_y < 0 {
                continue;
            }
            if let Some(line) = self.get(cur_y as usize) {
                for j in 0..=2 {
                    let x = x as i32;
                    let cur_x: i32 = x + j - 1;

                    if cur_y == y && cur_x == x {
                        continue;
                    }
                    if cur_x < 0 {
                        continue;
                    }
                    if let Some(Cell::Alive) = line.get(cur_x as usize) {
                        counter += 1;
                    }
                }
            }
        }

        counter
    }
}

impl Iterator for Screen {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        clear_screen();
        print!("{}", &self);
        let prev_frame = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let cur = self[y][x];
                let nbors = prev_frame.neighbours(x, y);
                match cur {
                    Cell::Dead => {
                        if nbors == 3 {
                            self[y][x] = Cell::Alive;
                        }
                    }
                    Cell::Alive => {
                        if nbors < 2 || nbors > 3 {
                            self[y][x] = Cell::Dead;
                        }
                    }
                }
            }
        }

        if prev_frame.to_string() == self.to_string() {
            return None;
        }

        Some(())
    }
}

impl Deref for Screen {
    type Target = Vec<Vec<Cell>>;
    fn deref(&self) -> &Self::Target {
        &self.screen
    }
}

impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.screen
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut screen = String::new();
        for y in &self.screen {
            for x in y {
                screen.push((*x).into());
            }
        }
        write!(f, "{}", screen)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch: char = (*self).into();
        write!(f, "{}", ch)
    }
}

impl Into<char> for Cell {
    fn into(self) -> char {
        match self {
            Self::Alive => ALIVE_CHAR,
            Self::Dead => DEAD_CHAR,
        }
    }
}

#[cfg(windows)]
pub fn clear_screen() {
    Command::new("cmd")
        .args(["/c", "cls"])
        .spawn()
        .expect("cls command failed to start")
        .wait()
        .expect("failed to wait");
    if cfg!(target_os = "windows") {
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}

#[cfg(not(windows))]
pub fn clear_screen() {
    Command::new("clear")
        .spawn()
        .expect("clear command failed to start")
        .wait()
        .expect("failed to wait");
}
