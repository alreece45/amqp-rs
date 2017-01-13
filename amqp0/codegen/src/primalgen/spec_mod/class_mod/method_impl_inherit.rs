// Copyright 2016-7 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{HashMap, HashSet};
use std::io;

use WriteRust;
use common::ClassMethod;

impl<'a> WriteRust for InheritMethodImplWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" };

        try!(writeln!(writer, "\nimpl{1} {0}{1} {{", self.method.pascal_case(), lifetimes));
        try!(self.write_constructor(writer));
        try!(self.write_getters(writer));
        try!(writeln!(writer, "}} // impl{1} {0}{1}", self.method.pascal_case(), lifetimes));

        Ok(())
    }
}

pub struct InheritMethodImplWriter<'a> {
    method: &'a ClassMethod,

    /// For non-copy parameters, allow conversion using Into<>.
    /// Using Into requires defining generic parameters.
    /// We store the names of the generic parameters here
    generic_types: HashMap<&'a str, String>,
    has_fields: bool,
}

impl<'a> InheritMethodImplWriter<'a> {
    pub fn new(method: &'a ClassMethod) -> Self {
        let has_fields = method.fields().iter().any(|f| !f.is_reserved());
        let generic_types = {
            let mut labels = HashSet::new();
            method.fields().iter()
                .filter(|f| !f.is_reserved() && !f.ty().is_copy())
                .map(|field| {
                    let name = field.var_name();
                    let first_char = field.var_name().chars().next().unwrap();
                    let prefix = first_char.to_uppercase().collect::<String>();

                    let mut label: String = prefix.clone();
                    let mut suffix = 0;

                    while labels.contains(label.as_str()) {
                        label = format!("{}{}", prefix, suffix);
                        suffix += 1;
                    }

                    labels.insert(label.clone());
                    (name, label)
                })
                .collect()
        };

        InheritMethodImplWriter {
            method: method,
            generic_types: generic_types,
            has_fields: has_fields,
        }
    }

    pub fn write_constructor<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if !self.has_fields {
            try!(writeln!(writer, "pub fn new() -> Self {{"));
            try!(writeln!(writer, "{}", self.method.pascal_case()));
            try!(writeln!(writer, "}} // fn new()"));
            return Ok(());
        }

        try!(write!(writer, "pub fn new"));

        // generic arguments: <A, B, C>
        if !self.generic_types.is_empty() {
            let generics = self.method.fields().iter()
                .filter_map(|f| self.generic_types.get(f.var_name()))
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            try!(write!(writer, "<{}>", generics));
        }

        let arguments = self.method.fields().iter()
            .filter(|f| !f.is_reserved())
            .map(|field| {
                let ty = if let Some(generic) = self.generic_types.get(field.var_name()) {
                    generic
                } else {
                    field.ty().owned_type()
                };

                format!("{}: {}", field.var_name(), ty)
            }).collect::<Vec<_>>();

        // function arguments: (name1: ty1, name2: ty2)
        try!(write!(writer, "(\n{}\n) -> Self", arguments.join(",\n")));

        // generic conditions: where A: ..., B: ...
        if !self.generic_types.is_empty() {
            try!(write!(writer, "\n where "));
            for field in self.method.fields() {
                if let Some(label) = self.generic_types.get(field.var_name()) {
                    let ty = field.ty().cow_definition("a");
                    try!(writeln!(writer, "{}: Into<{}>,", label, ty));
                }
            }
        }

        try!(writeln!(writer, " {{"));

        // construction body
        try!(writeln!(writer, "{} {{", self.method.pascal_case()));
        for field in self.method.fields() {
            if field.is_reserved() {
                continue
            }

            let name = field.var_name();
            if self.generic_types.contains_key(field.var_name()) {
                try!(writeln!(writer, "{}: {}.into(),", name, name));
            }
                else {
                    try!(writeln!(writer, "{}: {},", name, name));
                }
        }
        try!(writeln!(writer, "}} // {}", self.method.pascal_case())); // struct creation
        try!(writeln!(writer, "}} // fn new()")); // constructor

        Ok(())
    }

    pub fn write_getters<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if !self.has_fields {
            return Ok(());
        }

        try!(writeln!(writer, "impl_properties! {{"));
        for field in self.method.fields() {
            if field.is_reserved() {
                continue;
            }
            let name = field.var_name();
            let ty = field.ty().borrowed_type();
            try!(match (field.ty().is_copy(), field.ty().is_owned()) {
                (true, _) => writeln!(writer, "({0}, set_{0}) -> {1},", name, ty),
                (_, true) => writeln!(writer, "({0}, {0}_mut, set_{0}) -> &{1},", name, ty),
                _ => writeln!(writer, "({0}, {0}_mut, set_{0}) -> Cow<{1}>,", name, ty)
            });
        }
        try!(writeln!(writer, "}} // impl_properties"));

        Ok(())
    }
}
