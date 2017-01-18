// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;

use WriteRust;
use common::SpecMethod;

pub struct SetterTraitDefinitionWriter<'a> {
    method: &'a SpecMethod
}

impl<'a> SetterTraitDefinitionWriter<'a> {
    pub fn new(method: &'a SpecMethod) -> Self {
        SetterTraitDefinitionWriter {
            method: method
        }
    }
}

impl<'a> WriteRust for SetterTraitDefinitionWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if !self.method.has_usable_fields() {
            return Ok(())
        }

        let method_pascal = self.method.method_name().to_pascal_case();
        let lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" };
        let section = format!("pub trait Set{}MethodFields{}", method_pascal, lifetimes);

        try!(writeln!(writer, "{} {{", section));
        let fields = self.method.fields();

        if !fields.is_empty() {
            for (var_name, ty) in fields.vars() {
                if ty.is_copy() {
                    try!(writeln!(
                        writer,
                        "fn set_{0}(&mut self, _: {1}) {{}}",
                        var_name,
                        ty.owned_type()
                    ));
                } else {
                    try!(writeln!(
                        writer,
                        "fn set_{0}<V>(&mut self, _: V) where V: Into<{1}> {{}}",
                        var_name,
                        ty.cow_definition("a")
                    ));
                }
            }
        }

        try!(writeln!(writer, "}} // {}\n", section));

        Ok(())
    }
}