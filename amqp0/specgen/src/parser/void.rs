// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use xml::reader::XmlEvent;
use super::Error;

#[derive(Debug)]
pub enum VoidParser {
    Parsing(usize),
    Finished,
}

impl VoidParser {
    pub fn new() -> VoidParser {
        VoidParser::Parsing(0)
    }
    pub fn parse(self, event: &XmlEvent) -> Result<Self, Error> {
        Ok(match self {
            VoidParser::Parsing(depth) => {
                match *event {
                    XmlEvent::StartElement { .. } => VoidParser::Parsing(depth + 1),
                    XmlEvent::EndElement { .. } if depth == 0 => VoidParser::Finished,
                    XmlEvent::EndElement { .. } => VoidParser::Parsing(depth - 1),
                    _ => self,
                }
            },
            VoidParser::Finished => return Err(Error::ExpectedEnd),
        })
    }
}

impl Default for VoidParser {
    fn default() -> Self {
        VoidParser::new()
    }
}
