// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;

use amqp0::Constant;
use amqp0::parser::Error;
use parser::VoidParser;

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Constant<'a>),
    Void(Constant<'a>, VoidParser),
    Finished(Constant<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, Error> {
        if let XmlEvent::StartElement { name: ref element, ref attributes, .. } = *event {
            if element.local_name != "constant" {
                return Err(Error::ExpectedElementStart("constant".into()));
            }

            let (mut name, mut value, mut class) = (None, None, None);
            for attribute in attributes {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "value" => value = value.or_else(|| Some(attribute.value.clone())),
                    "class" => class = class.or_else(|| Some(attribute.value.clone())),
                    _ => (),
                }
            }

            let name = try!(name.ok_or_else(|| {
                Error::ExpectedAttribute("constant".into(), "name".into())
            }));
            let value = try!(value.ok_or_else(|| {
                Error::ExpectedAttribute("constant".into(), "value".into())
            }));

            Ok(Parser::Idle(Constant {
                name: name.into(),
                value: value.into(),
                class: class.map(|c| c.into()),
            }))
        } else {
            Err(Error::ExpectedElementStart("constant".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            Parser::Idle(argument) => {
                match *event {
                    XmlEvent::StartElement { .. } => Parser::Void(argument, VoidParser::new()),
                    XmlEvent::EndElement { .. } => Parser::Finished(argument),
                    _ => Parser::Idle(argument),
                }
            },
            Parser::Void(argument, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(argument),
                    parser => Parser::Void(argument, parser),
                }
            },
            Parser::Finished(_) => return Err(Error::ExpectedEnd),
        })
    }
}
