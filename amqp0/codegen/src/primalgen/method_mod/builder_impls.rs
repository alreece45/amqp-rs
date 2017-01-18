// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;

use common::SpecMethod;
use WriteRust;

pub struct BuilderImplsWriter<'a> {
    method: &'a SpecMethod,
}

impl<'a> WriteRust for BuilderImplsWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let pascal_method = self.method.method_name().to_pascal_case();
        let section = format!("impl<T> {}Builder<T>", pascal_method,);

        try!(writeln!(
            writer, "\
            {section}\n\
                where T: Default + ::Encodable\n\
            {{\n\
            \n\
                pub fn new() -> Self {{\n\
                    Default::default()\n\
                }}\n\
            }} // impl Builder (new)\n\
            \n\
            {section}\n\
                where T: ::Encodable\n\
            {{\n\
                pub fn build(self) -> T {{\n\
                    self.payload\n\
                }}\n\
            }} // {section} \n",
            section = section,
        ));

        Ok(())
    }
}

impl<'a> BuilderImplsWriter<'a> {
    pub fn new(method: &'a SpecMethod) -> Self {
        BuilderImplsWriter {
            method: method,
        }
    }
}
