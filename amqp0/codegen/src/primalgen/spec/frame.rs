// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeMap;
use std::io;
use inflections::Inflect;
use amqp0::Constant;

pub struct FrameWriter<'a> {
    frame_types: &'a BTreeMap<&'static str,  Constant>,
}

impl<'a> FrameWriter<'a> {
    pub fn new(frame_types: &'a BTreeMap<&'static str, Constant>) -> Self {
        FrameWriter {
            frame_types: frame_types,
        }
    }

    pub fn write_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.frame_types.    len() == 0 {
            return Ok(())
        }

        try!(writeln!(writer, "enum Frame {{"));
        for frame_type in self.frame_types.keys() {
            let pascal_case = match frame_type.starts_with("frame-") {
                true => (&frame_type[6..]).to_pascal_case(),
                false => frame_type.to_pascal_case(),
            };
            let args = match pascal_case {
                "Method" => "Method",
                "Header" => "Header"
            };
            try!(writeln!(writer, "{},", pascal_case));
        }
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}