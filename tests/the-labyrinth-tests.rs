// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use anyhow::Result;

use the_labyrinth::core::*;
use the_labyrinth::input::*;

#[test]
fn test_input() -> Result<()> {
    let vec_lines = vec![
        "10 30 23",
        "3 6",
        "??????????????????????????????",
        "????.....?????????????????????",
        "????#####?????????????????????",
        "????..T..?????????????????????",
        "????.....?????????????????????",
        "????#####?????????????????????",
        "??????????????????????????????",
        "??????????????????????????????",
        "??????????????????????????????",
        "??????????????????????????????",
    ];
    let mut it_lines = vec_lines.iter().cloned().map(String::from).map(Ok);
    let mut params = Params::default();
    let mut node = Node::default();
    input_first(&mut it_lines, &mut params, &mut node)?;
    assert_eq!(params.width, 30);
    assert_eq!(params.height, 10);
    assert_eq!(params.countdown, 23);
    input(&mut it_lines, &mut params, &mut node)?;
    let kirk_tuple = <(u16, u16)>::from(node.kirk);
    assert_eq!(kirk_tuple.0, 6);
    assert_eq!(kirk_tuple.1, 3);
    assert_eq!(node.maze[node.kirk], Cell::Start);
    assert!(it_lines.next().is_none());
    Ok(())
}
