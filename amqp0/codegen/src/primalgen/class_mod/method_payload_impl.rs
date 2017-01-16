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
use inflections::Inflect;

impl<'a> WriteRust for MethodPayloadImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writeln!(
            writer,
            "\nimpl{lifetimes} ::ProtocolMethodPayload for {method_pascal}{lifetimes} {{\n\
                fn class(&self) -> ::Class {{ ::Class::{class_pascal} }}\n\
                fn class_id(&self) -> u16 {{ {class_index} }}\n\
                fn class_name(&self) -> &'static str {{ \"{class_kebeb}\" }}\n\
                fn method_id(&self) -> u16 {{ {method_index} }}\n\
                fn method_name(&self) -> &'static str {{ \"{method_kebeb}\" }}\n\
            }} // impl ::ProtocolMethodPayload for {method_pascal}{lifetimes}",
            lifetimes = if self.method.has_lifetimes() { ("<'a>") } else { ("") },
            class_pascal = self.class.pascal_case(),
            class_kebeb = self.class.name().to_kebab_case(),
            class_index = self.class.index(),
            method_pascal = self.method.pascal_case(),
            method_kebeb = self.method.name().to_kebab_case(),
            method_index = self.method.index(),
        )
    }
}

pub struct MethodPayloadImplWriter<'a> {
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> MethodPayloadImplWriter<'a> {
    pub fn new(class: &'a Class, method: &'a ClassMethod) -> Self {
        MethodPayloadImplWriter {
            class: class,
            method: method,
        }
    }
}
