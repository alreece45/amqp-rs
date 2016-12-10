// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;

use amqp0::Domain;
use amqp0::parser::Error;
use parser::VoidParser;

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Domain<'a>),
    Void(Domain<'a>, VoidParser),
    Finished(Domain<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, Error> {
        if let XmlEvent::StartElement { name: ref e, ref attributes, .. } = *event {
            if e.local_name != "domain" {
                return Err(Error::ExpectedElementStart("domain".into()));
            }

            let (mut name, mut mapping) = (None, None);
            for attribute in attributes.iter() {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "type" => mapping = mapping.or_else(|| Some(attribute.value.clone())),
                    _ => (),
                };
            }

            let name = try!(name.ok_or_else(|| {
                Error::ExpectedAttribute("field".into(), "name".into())
            }));
            let mapping = try!(mapping.ok_or_else(|| {
                Error::ExpectedAttribute("field".into(), "domain".into())
            }));

            Ok(Parser::Idle(Domain {
                name: name.into(),
                mapping: mapping.into(),
            }))
        } else {
            Err(Error::ExpectedElementStart("field".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            Parser::Idle(domain) => {
                match *event {
                    XmlEvent::StartElement { .. } => Parser::Void(domain, VoidParser::new()),
                    XmlEvent::EndElement { .. } => Parser::Finished(domain),
                    _ => Parser::Idle(domain),
                }
            },
            Parser::Void(domain, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(domain),
                    parser => Parser::Void(domain, parser),
                }
            },
            Parser::Finished(_) => return Err(Error::ExpectedEnd),
        })
    }
}
