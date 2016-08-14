
use std::borrow::Cow;
use xml::reader::XmlEvent;

use amqp0::{ParseError, VoidParser};

#[derive(Debug)]
pub struct Domain<'a> {
    name: Cow<'a, str>,
    mapping: Cow<'a, str>,
}

impl<'a> Domain<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mapping(&self) -> &str {
        &self.mapping
    }
}

#[derive(Debug)]
pub enum Parser<'a> {
    Idle(Domain<'a>),
    Void(Domain<'a>, VoidParser),
    Finished(Domain<'a>),
}

impl<'a> Parser<'a> {
    pub fn from_xml_event(event: &XmlEvent) -> Result<Self, ParseError> {
        if let XmlEvent::StartElement { name: ref e, ref attributes, .. } = *event {
            if e.local_name != "domain" {
                return Err(ParseError::ExpectedElementStart("domain".into()));
            }

            let (mut name, mut mapping) = (None, None);
            for attribute in attributes.iter() {
                match attribute.name.local_name.as_ref() {
                    "name" => name = name.or_else(|| Some(attribute.value.clone())),
                    "type" => mapping = mapping.or_else(|| Some(attribute.value.clone())),
                    _ => (),
                };
            }

            let name = try!(name.ok_or(ParseError::ExpectedAttribute("field".into(), "name".into())));
            let mapping = try!(mapping.ok_or(ParseError::ExpectedAttribute("field".into(), "domain".into())));

            Ok(Parser::Idle(Domain {
                name: name.into(),
                mapping: mapping.into(),
            }))
        } else {
            Err(ParseError::ExpectedElementStart("field".into()))
        }
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            Parser::Idle(domain) => {
                match *event {
                    XmlEvent::StartElement { .. } => Parser::Void(domain, VoidParser::new()),
                    XmlEvent::EndElement { .. } => Parser::Finished(domain),
                    _ => Parser::Idle(domain),
                }
            }
            Parser::Void(domain, parser) => {
                match try!(parser.parse(event)) {
                    VoidParser::Finished => Parser::Idle(domain),
                    parser => Parser::Void(domain, parser),
                }
            }
            Parser::Finished(_) => return Err(ParseError::ExpectedEnd),
        })
    }
}
