// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;
use inflections::Inflect;

use common::{Specs, SpecMethod};
use WriteRust;

pub struct BuilderImplsWriter<'a> {
    method: &'a SpecMethod,
}

impl<'a> WriteRust for BuilderImplsWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let pascal_method = self.method.method_name().to_pascal_case();
        let section = format!(
            "impl<{lifetimes}T> {method}Builder<T>",
            lifetimes = if self.method.has_lifetimes() { "'a, " } else { "" },
            method = pascal_method,
        );

        try!(writeln!(
            writer, "\
            {0}\n\
                where T: Default + ::Encodable\n\
            {{\n\
            \n\
                pub fn new() -> Self {{\n\
                    Default::default()\n\
                }}\n\
            }} // impl Builder (new)\n\
            \n\
            {0}\n\
                where T: ::Encodable\n\
            {{\n\
                pub fn build(self) -> T {{\n\
                    self.payload\n\
                }}\n\
            }} // impl Builder (build)\n",
            section,
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
