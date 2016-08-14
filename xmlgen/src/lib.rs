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

extern crate amqp_xmlparse as parser;
extern crate inflections;

use std::borrow::Cow;
use std::io;

mod common;
pub mod amqp0;

pub trait WriteRust<P> {
    fn write_rust<W>(&self, protocol: &P, writer: &mut W) -> Result<(), Error>
        where W: io::Write;
}

pub trait WrittenBy<T> {
    fn write_rust_for<W>(&self, _type: &T, writer: &mut W) -> Result<(), Error>
        where W: io::Write;
}

impl<P, T> WrittenBy<T> for P
    where T: WriteRust<P>
{
    fn write_rust_for<W>(&self, _type: &T, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        _type.write_rust(self, writer)
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    UnknownType(Cow<'static, str>),
    InvalidValue(Cow<'static, str>, Cow<'static, str>),
}

impl<'a> From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
