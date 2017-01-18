// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::Spec;

impl<'a> WriteRust for ProtocolImplementationWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writeln!(
            writer,
            "impl<'a> ::Protocol<'a> for {struct_name} {{\n\
                type Frame = {mod_name}::Frame<'a>;\n\
                fn protocol_header() -> &'static [u8] {{\n\
                    b\"AMQP\\x00\\x00\\x{minor:02x}\\x{revision:02x}\"\n\
                }} // fn protocol_header()\n\
            }} // impl ::Protocol<'a> for {struct_name}",

            struct_name = self.spec.pascal_case(),
            mod_name = self.spec.mod_name(),
            minor = self.spec.version().minor(),
            revision = self.spec.version().revision(),
        )
    }
}

pub struct ProtocolImplementationWriter<'a> {
    spec: &'a Spec,
}

impl<'a> ProtocolImplementationWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        ProtocolImplementationWriter {
            spec: spec,
        }
    }
}
