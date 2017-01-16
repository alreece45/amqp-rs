// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;

use WriteRust;
use common::Specs;

pub struct ClassEnumWriter<'a> {
    specs: &'a Specs<'a>,
}

impl<'a> WriteRust for ClassEnumWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(writer, "pub enum Class {{"));
        for class_name in self.specs.class_names() {
            try!(writeln!(writer, "{},", class_name.to_pascal_case()));
        }
        try!(writeln!(writer, "}} // pub trait Class"));

        Ok(())
    }
}

impl<'a> ClassEnumWriter<'a> {
    pub fn new(specs: &'a Specs<'a>) -> Self {
        ClassEnumWriter {
            specs: specs
        }
    }
}