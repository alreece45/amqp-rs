// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use WriteRust;
use common::{Class, ClassMethod};

impl<'a> WriteRust for MethodPayloadImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.method.has_lifetimes() { ("<'a>") } else { ("") };
        try!(writeln!(writer, "\nimpl{1} ::ProtocolMethodPayload for {0}{1} {{", self.method.pascal_case(), lifetimes));

        try!(write!(writer, "fn class_id(&self) -> u16 {{"));
        try!(writeln!(writer, "{}", self.class.index()));
        try!(writeln!(writer, "}} // fn class_id()"));

        try!(writeln!(writer, "fn method_id(&self) -> u16 {{"));
        try!(writeln!(writer, "{}", self.method.index()));
        try!(writeln!(writer, "}} // fn method_id()"));

        try!(writeln!(writer, "}} // impl ::Payload for {}", self.method.pascal_case()));

        Ok(())
    }
}

pub struct MethodPayloadImplWriter<'a> {
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> MethodPayloadImplWriter<'a> {
    pub fn new(class: &'a Class, method: &'a ClassMethod) -> Self {
        let has_fields = method.fields().iter().any(|f| !f.is_reserved());

        MethodPayloadImplWriter {
            class: class,
            method: method,
        }
    }
}
