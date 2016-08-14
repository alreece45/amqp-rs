// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;

use super::Child;
use amqp0::{VoidParser, ParseError};
use amqp0::protocol::{ClassParser, ConstantParser, DomainParser};

#[derive(Debug)]
pub enum Parser<'a> {
    Void(VoidParser),
    Class(ClassParser<'a>),
    Constant(ConstantParser<'a>),
    Domain(DomainParser<'a>),
}

impl<'a> Parser<'a> {
    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            Parser::Void(p) => Parser::Void(try!(p.parse(event))),
            Parser::Constant(p) => Parser::Constant(try!(p.parse(event))),
            Parser::Domain(p) => Parser::Domain(try!(p.parse(event))),
            Parser::Class(p) => Parser::Class(try!(p.parse(event))),
        })
    }

    pub fn child(self) -> Result<Option<Child<'a>>, Self> {
        match self {
            Parser::Void(parser) => {
                match parser {
                    VoidParser::Finished => Ok(None),
                    parser => Err(Parser::Void(parser)),
                }
            }
            Parser::Constant(parser) => {
                match parser {
                    ConstantParser::Finished(c) => Ok(Some(c.into())),
                    parser => Err(Parser::Constant(parser)),
                }
            }
            Parser::Domain(parser) => {
                match parser {
                    DomainParser::Finished(d) => Ok(Some(d.into())),
                    parser => Err(Parser::Domain(parser)),
                }
            }
            Parser::Class(parser) => {
                match parser {
                    ClassParser::Finished(c) => Ok(Some(c.into())),
                    parser => Err(Parser::Class(parser)),
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        match *self {
            Parser::Void(VoidParser::Finished)
                | Parser::Constant(ConstantParser::Finished(_))
                | Parser::Domain(DomainParser::Finished(_))
                | Parser::Class(ClassParser::Finished(_))       => true,
            _ => false,
        }
    }
}

impl<'a> From<VoidParser> for Parser<'a> {
    fn from(parser: VoidParser) -> Self {
        Parser::Void(parser)
    }
}

impl<'a> From<ClassParser<'a>> for Parser<'a> {
    fn from(parser: ClassParser<'a>) -> Self {
        Parser::Class(parser)
    }
}

impl<'a> From<ConstantParser<'a>> for Parser<'a> {
    fn from(parser: ConstantParser<'a>) -> Self {
        Parser::Constant(parser)
    }
}

impl<'a> From<DomainParser<'a>> for Parser<'a> {
    fn from(parser: DomainParser<'a>) -> Self {
        Parser::Domain(parser)
    }
}
