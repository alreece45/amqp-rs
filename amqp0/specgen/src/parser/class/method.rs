// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;

use ClassMethod;
use parser::{Error, FieldParser, VoidParser};

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(ClassMethod<'a>),
    Void(ClassMethod<'a>, VoidParser),
    Field(ClassMethod<'a>, FieldParser<'a>),
    Finished(ClassMethod<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, Error> {
        if let XmlEvent::StartElement { name: ref e, ref attributes, .. } = *event {
            if e.local_name != "method" {
                return Err(Error::ExpectedElementStart("method".into()));
            }

            let (mut name, mut index) = (None, None);
            let (mut is_synchronous, mut has_content) = (false, false);

            for attribute in attributes {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "index" => index = index.or_else(|| Some(attribute.value.clone())),
                    "synchronous" => is_synchronous = attribute.value == "1",
                    "content" => has_content = attribute.value == "1",
                    _ => (),
                }
            }

            let name = try!(name.ok_or_else(|| {
                Error::ExpectedAttribute("method".into(), "name".into())
            }));
            let index = try!(index.ok_or_else(|| {
                Error::ExpectedAttribute("field".into(), "index".into())
            }));
            Ok(Parser::Idle(ClassMethod::new(name, index, is_synchronous, has_content)))
        } else {
            Err(Error::ExpectedElementStart("method".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            Parser::Idle(mut method) => {
                match *event {
                    XmlEvent::StartElement { name: ref e, .. } if e.local_name == "field" => {
                        Parser::Field(method, try!(FieldParser::from_xml_event(&event)))
                    },
                    XmlEvent::StartElement { name: ref e, ref attributes, .. } => {
                        match e.local_name.as_str() {
                            "response" => {
                                trace!("Method Response");
                                let name = attributes.iter().find(|a| a.name.local_name == "name");
                                if let Some(attribute) = name {
                                    method.response = Some(attribute.value.clone().into());
                                }
                            },
                            "chassis" => {
                                trace!("Method Chassis");
                                let (name, implement) = {
                                    let (mut name, mut implement) = (None, None);
                                    for attribute in attributes {
                                        match attribute.name.local_name.as_str() {
                                            "name" => name = Some(attribute.value.clone()),
                                            "implement" => implement = Some(attribute.value.clone()),
                                            _ => (),
                                        }

                                        if name.is_some() && implement.is_some() {
                                            break;
                                        }
                                    }
                                    (name, implement)
                                };

                                if let (Some(name), Some(implement)) = (name, implement) {
                                    method.chassis.insert(name.to_string(), implement.to_string());
                                }
                            },
                            _ => trace!("Method ignoring e: {}", e.local_name),
                        }
                        Parser::Void(method, VoidParser::new())
                    },
                    XmlEvent::EndElement { .. } => Parser::Finished(method),
                    _ => Parser::Idle(method),
                }
            },
            Parser::Void(method, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(method),
                    parser => Parser::Void(method, parser),
                }
            },
            Parser::Field(mut method, parser) => {
                match try!(parser.parse(event)) {
                    FieldParser::Finished(argument) => {
                        method.fields.push(argument);
                        Parser::Idle(method)
                    },
                    parser => Parser::Field(method, parser),
                }
            },
            Parser::Finished(_) => return Err(Error::ExpectedEnd),
        })
    }
}
