
use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use xml::reader::{XmlEvent, EventReader, Error as XmlError};

mod protocol;

pub use self::protocol::ProtocolParser as Parser;
pub use self::protocol::{Class, Constant, Domain, Field, Method, Protocol};

#[derive(Debug)]
pub enum VoidParser {
    Parsing(usize),
    Finished,
}

impl VoidParser {
    pub fn new() -> VoidParser {
        VoidParser::Parsing(0)
    }
    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        Ok(match self {
            VoidParser::Parsing(depth) => {
                match *event {
                    XmlEvent::StartElement { .. } => VoidParser::Parsing(depth + 1),
                    XmlEvent::EndElement { .. } if depth == 0 => VoidParser::Finished,
                    XmlEvent::EndElement { .. } => VoidParser::Parsing(depth - 1),
                    _ => self,
                }
            }
            VoidParser::Finished => return Err(ParseError::ExpectedEnd),
        })
    }
}

impl Default for VoidParser {
    fn default() -> Self {
        VoidParser::new()
    }
}

pub fn parse<'a, P>(path: P) -> Result<Protocol<'a>, ParseError>
    where P: AsRef<Path>
{
    let path = path.as_ref();
    let file = try!(File::open(&path));
    let file = BufReader::new(file);

    let mut parser = Parser::new();

    for event in EventReader::new(file) {
        let event = try!(event);
        parser = try!(parser.parse(&event));
    }

    Ok(parser.into())
}

#[derive(Debug)]
pub enum ParseError {
    ExpectedAttribute(Cow<'static, str>, Cow<'static, str>),
    ExpectedElementStart(Cow<'static, str>),
    ExpectedAmqpRoot,
    // No more events are expected
    ExpectedEnd,
    Io(io::Error),
    Xml(XmlError),
}

impl<'a> From<io::Error> for ParseError {
    fn from(e: io::Error) -> Self {
        ParseError::Io(e)
    }
}

impl<'a> From<XmlError> for ParseError {
    fn from(e: XmlError) -> Self {
        ParseError::Xml(e)
    }
}
