
use std::borrow::Cow;
use std::slice::Iter;
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

    pub fn methods(&'a self) -> Methods<'a> {
        Methods(self.methods.iter())
    }
}

pub struct Methods<'a>(Iter<'a, Method<'a>>);

impl<'a> Iterator for Methods<'a> {
    type Item = &'a Method<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
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

            let name = try!(name.ok_or(ParseError::ExpectedAttribute("method".into(), "name".into())));
            let index = try!(index.ok_or(ParseError::ExpectedAttribute("field".into(), "index".into())));
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
                    XmlEvent::StartElement { name: ref element, .. } if element.local_name ==
                        "method" => {
                        trace!(" > Method {} ", element.local_name);
                        Parser::Method(class, try!(MethodParser::from_xml_event(&event)))
                    }
                    XmlEvent::StartElement { name: ref element, .. } => {
                        trace!(" > Ignored Element {} ", element.local_name);
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