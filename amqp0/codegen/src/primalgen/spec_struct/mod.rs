// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod protocol_impl;

use std::io;

use WriteRust;
use common::Spec;

use self::protocol_impl::ProtocolImplementationWriter;

pub struct SpecStructWriter<'a> {
    spec: &'a Spec,
}

impl<'a> WriteRust for SpecStructWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(
            writer,
            "\n#[allow(non_camel_case_types)]\n\
             \n#[derive(Debug, Clone, PartialEq)]
             pub struct {};",
            self.spec.pascal_case()
        ));

        let protocol = ProtocolImplementationWriter::new(self.spec);
        try!(protocol.write_rust_to(writer));

        Ok(())
    }
}

impl<'a> SpecStructWriter<'a> {
    pub fn new(spec: &'a Spec) -> Self {
        SpecStructWriter {
            spec: spec,
        }
    }
}
