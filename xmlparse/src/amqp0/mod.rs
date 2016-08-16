// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use xml::reader::{XmlEvent, EventReader, Error as XmlError};

mod protocol;

use self::protocol::ProtocolParser as Parser;
pub use self::protocol::{Class, Constant, Domain, Field, Method, Protocol};

pub fn parse<'a, P>(path: P) -> Result<Protocol<'a>, ParseError>
    where P: AsRef<Path>
{
    let path = path.as_ref();
    let file = try!(File::open(&path));
    let file = BufReader::new(file);

    let mut parser = Parser::new();

    for event in EventReader::new(file) {
        let event = try!(event);
        parser = try!(parser.parse(&event));
    }

    Ok(try!(parser.into_protocol()))
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedAttribute(Cow<'static, str>, Cow<'static, str>),
    ExpectedElementStart(Cow<'static, str>),
    ExpectedAmqpRoot,
    // No more events are expected
    ExpectedEnd,
    // element, attribute, type, value
    InvalidValue(Cow<'static, str>, Cow<'static, str>, &'static str, Cow<'static, str>),
    Io(io::Error),
    Xml(XmlError),
}

impl<'a> From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}

impl<'a> From<XmlError> for ParseError {
    fn from(e: XmlError) -> Self {
        ParseError::Xml(e)
    }
}
