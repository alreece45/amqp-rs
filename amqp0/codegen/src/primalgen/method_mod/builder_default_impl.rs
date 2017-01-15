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

pub struct DefaultImplWriter<'a> {
    method: &'a SpecMethod,
}

impl<'a> WriteRust for DefaultImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writeln!(
            writer, "\
            impl<T> Default for {method}Builder<T>\n\
                where T: ::Encodable + Default\n\
            {{\n\
                fn default() -> Self {{\n\
                    {method}Builder {{\n\
                        payload: Default::default()\n\
                    }}\n\
                }}\n\
            }} // impl Default for {method}Builder",
            method = self.method.method_name().to_pascal_case(),
        )
    }
}

impl<'a> DefaultImplWriter<'a> {
    pub fn new(method: &'a SpecMethod) -> Self {
        DefaultImplWriter {
            method: method,
        }
    }
}
