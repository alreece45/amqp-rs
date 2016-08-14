// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use xml::reader::XmlEvent;

use amqp0::{ParseError, VoidParser};
use super::{Method, MethodParser};

#[derive(Debug)]
pub struct Class<'a> {
    name: Cow<'a, str>,
    index: Cow<'a, str>,
    methods: Vec<Method<'a>>,
}

impl<'a> Class<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn index(&self) -> &str {
        &self.index
    }

    pub fn methods(&'a self) -> &Vec<Method<'a>> {
        &self.methods
    }
}

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Class<'a>),
    Void(Class<'a>, VoidParser),
    Method(Class<'a>, MethodParser<'a>),
    Finished(Class<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, ParseError> {
        if let XmlEvent::StartElement { name: ref element, ref attributes, .. } = *event {
            if element.local_name != "class" {
                return Err(ParseError::ExpectedElementStart("class".into()));
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
                ParseError::ExpectedAttribute("method".into(), "name".into())
            }));
            let index = try!(index.ok_or_else(|| {
                ParseError::ExpectedAttribute("field".into(), "index".into())
            }));

            Ok(Parser::Idle(Class {
                name: name.into(),
                index: index.into(),
                methods: vec![],
            }))
        } else {
            Err(ParseError::ExpectedElementStart("class".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            Parser::Idle(class) => {
                match *event {
                    XmlEvent::StartElement { name: ref el, .. } if el.local_name == "method" => {
                        trace!(" > Method {} ", el.local_name);
                        Parser::Method(class, try!(MethodParser::from_xml_event(&event)))
                    }
                    XmlEvent::StartElement { name: ref el, .. } => {
                        trace!(" > Ignored Element {} ", el.local_name);
                        Parser::Void(class, VoidParser::new())
                    }
                    XmlEvent::EndElement { .. } => Parser::Finished(class),
                    _ => Parser::Idle(class),
                }
            }
            Parser::Void(class, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(class),
                    parser => Parser::Void(class, parser),
                }
            }
            Parser::Method(mut class, parser) => {
                match try!(parser.parse(event)) {
                    MethodParser::Finished(method) => {
                        class.methods.push(method);
                        Parser::Idle(class)
                    }
                    parser => Parser::Method(class, parser),
                }
            }
            Parser::Finished(_) => return Err(ParseError::ExpectedEnd),
        })
    }
}
