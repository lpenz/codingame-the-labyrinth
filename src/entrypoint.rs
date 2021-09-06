// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::error::Error;
use std::io;
use std::io::BufRead;

use crate::core::*;
use crate::input::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut params = Params::default();
    let mut node = Node::default();
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    input_first(&mut stdin_lines, &mut params, &mut node)?;
    eprintln!("Dimensions: {}x{}", params.width, params.height);
    input(&mut stdin_lines, &params, &mut node)?;
    // Discovery phase:
    loop {
        eprintln!("Discovery phase");
        if let Some(destinfo) = node.bfs(Cell::Unknown) {
            println!("{}", destinfo.1[0].name_direction());
        } else {
            break;
        }
        input(&mut stdin_lines, &params, &mut node)?;
    }
    // Activation phase:
    while node.maze[node.kirk] != Cell::Control {
        eprintln!("Activation phase");
        if let Some(destinfo) = node.bfs(Cell::Control) {
            println!("{}", destinfo.1[0].name_direction());
        } else {
            break;
        }
        input(&mut stdin_lines, &params, &mut node)?;
    }
    // Escape phase:
    while node.maze[node.kirk] != Cell::Start {
        eprintln!("Escape phase");
        if let Some(destinfo) = node.bfs(Cell::Start) {
            println!("{}", destinfo.1[0].name_direction());
        } else {
            break;
        }
        input(&mut stdin_lines, &params, &mut node)?;
    }
    Ok(())
}
