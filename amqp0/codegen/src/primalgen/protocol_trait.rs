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

pub struct ProtocolTraitWriter<'a> {
    specs: &'a Specs<'a>,
}

impl<'a> WriteRust for ProtocolTraitWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(
            writer,
            "pub trait Protocol<'a> {{\n\
                type Frame: 'a;\n\
            \n\
                fn protocol_header() -> &'static [u8];\n"
        ));

        for method in self.specs.methods() {
            try!(writeln!(
                writer,
                "fn {class}_{snake_method}() -> <Self as ::method::{class}::{pascal_method}Method{lifetimes}>::Payload\n\
                    where Self: ::method::{class}::{pascal_method}Method{lifetimes}\n\
                {{\n\
                    Default::default()\n\
                }}",
                lifetimes = if method.has_lifetimes() { "<'a>" } else { "" },
                class = method.class_name().to_snake_case(),
                snake_method = method.method_name().to_snake_case(),
                pascal_method = method.method_name().to_pascal_case(),
            ))
        }

        try!(writeln!(writer, "}} // pub trait Protocol<'a>"));

        Ok(())
    }
}

impl<'a> ProtocolTraitWriter<'a> {
    pub fn new(specs: &'a Specs<'a>) -> Self {
        ProtocolTraitWriter {
            specs: specs
        }
    }
}