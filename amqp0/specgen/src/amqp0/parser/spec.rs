// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use xml::reader::XmlEvent;

use amqp0::{Spec, Version};
use amqp0::parser::Error;
use parser::VoidParser;

use super::Child;
use super::{ChildParser, ClassParser, ConstantParser, DomainParser};

pub enum Parser<'a> {
    Start,
    Idle(Spec<'a>),
    Child(Spec<'a>, ChildParser<'a>),
    Finished(Spec<'a>),
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser::Start
    }

    pub fn into_spec(self) -> Result<Spec<'a>, Error> {
        match self {
            Parser::Start => Err(Error::ExpectedElementStart("amqp".into())),
            Parser::Idle(p) |
            Parser::Child(p, _) |
            Parser::Finished(p) => Ok(p),
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            // Start
            Parser::Start => {
                match *event {
                    XmlEvent::StartDocument { .. } => Parser::Start,
                    XmlEvent::StartElement { name: ref e, ref attributes, .. } if e.local_name == "amqp" => {
                        let mut attributes = try!(attributes.iter()
                            .map(|a| (a.name.local_name.as_str(), &a.value))
                            .filter(|&(name, _)| name == "major" || name == "minor" || name == "revision")
                            .map(|(name, value)| {
                                let value = try!(value.parse::<u8>()
                                    .map_err(|_| {
                                        let name = match name {
                                            "major" => "major",
                                            "minor" => "minor",
                                            "revision" => "revision",
                                            _ => unreachable!(),
                                        };
                                        Error::InvalidValue("amqp", name, "u8", value.to_string())
                                    })
                                );
                                Ok((name, value))
                            })
                            .collect::<Result<HashMap<&str, u8>, Error>>()
                        );

                        let major = try!(attributes.remove("major").ok_or_else(|| {
                            Error::ExpectedAttribute("amqp", "major")
                        }));
                        let minor = try!(attributes.remove("minor").ok_or_else(|| {
                            Error::ExpectedAttribute("amqp", "minor")
                        }));
                        let revision = attributes.remove("revision").unwrap_or(0);
                        let version = Version::new(major, minor, revision);

                        Parser::Idle(Spec::new(version))
                    },
                    _ => return Err(Error::ExpectedAmqpRoot),
                }
            },

            // Idle
            Parser::Idle(spec) => {
                match *event {
                    XmlEvent::StartElement { name: ref e, .. } => {
                        let child_parser = match e.local_name.as_str() {
                            "class"    => try!(ClassParser::from_xml_event(event)).into(),
                            "constant" => try!(ConstantParser::from_xml_event(event)).into(),
                            "domain"   => try!(DomainParser::from_xml_event(event)).into(),
                            _ => VoidParser::new().into(),
                        };
                        Parser::Child(spec, ChildParser::new(child_parser))
                    },
                    XmlEvent::EndDocument => Parser::Finished(spec),
                    _ => Parser::Idle(spec),
                }
            },
            // Child
            Parser::Child(mut spec, parser) => {
                match try!(parser.parse(event)) {
                    ChildParser::Finished(child) => {
                        if let Some(child) = child {
                            match child {
                                Child::Class(class) => {
                                    spec.classes.insert(class.name().to_string(), class);
                                },
                                Child::Constant(constant) => {
                                    spec.constants.insert(constant.name().to_string(), constant);
                                },
                                Child::Domain(domain) => {
                                    spec.domains.insert(domain.name().to_string(), domain);
                                },
                            }
                        }
                        Parser::Idle(spec)
                    },
                    parser => Parser::Child(spec, parser),
                }
            },
            // End
            Parser::Finished(_) => return Err(Error::ExpectedEnd),
        })
    }
}

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Parser::new()
    }
}
