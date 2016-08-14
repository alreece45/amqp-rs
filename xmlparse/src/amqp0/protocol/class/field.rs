// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use xml::reader::XmlEvent;

use amqp0::{VoidParser, ParseError};

#[derive(Debug)]
pub struct Field<'a> {
    name: Cow<'a, str>,
    domain: Cow<'a, str>,
    max_length: Option<usize>,
    is_optional: bool,
    is_reserved: bool,
}

impl<'a> Field<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    pub fn is_reserved(&self) -> bool {
        self.is_reserved
    }
}

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Field<'a>),
    Void(Field<'a>, VoidParser),
    Finished(Field<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, ParseError> {
        if let XmlEvent::StartElement { ref attributes, .. } = *event {
            let (mut name, mut domain, mut is_reserved) = (None, None, false);

            for attribute in attributes.iter() {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "domain" => domain = domain.or_else(|| Some(attribute.value.clone())),
                    "type" => domain = domain.or_else(|| Some(attribute.value.clone())),
                    "reserved" => is_reserved = is_reserved || attribute.value == "1",
                    _ => (),

                };
            }

            let name = try!(name.ok_or_else(|| {
                ParseError::ExpectedAttribute("field".into(), "name".into())
            }));
            let domain = try!(domain.ok_or_else(|| {
                ParseError::ExpectedAttribute("field".into(), "domain".into())
            }));

            Ok(Parser::Idle(Field {
                name: name.into(),
                domain: domain.into(),
                max_length: None,
                is_optional: false,
                is_reserved: is_reserved,
            }))
        } else {
            Err(ParseError::ExpectedElementStart("field".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            Parser::Idle(argument) => {
                match *event {
                    XmlEvent::StartElement { .. } => Parser::Void(argument, VoidParser::new()),
                    XmlEvent::EndElement { .. } => Parser::Finished(argument),
                    _ => Parser::Idle(argument),
                }
            }
            Parser::Void(argument, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(argument),
                    parser => Parser::Void(argument, parser),
                }
            }
            Parser::Finished(_) => return Err(ParseError::ExpectedEnd),
        })
    }
}
