// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::io;

use inflections::Inflect;
use amqp0::{self, ClassMethodField};

use {Domain, DomainMapper};

type Field<'a> = super::field::Field<'a, ClassMethodField>;

#[derive(Debug)]
enum Part<'a> {
    Field(&'a str, Option<&'a str>), // parser, var_name
    Flags(u8, Vec<bool>, Option<Cow<'a, str>>),   // flag number, num_bits, is_used
}

impl<'a> Part<'a> {
    pub fn from_field(field: &'a Field, flag_num: u8) -> Self {
        if field.is_reserved() {
            match *field.ty() {
                Domain::Bit => Part::Flags(flag_num, vec![!field.is_reserved()], None),
                _ => Part::Field(field.ty().nom_parser(), None),
            }
        }
        else {
            let name = field.var_name();
            match *field.ty() {
                Domain::Bit => Part::Flags(flag_num, vec![!field.is_reserved()], Some(name.into())),
                _ => Part::Field(field.ty().nom_parser(), Some(name)),
            }
        }
    }

    pub fn add_field(&mut self, field: &'a Field) -> bool {
        match *self {
            Part::Flags(flag_num, ref mut bits, ref mut name) if bits.len() <= 8 => {
                bits.push(!field.is_reserved());

                if bits.len() > 1 {
                    if let Some(Cow::Borrowed(_)) = *name {
                        mem::replace(name, Some(format!("flag{}", flag_num).into()));
                    }
                }
                true
            }
            _ => false,
        }
    }

    pub fn capture_name(&self) -> Option<&str> {
         match *self {
            Part::Flags(_, _, ref name) => name.as_ref().map(|n| n.as_ref()),
            Part::Field(_, name) => name,
        }
    }

    pub fn arg_names(&self) -> Vec<Cow<str>> {
        match *self {
            Part::Field(_, Some(ref name)) => vec![(*name).into()],
            Part::Flags(_, ref bits, Some(ref name)) => {
                if bits.len() > 1 {
                    bits.iter()
                        .filter(|f| **f)
                        .enumerate()
                        .map(|(bit, _): (usize, &bool)| -> Cow<str> { format!("{}.{}", name, bit).into() })
                        .collect()
                }
                else {
                    vec![Cow::Borrowed(&*name)]
                }
            },
            _ => vec![],
        }
    }

    pub fn nom_parser(&self) -> Cow<'a, str> {
        const BOOL_MAPPER: &'static str = "call!(::amqp0::nom::bool_bit)";
        match *self {
            Part::Field(parser, _) => parser.into(),
            Part::Flags(_, ref bits, _) => {
                if bits.len()> 1 {
                    let collectors = bits.iter()
                        .map(|_| BOOL_MAPPER)
                        .collect::<Vec<_>>()
                        .join(",\n");
                    format!("bits!(tuple!(\n{}\n))", collectors).into()
                }
                else {
                    format!("bits!({})", BOOL_MAPPER).into()
                }
            },
        }
    }
}

pub struct ModuleWriter<'a> {
    class: &'a amqp0::Class,
    method: &'a amqp0::ClassMethod,
    struct_name: String,
    fields: Vec<Field<'a>>,

    /// For non-copy parameters, allow conversion using Into<>
    /// the conversions require generic parameters: we generate
    /// and store the generic parameters for those properties here
    generic_types: HashMap<&'a str, String>,

    /// some values ("bit") are grouped together into one byte. To extract
    /// them, we need to extract and parse the individual bits. Before
    /// parsing, we group those those bits up into "parts", and then generate
    /// the parser code for each individual part
    // parts: Vec<Part<'a>>,

    has_lifetimes: bool,
}

impl<'a> ModuleWriter<'a> {
    pub fn new(
        class: &'a amqp0::Class,
        method: &'a amqp0::ClassMethod,
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

        ModuleWriter {
            class: class,
            method: method,
            struct_name: method.name().to_pascal_case(),
            fields: fields,
            has_lifetimes: has_lifetimes,
            generic_types: generic_types,
        }
    }

    pub fn write_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(self.write_struct(writer));
        try!(self.write_inherent_impl(writer));
        try!(self.write_amqp0_payload_impl(writer));
        // try!(self.write_nom_bytes_impl(writer));

        Ok(())
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
            if field.ty().is_copy() {
                try!(writeln!(writer, "{},", field.ty().owned_type()));
            }
            else {
                try!(writeln!(writer, "::std::borrow::Cow<'a, {}>,", field.ty().borrowed_type()));
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
                    let ty = field.ty().borrowed_type();
                    try!(writeln!(writer, "{}: Into<::std::borrow::Cow<'a, {}>>,", label, ty));
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

            let borrow = if is_copy { "" } else { "&*" };
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


    pub fn write_nom_bytes_impl<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let parts: Vec<Part> = {
            let mut num_flags = 0;
            let parts: Vec<Part> = Vec::new();
            self.fields.iter()
                .fold(parts, |mut parts, field| {
                    let part_needs_adding = if let Domain::Bit = *field.ty() {
                        let needs_adding = parts.last_mut()
                            .map(|flag| !flag.add_field(&field))
                            .unwrap_or(true);

                        if needs_adding {
                            num_flags += 1;
                        }
                        needs_adding
                    }
                    else {
                        true
                    };

                    if part_needs_adding {
                        if let Domain::Bit = *field.ty() {
                            assert_ne!(0, num_flags);
                        }
                        parts.push(Part::from_field(field, num_flags))
                    }
                    parts
                })
        };

        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "use nom::IResult;"));
        try!(writeln!(writer, "use nom::{{be_u8, be_u16, be_u32, be_u64}};\n"));
        try!(writeln!(writer, "impl<'a> ::amqp0::nom::NomBytes<'a> for {}{} {{", self.struct_name, lifetimes));
        try!(writeln!(writer, "fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>"));
        try!(writeln!(writer, "    where P: ::amqp0::nom::ParserPool"));
        try!(writeln!(writer, "{{"));
        try!(writeln!(writer, "do_parse!(input, "));

        for part in &parts {
            let nom_parser = part.nom_parser();
            if let Some(name) = part.capture_name() {
                try!(writeln!(writer, "{}: {} >>", name, nom_parser));
            }
            else {
                try!(writeln!(writer, "{} >>", nom_parser));
            }
        }

        let arguments = parts.iter()
            .flat_map(|p| p.arg_names())
            .collect::<Vec<_>>()
            .join(", ");

        try!(writeln!(writer, "({}::new({}))", self.struct_name, arguments));

        try!(writeln!(writer, ")")); // do_parse!
        try!(writeln!(writer, "}}")); // fn nom_bytes
        try!(writeln!(writer, "}}")); // impl NomBytes

        Ok(())
    }

}
