// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod setter_trait_def;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::io;
use inflections::Inflect;

use WriteRust;
use common::Spec;

use self::setter_trait_def::SetterTraitDefinitionWriter;

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

    fn class_method_fields(&'a self) -> BTreeMap<&'a str, BTreeMap<&'a str, Vec<&'a str>>> {
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
                            .collect::<BTreeSet<&'a str>>()
                            .into_iter()
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
        for (class, class_methods) in self.class_method_fields() {
            let class_snake = class.to_snake_case();
            try!(writeln!(writer, "\npub mod {} {{", class_snake));

            for (method, fields) in class_methods {
                let has_lifetimes = self.specs.iter()
                    .filter_map(|spec| spec.class(class))
                    .filter_map(|class| class.method(method))
                    .any(|method| method.has_lifetimes());

                let lifetimes = if has_lifetimes { "<'a>" } else { "" };
                let pascal_method = method.to_pascal_case();

                let section = format!("pub trait {}Method{}", pascal_method, lifetimes);
                try!(write!(writer, "{} {{\ntype Payload: Default", section));

                if fields.len() > 0 {
                    try!(write!(writer, " + Set{}MethodFields{}", pascal_method, lifetimes))
                }
                try!(writeln!(writer, ";\n}} // {}\n", section));

                let setter = SetterTraitDefinitionWriter::new(self.specs, class, method, &fields);
                try!(setter.write_rust_to(writer));
            }

            try!(writeln!(writer, "}} // mod {}", class_snake));
        }

        Ok(())
    }
}