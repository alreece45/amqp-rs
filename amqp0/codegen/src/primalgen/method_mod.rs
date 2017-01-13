// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io;
use inflections::Inflect;

use WriteRust;
use common::Spec;

pub struct MethodModuleWriter<'a> {
    specs: &'a [Spec]
}

impl<'a> MethodModuleWriter<'a> {
    pub fn new(specs: &'a [Spec]) -> Self {
        MethodModuleWriter {
            specs: specs
        }
    }

    fn class_names(&self) -> HashSet<&'a str> {
        self.specs.iter()
            .flat_map(|spec| spec.classes())
            .map(|class| class.name())
            .collect()
    }

    fn class_methods(&'a self) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
        self.class_names().into_iter()
            .filter_map(|class_name| {
                let method_names = self.specs.iter()
                    .filter_map(|spec| spec.class(class_name))
                    .flat_map(|class| class.methods())
                    .map(|method| method.name())
                    .collect::<BTreeSet<_>>();

                if method_names.is_empty() {
                    None
                } else {
                    Some((class_name, method_names))
                }
            })
            .collect()
    }

    fn class_method_fields(&'a self) -> BTreeMap<&'a str, BTreeMap<&'a str, BTreeSet<&'a str>>> {
        self.class_methods().into_iter()
            .map(|(class_name, method_names)| {
                // class_name
                let class_methods = method_names.into_iter()
                    .map(|method_name| {
                        // method_name
                        let method_fields = self.specs.iter()
                            .filter_map(|spec| spec.class(class_name))
                            .filter_map(|class| class.method(method_name))
                            .flat_map(|method| method.fields())
                            .filter(|field| !field.is_reserved())
                            .map(|field| field.var_name())
                            .collect();
                        (method_name, method_fields)
                        // end method_name
                    })
                    .collect();
                (class_name, class_methods)
                // end class name
            })
            .collect()
    }
}

impl<'a> WriteRust for MethodModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        for (class_name, class_methods) in self.class_method_fields() {
            let class_snake = class_name.to_snake_case();
            try!(writeln!(writer, "\npub mod {} {{", class_snake));

            for (method_name, field_names) in class_methods {
                let method_pascal = method_name.to_pascal_case();
                if field_names.is_empty() {
                    try!(writeln!(writer, "pub trait Set{}MethodFields {{}}", method_pascal));
                    continue;
                }

                let has_lifetimes = self.specs.iter()
                    .filter_map(|spec| spec.class(class_name))
                    .filter_map(|class| class.method(method_name))
                    .any(|method| method.has_lifetimes());

                let lifetimes = if has_lifetimes { "<'a>" } else { "" };
                let section = format!("pub trait Set{}MethodFields{}", method_pascal, lifetimes);

                try!(writeln!(writer, "{} {{", section));
                for field_name in field_names {
                    let tys = self.specs.iter()
                        .filter_map(|spec| spec.class(class_name))
                        .filter_map(|class| class.method(method_name))
                        .flat_map(|method| method.fields())
                        .filter(|field| field.var_name() == field_name)
                        .map(|field| (field.ty()))
                        .map(|ty| (ty.owned_type(), ty))
                        .collect::<HashMap<_, _>>();

                    if tys.len() > 1 {
                        panic!(
                            "Conflicting types for {}::{}::{}",
                            class_name,
                            method_name,
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
            }
            try!(writeln!(writer, "}} // mod {}", class_snake));
        }

        Ok(())
    }
}