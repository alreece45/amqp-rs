
use std::collections::HashMap;
use std::borrow::Cow;
use xml::reader::XmlEvent;

use amqp0::{ParseError, VoidParser};
use super::{Field, FieldParser};

#[derive(Debug)]
pub struct Method<'a> {
    name: Cow<'a, str>,
    index: Cow<'a, str>,
    chassis: HashMap<String, Cow<'a, str>>,
    response: Option<Cow<'a, str>>,
    fields: Vec<Field<'a>>,
    is_synchronous: bool,
}

impl<'a> Method<'a> {
    pub fn new<N, I>(name: N, index: I, is_synchronous: bool) -> Self
        where N: Into<Cow<'a, str>>,
              I: Into<Cow<'a, str>>
    {
        let name = name.into();
        trace!("Method: {}", &name);
        Method {
            name: name,
            index: index.into(),
            chassis: HashMap::new(),
            response: None,
            fields: vec![],
            is_synchronous: is_synchronous,
        }
    }
    pub fn index(&self) -> &str {
        &self.index
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn fields(&self) -> &Vec<Field<'a>> {
        &self.fields
    }
}

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Method<'a>),
    Void(Method<'a>, VoidParser),
    Field(Method<'a>, FieldParser<'a>),
    Finished(Method<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, ParseError> {
        if let XmlEvent::StartElement { name: ref e, ref attributes, .. } = *event {
            if e.local_name != "method" {
                return Err(ParseError::ExpectedElementStart("method".into()));
            }
            let (mut name, mut index, mut is_synchronous) = (None, None, false);
            for attribute in attributes {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "index" => index = index.or_else(|| Some(attribute.value.clone())),
                    "synchronous" => is_synchronous = attribute.value == "1",
                    _ => (),
                }
            }

            let name = try!(name.ok_or_else(|| {
                ParseError::ExpectedAttribute("method".into(), "name".into())
            }));
            let index = try!(index.ok_or_else(|| {
                ParseError::ExpectedAttribute("field".into(), "index".into())
            }));
            Ok(Parser::Idle(Method::new(name, index, is_synchronous)))
        } else {
            Err(ParseError::ExpectedElementStart("method".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            Parser::Idle(mut method) => {
                match *event {
                    XmlEvent::StartElement { name: ref e, .. } if e.local_name == "field" => {
                        Parser::Field(method, try!(FieldParser::from_xml_event(&event)))
                    },
                    XmlEvent::StartElement { name: ref e, ref attributes, .. } => {
                        if e.local_name == "response" {
                            if let Some(attribute) = attributes.iter()
                                .find(|a| a.name.local_name == "name") {
                                method.response = Some(attribute.value.clone().into());
                            }
                        }
                        trace!("Method ignoring e: {}", e.local_name);
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
                    }
                    parser => Parser::Field(method, parser),
                }
            },
            Parser::Finished(_) => return Err(ParseError::ExpectedEnd),
        })
    }
}
