// Copyright (C) 2021 Leandro Lisboa Penz <lpenz@lpenz.org>
// This file is subject to the terms and conditions defined in
// file 'LICENSE', which is part of this source code package.

use std::convert::TryFrom;
use std::convert::TryInto;
use std::io;
use std::str::FromStr;

use crate::core::*;
use crate::error::*;

impl FromStr for Params {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ints = s.split(' ').map(String::from).collect::<Vec<_>>();
        debug_assert!(ints.len() == 3);
        Ok(Params {
            width: ints[1].parse()?,
            height: ints[0].parse()?,
            countdown: ints[2].parse()?,
        })
    }
}

pub fn input_first(
    lineit: &mut impl Iterator<Item = io::Result<String>>,
    params: &mut Params,
    _node: &mut Node,
) -> Result<(), Error> {
    let line = lineit.next().unwrap()?;
    *params = line.parse::<Params>()?;
    Ok(())
}

pub fn input(
    mut lineit: impl Iterator<Item = io::Result<String>>,
    params: &Params,
    node: &mut Node,
) -> Result<(), Error> {
    let line = lineit.next().unwrap()?;
    let kirk_qa = line.split(' ').collect::<Vec<_>>();
    debug_assert!(kirk_qa.len() == 2);
    node.kirk = (kirk_qa[1].parse::<u16>()?, kirk_qa[0].parse::<u16>()?).try_into()?;
    for (y, lineres) in (0..params.height).zip(lineit) {
        let mazeline = node.maze.line_mut(y);
        for (x, c) in lineres?.chars().enumerate() {
            mazeline[x] = Cell::try_from(c)?;
        }
    }
    Ok(())
}
