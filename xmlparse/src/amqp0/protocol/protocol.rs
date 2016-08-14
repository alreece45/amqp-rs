// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use xml::reader::XmlEvent;

use amqp0::{VoidParser, ParseError};
use super::{Child, Class, Constant, Domain};
use super::{ClassParser, ConstantParser, DomainParser, ChildParser};

#[derive(Debug)]
pub struct Protocol<'a> {
    classes: Vec<Class<'a>>,
    domains: HashMap<String, Domain<'a>>,
    constants: Vec<Constant<'a>>,
}

impl<'a> Protocol<'a> {
    pub fn new() -> Self {
        Protocol {
            domains: HashMap::new(),
            classes: vec![],
            constants: vec![],
        }
    }

    pub fn domain<S>(&self, domain: S) -> Option<&Domain<'a>>
        where S: AsRef<str>
    {
        self.domains.get(domain.as_ref())
    }

    pub fn classes(&self) -> &Vec<Class<'a>> {
        &self.classes
    }
    pub fn constants(&self) -> &Vec<Constant<'a>> {
        &self.constants
    }
}

pub enum Parser<'a> {
    Start(Protocol<'a>),
    Idle(Protocol<'a>),
    Child(Protocol<'a>, ChildParser<'a>),
    Finished(Protocol<'a>),
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser::Start(Protocol::new())
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            // Start
            Parser::Start(protocol) => {
                match *event {
                    XmlEvent::StartDocument { .. } => Parser::Start(protocol),
                    XmlEvent::StartElement { name: ref e, .. } if e.local_name == "amqp" => {
                        Parser::Idle(protocol)
                    }
                    _ => return Err(ParseError::ExpectedAmqpRoot),
                }
            },

            // Idle
            Parser::Idle(protocol) => {
                match *event {
                    XmlEvent::StartElement { name: ref e, .. } => {
                        let child_parser = match e.local_name.as_str() {
                            "class"    => try!(ClassParser::from_xml_event(event)).into(),
                            "constant" => try!(ConstantParser::from_xml_event(event)).into(),
                            "domain"   => try!(DomainParser::from_xml_event(event)).into(),
                            _ => VoidParser::new().into(),
                        };
                        Parser::Child(protocol, ChildParser::new(child_parser))
                    },
                    XmlEvent::EndDocument => Parser::Finished(protocol),
                    _ => Parser::Idle(protocol),
                }
            },
            // Child
            Parser::Child(mut protocol, parser) => {
                match try!(parser.parse(event)) {
                    ChildParser::Finished(child) => {
                        if let Some(child) = child {
                            match child {
                                Child::Class(class) => protocol.classes.push(class),
                                Child::Constant(constant) => protocol.constants.push(constant),
                                Child::Domain(domain) => {
                                    protocol.domains.insert(domain.name().to_string(), domain);
                                },
                            }
                        }
                        Parser::Idle(protocol)
                    }
                    parser => Parser::Child(protocol, parser),
                }
            },
            // End
            Parser::Finished(_) => return Err(ParseError::ExpectedEnd),
        })
    }
}

impl<'a> From<Parser<'a>> for Protocol<'a> {
    fn from(parser: Parser<'a>) -> Protocol<'a> {
        match parser {
            Parser::Start(p)
             | Parser::Idle(p)
             | Parser::Child(p, _)
             | Parser::Finished(p) => p,
        }
    }
}

impl<'a> Default for Protocol<'a> {
    fn default() -> Self {
        Protocol::new()
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Parser::new()
    }
}
