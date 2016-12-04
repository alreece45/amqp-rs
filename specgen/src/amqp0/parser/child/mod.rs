// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod inner;

use amqp0::{Class, Constant, Domain};
use amqp0::parser::Error;

use xml::reader::XmlEvent;

pub use self::inner::Parser as InnerParser;

#[derive(Debug)]
pub enum Child<'a> {
    Constant(Constant<'a>),
    Domain(Domain<'a>),
    Class(Class<'a>),
}

#[derive(Debug)]
pub enum Parser<'a> {
    Parsing(InnerParser<'a>),
    Finished(Option<Child<'a>>),
}

impl<'a> Parser<'a> {
    pub fn new(parser: InnerParser<'a>) -> Self {
        Parser::Parsing(parser.into())
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        match self {
            Parser::Parsing(parser) => {
                let parser = try!(parser.parse(event));
                Ok(match parser.child() {
                    Ok(child) => Parser::Finished(child.into()),
                    Err(parser) => Parser::Parsing(parser),
                })
            },
            _ => Err(Error::ExpectedEnd),
        }
    }
}


impl<'a> From<Constant<'a>> for Child<'a> {
    fn from(child: Constant<'a>) -> Self {
        Child::Constant(child)
    }
}

impl<'a> From<Domain<'a>> for Child<'a> {
    fn from(child: Domain<'a>) -> Self {
        Child::Domain(child)
    }
}

impl<'a> From<Class<'a>> for Child<'a> {
    fn from(child: Class<'a>) -> Self {
        Child::Class(child)
    }
}
