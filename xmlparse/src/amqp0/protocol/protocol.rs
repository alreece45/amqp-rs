// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use xml::reader::XmlEvent;

use amqp0::{Version, ParseError};
use amqp0::{VoidParser, ParseError};
use super::{Child, Class, Constant, Domain};
use super::{ClassParser, ConstantParser, DomainParser, ChildParser};

#[derive(Debug)]
pub struct Protocol<'a> {
    classes: Vec<Class<'a>>,
    domains: HashMap<String, Domain<'a>>,
    version: Version,
    constants: Vec<Constant<'a>>,
}

impl<'a> Protocol<'a> {
    pub fn new(version: Version) -> Self {
        Protocol {
            domains: HashMap::new(),
            classes: vec![],
            constants: vec![],
            version: version,
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
    pub fn version(&self) -> &Version {
        &self.version
    }
}

pub enum Parser<'a> {
    Start,
    Idle(Protocol<'a>),
    Child(Protocol<'a>, ChildParser<'a>),
    Finished(Protocol<'a>),
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser::Start
    }

    pub fn into_protocol(self) -> Result<Protocol<'a>, ParseError> {
        match self {
            Parser::Start => Err(ParseError::ExpectedElementStart("amqp".into())),
            Parser::Idle(p) | Parser::Child(p, _) | Parser::Finished(p) => Ok(p),
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
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
                                    .map_err(|_| ParseError::invalid_value("amqp", name.to_string(), "u8", value.to_string()))
                                );
                                Ok((name, value))
                            })
                            .collect::<Result<HashMap<&str, u8>, ParseError>>()
                        );

                        let major = try!(attributes.remove("major").ok_or_else(|| {
                            ParseError::expected_attribute("protocol", "major")
                        }));
                        let minor = try!(attributes.remove("minor").ok_or_else(|| {
                            ParseError::expected_attribute("protocol", "minor")
                        }));
                        let revision = attributes.remove("revision").unwrap_or(0);
                        let version = Version::new(major, minor, revision);

                        Parser::Idle(Protocol::new(version))
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

impl<'a> Default for Parser<'a> {
    fn default() -> Self {
        Parser::new()
    }
}
