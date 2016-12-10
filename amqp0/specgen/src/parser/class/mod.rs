// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod field;
mod method;

use xml::reader::XmlEvent;
use {Class, ClassField};
use parser::{VoidParser, Error};

pub use self::field::Parser as FieldParser;
pub use self::method::Parser as ClassMethodParser;

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Class<'a>),
    Void(Class<'a>, VoidParser),
    Method(Class<'a>, ClassMethodParser<'a>),
    Finished(Class<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, Error> {
        if let XmlEvent::StartElement { name: ref element, ref attributes, .. } = *event {
            if element.local_name != "class" {
                return Err(Error::ExpectedElementStart("class".into()));
            }

            let (mut name, mut index) = (None, None);
            for attribute in attributes {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "index" => index = index.or_else(|| Some(attribute.value.clone())),
                    _ => (),
                }
            }

            let name = try!(name.ok_or_else(|| {
                Error::ExpectedAttribute("method".into(), "name".into())
            }));
            let index = try!(index.ok_or_else(|| {
                Error::ExpectedAttribute("field".into(), "index".into())
            }));

            Ok(Parser::Idle(Class {
                name: name.into(),
                fields: vec![],
                index: index.into(),
                methods: vec![],
            }))
        } else {
            Err(Error::ExpectedElementStart("class".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            Parser::Idle(mut class) => {
                match *event {
                    XmlEvent::StartElement { name: ref el, ref attributes, .. } if el.local_name == "field" => {
                        trace!(" > Field {} ", el.local_name);

                        let (mut name, mut domain) = (None, None);
                        for attribute in attributes.iter() {
                            match attribute.name.local_name.as_ref() {
                                "name" => name = name.or_else(|| Some(attribute.value.clone())),
                                "domain"|"type" => domain = domain.or_else(|| Some(attribute.value.clone())),
                                name => trace!("Unknown class field attribute: {}", name),
                            };
                        }

                        let name = try!(name.ok_or_else(|| {
                            Error::ExpectedAttribute("field".into(), "name".into())
                        }));
                        let domain = try!(domain.ok_or_else(|| {
                            println!("{:?}", class);
                            Error::ExpectedAttribute("field".into(), "domain".into())
                        }));

                        let field = ClassField { name: name.into(), domain: domain.into() };
                        class.fields.push(field);

                        Parser::Void(class, VoidParser::new())
                    },
                    XmlEvent::StartElement { name: ref el, .. } if el.local_name == "method" => {
                        trace!(" > Method {} ", el.local_name);
                        Parser::Method(class, try!(ClassMethodParser::from_xml_event(&event)))
                    },
                    XmlEvent::StartElement { name: ref el, .. } => {
                        trace!(" > Ignored Element {} ", el.local_name);
                        Parser::Void(class, VoidParser::new())
                    },
                    XmlEvent::EndElement { .. } => Parser::Finished(class),
                    _ => Parser::Idle(class),
                }
            },
            Parser::Void(class, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(class),
                    parser => Parser::Void(class, parser),
                }
            },
            Parser::Method(mut class, parser) => {
                match try!(parser.parse(event)) {
                    ClassMethodParser::Finished(method) => {
                        class.methods.push(method);
                        Parser::Idle(class)
                    },
                    parser => Parser::Method(class, parser),
                }
            },
            Parser::Finished(_) => return Err(Error::ExpectedEnd),
        })
    }
}
