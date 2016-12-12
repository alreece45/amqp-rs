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

use CodeGenerator;
use specs::Constant;

pub struct FrameEnumWriter<'a> {
    frame_types: &'a BTreeMap<&'static str,  Constant>,
}

impl<'a> FrameEnumWriter<'a> {
    pub fn new(frame_types: &'a BTreeMap<&'static str, Constant>) -> Self {
        FrameEnumWriter {
            frame_types: frame_types,
        }
    }
}

impl<'a> CodeGenerator for FrameEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.frame_types.is_empty() {
            return Ok(())
        }

        try!(writeln!(writer, "pub enum Frame {{"));
        for frame_type in self.frame_types.keys() {
            let name_start = if frame_type.starts_with("frame-") { 6 } else { 0 };
            let pascal_case = (&frame_type[name_start..]).to_pascal_case();
            try!(writeln!(writer, "{},", pascal_case));
        }
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}