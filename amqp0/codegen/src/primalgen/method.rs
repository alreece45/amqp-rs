// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{HashMap, HashSet};
use std::io;

use inflections::Inflect;
use specs::{self, ClassMethodField};

use CodeGenerator;
use common;
use common::domain::{Domain, DomainMapper};

type Field<'a> = common::Field<'a, ClassMethodField>;

impl<'a> CodeGenerator for MethodModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(self.write_struct(writer));
        try!(self.write_inherent_impl(writer));
        try!(self.write_amqp0_payload_impl(writer));
        // try!(self.write_nom_bytes_impl(writer));

        Ok(())
    }
}

pub struct MethodModuleWriter<'a> {
    class: &'a specs::Class,
    method: &'a specs::ClassMethod,
    struct_name: String,
    fields: Vec<Field<'a>>,

    /// For non-copy parameters, allow conversion using Into<>.
    /// Using Into requires defining generic parameters.
    /// We store the names of the generic parameters here
    generic_types: HashMap<&'a str, String>,

    has_lifetimes: bool,
}

impl<'a> MethodModuleWriter<'a> {
    pub fn new(
        class: &'a specs::Class,
        method: &'a specs::ClassMethod,
        domain_mapper: &DomainMapper
    ) -> Self {
        let fields = method.fields().iter()
            .map(|field| {
                let ty = Domain::new(domain_mapper.map(field.domain()));
                Field::from_amqp0_field(field, ty)
            })
            .collect::<Vec<_>>();

        let generic_types = {
            let mut labels = HashSet::new();
            fields.iter()
                .filter(|f| !f.is_reserved() && !f.ty().is_copy())
                .map(|field| {
                    let name = field.name();
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

        let has_lifetimes = fields.iter()
            .map(|f| !f.is_reserved() && !f.ty().is_copy())
            .any(|is_copy| is_copy);

        MethodModuleWriter {
            class: class,
            method: method,
            struct_name: method.name().to_pascal_case(),
            fields: fields,
            has_lifetimes: has_lifetimes,
            generic_types: generic_types,
        }
    }

    #[doc(hidden)]
    pub fn struct_name(&self) -> &str {
        &*self.struct_name
    }

    #[doc(hidden)]
    pub fn has_lifetimes(&self) -> bool {
        self.has_lifetimes
    }

    pub fn write_struct<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.fields.is_empty() {
            try!(writeln!(writer, "pub struct {};", self.struct_name));
            return Ok(());
        }

        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "pub struct {}{} {{", self.struct_name, lifetimes));
        for field in &self.fields {
            if field.is_reserved() {
                continue
            }

            try!(write!(writer, "{}: ", field.var_name()));
            if field.ty().is_copy() || field.ty().is_owned() {
                try!(writeln!(writer, "{},", field.ty().owned_type()));
            }
            else {
                try!(writeln!(writer, "{},", field.ty().cow_definition("a")));
            }
        }
        try!(writeln!(writer, "}}"));
        Ok(())
    }

    pub fn write_inherent_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "impl{} {}{} {{", lifetimes, self.struct_name, lifetimes));
        try!(self.write_constructor(writer));
        try!(self.write_getters(writer));
        try!(writeln!(writer, "}}"));

        Ok(())
    }

    pub fn write_constructor<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.fields.is_empty() {
            try!(writeln!(writer, "pub fn new() -> Self {{"));
            try!(writeln!(writer, "{}", self.struct_name));
            try!(writeln!(writer, "}}"));
            return Ok(());
        }

        try!(write!(writer, "pub fn new"));

        // generic arguments: <A, B, C>
        if !self.generic_types.is_empty() {
            let generics = self.fields.iter()
                .filter_map(|f| self.generic_types.get(f.name()))
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            try!(write!(writer, "<{}>", generics));
        }

        let arguments = self.fields
            .iter()
            .filter(|f| !f.is_reserved())
            .map(|field| {
                let ty = if let Some(generic) = self.generic_types.get(field.name()) {
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
            for field in &self.fields {
                if let Some(label) = self.generic_types.get(field.name()) {
                    let ty = field.ty().cow_definition("a");
                    try!(writeln!(writer, "{}: Into<{}>,", label, ty));
                }
            }
        }

        try!(writeln!(writer, " {{"));

        // construction body
        try!(writeln!(writer, "{} {{", self.struct_name));
        for field in &self.fields {
            if field.is_reserved() {
                continue
            }

            let name = field.var_name();
            if self.generic_types.contains_key(field.name()) {
                try!(writeln!(writer, "{}: {}.into(),", name, name));
            }
            else {
                try!(writeln!(writer, "{}: {},", name, name));
            }
        }
        try!(writeln!(writer, "}}")); // struct creation
        try!(writeln!(writer, "}}")); // constructor

        Ok(())
    }

    pub fn write_getters<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.fields.is_empty() {
            return Ok(());
        }

        for field in &self.fields {
            if field.is_reserved() {
                continue;
            }
            let is_copy = field.ty().is_copy();
            let ty = field.ty().borrowed_type();
            let borrow = if is_copy { "" } else { "&" };
            try!(writeln!(writer, "pub fn {}(&self) -> {}{} {{", field.var_name(), borrow, ty));

            let borrow = match (is_copy, field.ty().is_owned()) {
                (true, _) => "",
                (_, true) => "&",
                _ => "&*",
            };
            try!(writeln!(writer, "{}self.{}", borrow, field.var_name()));
            try!(writeln!(writer, "}}"));
        }

        Ok(())
    }

    pub fn write_amqp0_payload_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "impl{} ::Payload for {}{} {{", lifetimes, self.struct_name, lifetimes));

        try!(writeln!(writer, "fn class_id(&self) -> u16 {{"));
        try!(writeln!(writer, "{}", self.class.index()));
        try!(writeln!(writer, "}}"));

        try!(writeln!(writer, "fn method_id(&self) -> u16 {{"));
        try!(writeln!(writer, "{}", self.method.index()));
        try!(writeln!(writer, "}}"));

        try!(writeln!(writer, "fn write_to<W>(&self, _: &mut W) -> ::std::io::Result<()>"));
        try!(writeln!(writer, "where W: ::std::io::Write"));
        try!(writeln!(writer, "{{\n::std::result::Result::Ok(())\n}}"));

        let static_size_bits = self.fields.iter()
            .map(|field| field.ty().num_bits_fixed())
            .fold(0, |sum, num_bits| sum + num_bits);

        let static_size = static_size_bits / 8 + if static_size_bits % 8 > 0 { 1 } else { 0 };
        let has_dynamic_field = self.fields.iter()
            .any(|field| field.ty().dynamic_bit_method().is_some());

        try!(writeln!(writer, "fn len(&self) -> usize {{"));
        if has_dynamic_field {
            try!(writeln!(writer, "["));
            try!(writeln!(writer, "{},", static_size));
            for field in &self.fields {
                if field.is_reserved() {
                    continue;
                }

                if let Some(method) = field.ty().dynamic_bit_method() {
                    try!(writeln!(writer, "self.{}.{}(),", field.var_name(), method));
                }
            }
            try!(writeln!(writer, "].iter().sum()"));
        } else {
            try!(writeln!(writer, "{}", static_size));
        }
        try!(writeln!(writer, "}}"));

        try!(writeln!(writer, "}}"));

        Ok(())
    }
}
