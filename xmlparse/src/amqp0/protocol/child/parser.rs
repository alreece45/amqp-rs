
use xml::reader::XmlEvent;

use amqp0::ParseError;
use super::{Child, ChainedParser};

#[derive(Debug)]
pub enum Parser<'a> {
    Parsing(ChainedParser<'a>),
    Finished(Option<Child<'a>>),
}

impl<'a> Parser<'a> {
    pub fn new(parser: ChainedParser<'a>) -> Self {
        Parser::Parsing(parser.into())
    }

    pub fn parse(self, event: &XmlEvent) -> Result<Self, ParseError> {
        match self {
            Parser::Parsing(parser) => {
                let parser = try!(parser.parse(event));
                Ok(match parser.child() {
                    Ok(child) => Parser::Finished(child.into()),
                    Err(parser) => Parser::Parsing(parser),
                })
            }
            _ => Err(ParseError::ExpectedEnd),
        }
    }
}
