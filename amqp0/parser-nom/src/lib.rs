// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(not(feature="clippy"), allow(unknown_lints))]

#[cfg(not(feature = "amqp0-build-parser"))]
include!(concat!("../pregen/mod.rs"));
#[cfg(feature = "amqp0-build-parser")]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

#[macro_use]
extern crate nom;
extern crate amqp0_primitives as primitives;

#[cfg(not(feature = "lifeguard"))]
extern crate lifeguard;

mod common;
mod field;

pub mod pool;

use nom::IResult;
use pool::ParserPool;

pub trait NomBytes<'a>: Sized {
    type Output: 'a;
    fn nom_bytes<'b, P>(&'a [u8], &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool;
}