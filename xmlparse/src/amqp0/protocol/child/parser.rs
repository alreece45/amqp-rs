// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;

use amqp0::ParseError;
use super::{Child, ChainedParser};

#[derive(Debug)]
pub enum Parser<'a> {
    Parsing(ChainedParser<'a>),
    Finished(Option<Child<'a>>),
}

impl<'a> Parser<'a> {
    pub fn new(parser: ChainedParser<'a>) -> Self {
        Parser::Parsing(parser.into())
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        match self {
            Parser::Parsing(parser) => {
                let parser = try!(parser.parse(event));
                Ok(match parser.child() {
                    Ok(child) => Parser::Finished(child.into()),
                    Err(parser) => Parser::Parsing(parser),
                })
            }
            _ => Err(ParseError::ExpectedEnd),
        }
    }
}
