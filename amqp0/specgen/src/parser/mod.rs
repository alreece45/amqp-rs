// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod child;
mod class;
mod constant;
mod domain;
mod spec;
mod void;

use std::io;
use xml::reader::Error as XmlError;

pub use self::child::{Child, Parser as ChildParser};
pub use self::class::{FieldParser, Parser as ClassParser};
pub use self::constant::Parser as ConstantParser;
pub use self::domain::Parser as DomainParser;
pub use self::spec::Parser;
pub use self::void::VoidParser;

#[derive(Debug)]
pub enum Error {
    ExpectedAttribute(&'static str, &'static str),
    ExpectedElementStart(&'static str),
    ExpectedAmqpRoot,
    // No more events are expected
    ExpectedEnd,
    // element, attribute, type, value
    InvalidValue(&'static str, &'static str, &'static str, String),
    Io(io::Error),
    Xml(XmlError),
}

impl<'a> From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl<'a> From<XmlError> for Error {
    fn from(e: XmlError) -> Self {
        Error::Xml(e)
    }
}