// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::collections::BTreeSet;
use inflections::Inflect;

use common::{Specs, Fields, Field, Domain};
use WriteRust;

pub struct MessageModuleWriter<'a> {
    specs: &'a Specs<'a>,
}

impl<'a> MessageModuleWriter<'a> {
    pub fn new(specs: &'a Specs<'a>) -> Self {
        MessageModuleWriter {
            specs: specs,
        }
    }
}

impl<'a> WriteRust for MessageModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let class_names = self.specs.iter()
            .flat_map(|spec| spec.classes())
            .filter(|class| class.methods().any(|method| method.has_content()))
            .map(|class| class.name())
            .collect::<BTreeSet<_>>();

        for class_name in class_names {
            let trait_name = class_name.to_pascal_case();
            try!(writeln!(writer, "\ntrait {}<'a> {{", trait_name));


            let has_content = self.specs.iter()
                .filter_map(|spec| spec.class(class_name))
                .flat_map(|class| class.methods())
                .any(|method| method.has_content());

            let body_field = if has_content {
                Some(Field::new("body", Domain::Content, false))
            } else {
                None
            };

            let fields = Fields::new(self.specs.iter()
                .filter_map(|spec| spec.class(class_name))
                .flat_map(|class| {
                    let method_fields = class.methods()
                        .filter(|method| method.has_content())
                        .flat_map(|method| method.fields());
                    class.fields().iter().chain(method_fields)
                })
                .chain(body_field.iter()));

            let fields = fields.iter()
                .map(|field| {
                    if !field.ty().is_option_type() {
                        return (field, false)
                    }

                    let var_name = field.var_name();
                    let is_optional = field.is_optional_property() || !self.specs.iter()
                        .filter_map(|spec| spec.class(class_name))
                        .flat_map(|class| class.methods())
                        .filter(|method| method.has_content())
                        .all(|method| method.field_by_var(var_name).is_some());

                    (field, is_optional)
                })
                .collect::<Vec<_>>();

            // Attempt to group the fields logically by doing multiple passes:
            //   "Mandatory" non-bits fields, "Mandatory" bit fields, optional non-bit fields,
            //   then finally, optional bit fields
            for pass in 0..4 {
                let mut is_header_written = false;
                let pass_header = match pass {
                    0 => "// Mandatory Fields (may be made optional in the future)",
                    1 => "\n// Mandatory Flags (may be made optional in the future)",
                    2 => "\n// Optional Fields",
                    3 => "\n// Optional Flags",
                    _ => unreachable!(),
                };
                for &(field, is_optional) in &fields {
                    let ty = field.ty();
                    match (pass, is_optional, ty) {
                        (0...1, true, _) => continue,
                        (0, false, &Domain::Bit) => continue,
                        (1, false, ty) if ty != &Domain::Bit => continue,
                        (2...3, false, _) => continue,
                        (2, true, &Domain::Bit) => continue,
                        (3, true, ty) if ty != &Domain::Bit => continue,
                        _ => (),
                    }

                    if fields.len() > 4 && !is_header_written {
                        is_header_written = true;
                        try!(writeln!(writer, "{}", pass_header));
                    }

                    let bool_prefix = if *ty == Domain::Bit { "is_" } else { "" };

                    if is_optional {
                        try!(writeln!(
                            writer,
                            "fn {}{}(&self) -> {} {{ None }}",
                            bool_prefix,
                            field.var_name(),
                            ty.option_type(),
                        ));
                    } else {
                        try!(writeln!(
                            writer,
                            "fn {}{}(&self) -> {}{};",
                            bool_prefix,
                            field.var_name(),
                            if ty.is_copy() { "" } else { "&" },
                            ty.borrowed_type(),
                        ));
                    }
                }
            }
            try!(writeln!(writer, "}} // trait {}", trait_name));
        }

        Ok(())
    }
}