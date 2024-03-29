// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert;
use std::fmt;

use crate::error::*;

pub const MAX_WIDTH: u16 = 200;
pub const MAX_HEIGHT: u16 = 100;
// pub const MAX_WIDTH: u16 = 30;
// pub const MAX_HEIGHT: u16 = 15;
pub const GRIDBOOL_WORDS: usize = (MAX_WIDTH as usize * MAX_HEIGHT as usize - 1) / 32 + 1;

pub type Sqrid = crate::sqrid_create!(MAX_WIDTH, MAX_HEIGHT, false);
pub type Qa = crate::qa_create!(Sqrid);
pub type Qr = crate::Qr;
pub type Maze = crate::grid_create!(Sqrid, Cell);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Unknown,
    Wall,
    Space,
    Start,
    Control,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Unknown
    }
}

impl convert::TryFrom<char> for Cell {
    type Error = Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '?' => Ok(Cell::Unknown),
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Space),
            'T' => Ok(Cell::Start),
            'C' => Ok(Cell::Control),
            _ => Err(Error::InvalidCellChar),
        }
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> char {
        match cell {
            Cell::Unknown => '?',
            Cell::Wall => '#',
            Cell::Space => '.',
            Cell::Start => 'T',
            Cell::Control => 'C',
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Default)]
pub struct Params {
    pub width: u16,
    pub height: u16,
    pub countdown: u16,
}

#[derive(Debug, Default)]
pub struct Node {
    pub kirk: Qa,
    pub maze: Maze,
}

impl Node {
    pub fn bfs(&self, goal: Cell) -> Option<(Qa, Vec<Qr>)> {
        Sqrid::bfs_path(
            |qa0, qr| {
                let qa: Option<Qa> = qa0 + qr;
                qa.filter(|qa| self.maze[qa] != Cell::Wall)
            },
            &self.kirk,
            |qa| self.maze[qa] == goal,
        )
        .ok()
    }
}
