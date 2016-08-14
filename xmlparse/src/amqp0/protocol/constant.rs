
use std::borrow::{Borrow, Cow};
use xml::reader::XmlEvent;

use amqp0::{ParseError, VoidParser};

#[derive(Debug)]
pub struct Constant<'a> {
    name: Cow<'a, str>,
    value: Cow<'a, str>,
    class: Option<Cow<'a, str>>,
}

impl<'a> Constant<'a> {
    pub fn name(&self) -> &str {
        self.name.borrow()
    }

    pub fn value(&self) -> &str {
        self.value.borrow()
    }

    pub fn class(&self) -> Option<&str> {
        self.class.as_ref().map(|c| c.borrow())
    }
}

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Constant<'a>),
    Void(Constant<'a>, VoidParser),
    Finished(Constant<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, ParseError> {
        if let XmlEvent::StartElement { name: ref element, ref attributes, .. } = *event {
            if element.local_name != "constant" {
                return Err(ParseError::ExpectedElementStart("constant".into()));
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

            let name = try!(name.ok_or(ParseError::ExpectedAttribute("constant".into(), "name".into())));
            let value = try!(value.ok_or(ParseError::ExpectedAttribute("constant".into(), "value".into())));

            Ok(Parser::Idle(Constant {
                name: name.into(),
                value: value.into(),
                class: class.map(|c| c.into()),
            }))
        } else {
            Err(ParseError::ExpectedElementStart("constant".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok (match self {
            Parser::Idle(argument) => {
                match *event {
                    XmlEvent::StartElement { .. } => {
                        Parser::Void(argument, VoidParser::new())
                    }
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