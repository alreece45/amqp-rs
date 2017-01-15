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

pub struct BuilderStructWriter<'a> {
    method: &'a SpecMethod,
}

impl<'a> BuilderStructWriter<'a> {
    pub fn new(method: &'a SpecMethod) -> Self {
        BuilderStructWriter {
            method: method,
        }
    }
}

impl<'a> WriteRust for BuilderStructWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writeln!(
            writer,
            "pub struct {struct_name}Builder<T: ::Encodable> {{\n\
                payload: T,\n\
            }} // struct {struct_name}Builder \n\
            \n",
            struct_name = self.method.method_name().to_pascal_case(),
        )
    }
}