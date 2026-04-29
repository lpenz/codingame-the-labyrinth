// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

extern crate the_labyrinth;
pub use the_labyrinth::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    entrypoint::main()?;
    Ok(())
}
