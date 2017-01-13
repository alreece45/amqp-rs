// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::HashMap;
use std::io;
use inflections::Inflect;

use WriteRust;
use common::Spec;

pub struct SetterTraitDefinitionWriter<'a> {
    specs: &'a [Spec],
    method_name: &'a str,
    class_name: &'a str,
    field_names: &'a [&'a str]
}

impl<'a> SetterTraitDefinitionWriter<'a> {
    pub fn new(
        specs: &'a [Spec],
        class_name: &'a str,
        method_name: &'a str,
        field_names: &'a [&'a str]
    ) -> Self {
        SetterTraitDefinitionWriter {
            specs: specs,
            method_name: method_name,
            class_name: class_name,
            field_names: field_names,
        }
    }
}

impl<'a> WriteRust for SetterTraitDefinitionWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let method_pascal = self.method_name.to_pascal_case();
        if self.field_names.is_empty() {
            return Ok(())
        }

        let has_lifetimes = self.specs.iter()
            .filter_map(|spec| spec.class(self.class_name))
            .filter_map(|class| class.method(self.method_name))
            .any(|method| method.has_lifetimes());

        let lifetimes = if has_lifetimes { "<'a>" } else { "" };
        let section = format!("pub trait Set{}MethodFields{}", method_pascal, lifetimes);

        try!(writeln!(writer, "{} {{", section));
        for field_name in self.field_names {
            let tys = self.specs.iter()
                .filter_map(|spec| spec.class(self.class_name))
                .filter_map(|class| class.method(self.method_name))
                .flat_map(|method| method.fields())
                .filter(|field| field.var_name() == *field_name)
                .map(|field| (field.ty()))
                .map(|ty| (ty.owned_type(), ty))
                .collect::<HashMap<_, _>>();

            if tys.len() == 0 {
                unreachable!(
                    "No field types for field {}::{}.{}",
                    self.class_name,
                    self.method_name,
                    field_name
                );
            }

            if tys.len() > 1 {
                panic!(
                    "Conflicting types for {}::{}::{}",
                    self.class_name,
                    self.method_name,
                    field_name
                );
            }

            let var_name = field_name.to_snake_case();
            let ty = tys.values().next().unwrap();
            if ty.is_copy() {
                try!(writeln!(
                    writer,
                    "fn set_{0}(_: {1}) {{}}",
                    var_name,
                    ty.owned_type()
                ));
            }
            else {
                try!(writeln!(
                    writer,
                    "fn set_{0}<V>(_: V) where V: Into<{1}> {{}}",
                    var_name,
                    ty.cow_definition("a")
                ));
            }
        }
        try!(writeln!(writer, "}} // {}\n", section));

        Ok(())
    }
}