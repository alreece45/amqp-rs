// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;

use amqp0::{Assertion, ClassMethodField};
use amqp0::parser::Error;
use parser::VoidParser;

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(ClassMethodField<'a>),
    Enum(ClassMethodField<'a>, EnumParser),
    Void(ClassMethodField<'a>, VoidParser),
    Finished(ClassMethodField<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, Error> {
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
                Error::ExpectedAttribute("field".into(), "name".into())
            }));
            let domain = try!(domain.ok_or_else(|| {
                Error::ExpectedAttribute("field".into(), "domain".into())
            }));

            Ok(Parser::Idle(ClassMethodField {
                name: name.into(),
                domain: domain.into(),
                assertions: Vec::new(),
                is_reserved: is_reserved,
            }))
        } else {
            Err(Error::ExpectedElementStart("field".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            Parser::Idle(mut argument) => {
                match *event {
                    XmlEvent::StartElement { ref name, ref attributes, .. } if name.local_name == "assert" => {
                        let check = attributes.iter()
                            .find(|a| a.name.local_name == "check");
                        let check = check.map(|a| a.value.as_str());

                        if let Some("enum") = check {
                            Parser::Enum(argument, try!(EnumParser::from_xml_event(event)))
                        } else {
                            match check {
                                Some("notnull") => argument.assertions.push(Assertion::NotNull),
                                Some("length") => {
                                    let value = attributes.iter()
                                        .find(|a| a.name.local_name == "value")
                                        .map(|v| v.value.parse());

                                    match value {
                                        Some(Ok(len)) => argument.assertions.push(Assertion::Length(len)),
                                        Some(Err(e)) => warn!("Error parsing le value: {}", e),
                                        None => warn!("value missing for le assertion"),
                                    }
                                },
                                Some("regexp") => {
                                    let value = attributes.iter().find(|a| a.name.local_name == "value");
                                    if let Some(pattern) = value {
                                        argument.assertions.push(Assertion::Regexp(pattern.value.clone()));
                                    } else {
                                        warn!("value missing for regexp assertion");
                                    }
                                },
                                Some("null") => argument.assertions.push(Assertion::Null),
                                Some("le") => {
                                    let (method, field) = {
                                        let (mut method, mut field) = (None, None);
                                        for attribute in attributes {
                                            match attribute.name.local_name.as_str() {
                                                "method" => method = Some(attribute.value.clone()),
                                                "field" => field = Some(attribute.value.replace(" ", "-")),
                                                _ => (),
                                            }

                                            if method.is_some() && field.is_some() {
                                                break;
                                            }
                                        }
                                        (method, field)
                                    };

                                    let (method, field) = (
                                        method.as_ref().map(|m| &**m),
                                        field.as_ref().map(|f| &**f)
                                    );
                                    match (method, field) {
                                        (Some("tune"), Some("channel-max")) => {
                                            argument.assertions.push(Assertion::ChannelMax);
                                        },
                                        (method, field) => {
                                            warn!("Unknown le assertion: {:?} {:?}", method, field)
                                        },
                                    }
                                },
                                Some("ne") => {
                                    let value = attributes.iter().find(|a| a.name.local_name == "value");

                                    if let Some("0") = value.map(|v| v.value.as_str()) {
                                        argument.assertions.push(Assertion::NotZero);
                                    } else {
                                        warn!("Unknown ne assertion: {:?}", value);
                                    }
                                },
                                Some("syntax") => {
                                    let rule = attributes.iter().find(|a| a.name.local_name == "rule");
                                    if let Some(rule) = rule {
                                        let new_rule = rule.value.clone();
                                        argument.assertions.push(Assertion::Syntax(new_rule));
                                    } else {
                                        warn!("rule missing for syntax assertion");
                                    }
                                },
                                _ => (),
                            }
                            Parser::Void(argument, VoidParser::new())
                        }
                    }
                    XmlEvent::StartElement { .. } => {
                        Parser::Void(argument, VoidParser::new())
                    },
                    XmlEvent::EndElement { .. } => Parser::Finished(argument),
                    _ => Parser::Idle(argument),
                }
            },
            Parser::Enum(mut argument, parser) => {
                match try!(parser.parse(event)) {
                    EnumParser::Finished(enums) => {
                        argument.assertions.push(Assertion::Enum(enums));;
                        Parser::Idle(argument)
                    },
                    parser => Parser::Enum(argument, parser),
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

#[derive(Debug)]
pub enum EnumParser {
    Idle(Vec<String>),
    Void(Vec<String>, VoidParser),
    Finished(Vec<String>),
}

impl EnumParser {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, Error> {
        if let XmlEvent::StartElement { .. } = *event {
            Ok(EnumParser::Idle(vec![]))
        } else {
            Err(Error::ExpectedElementStart("assert".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            EnumParser::Idle(mut values) => {
                match *event {
                    XmlEvent::StartElement { ref name, ref attributes, .. } => {
                        if name.local_name == "value" {
                            let name = attributes.iter().find(|a| a.name.local_name == "name");

                            if let Some(attribute) = name {
                                values.push(attribute.name.local_name.clone());
                            }
                        }
                        EnumParser::Void(values, VoidParser::new())
                    },
                    XmlEvent::EndElement { .. } => EnumParser::Finished(values),
                    _ => EnumParser::Idle(values),
                }
            },
            EnumParser::Void(values, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => EnumParser::Idle(values),
                    parser => EnumParser::Void(values, parser),
                }
            },
            EnumParser::Finished(_) => return Err(Error::ExpectedEnd),
        })
    }
}