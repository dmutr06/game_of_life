use std::{fmt::Display, ops::{Deref, DerefMut}, process::Command};

pub const DEAD_CHAR: char = ' ';
pub const ALIVE_CHAR: char = '*';

#[derive(Clone, Copy)]
pub enum Cell {
    Dead,
    Alive,
}

pub struct Screen(Vec<Vec<Cell>>);

impl Screen {
    pub fn new<T: Into<usize>>(width: T, height: T) -> Screen {
        Screen(vec![vec![Cell::Dead; width.into()]; height.into()])
    }
}

impl Deref for Screen {
    type Target = Vec<Vec<Cell>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut screen = String::new();
        for y in &self.0 {
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