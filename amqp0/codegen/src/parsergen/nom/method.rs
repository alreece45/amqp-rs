// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;
use specs::{ClassMethod, ClassMethodField};

use CodeGenerator;
use common;
use common::domain::{Domain, DomainMapper};
use parsergen::FieldChunk;

type Field<'a> = common::Field<'a, ClassMethodField>;

pub struct MethodModuleWriter<'a> {
    module: &'a str,
    method: &'a ClassMethod,
    fields: Vec<Field<'a>>,
    has_lifetimes: bool,
}

impl<'a> MethodModuleWriter<'a> {
    pub fn new(module: &'a str, method: &'a ClassMethod, domain_mapper: &DomainMapper) -> Self {
        // fields
        let fields = method.fields().iter()
            .map(|field| {
                let ty = Domain::new(domain_mapper.map(field.domain()));
                Field::from_amqp0_field(field, ty)
            })
            .collect::<Vec<_>>();

        // has_lifetimes
        let has_lifetimes = fields.iter()
            .map(|f| !f.is_reserved() && !f.ty().is_copy())
            .any(|is_copy| is_copy);

        MethodModuleWriter {
            module: module,
            method: method,
            fields: fields,
            has_lifetimes: has_lifetimes,
        }
    }

    fn field_chunks(&self) -> Vec<FieldChunk> {
        let mut num_flags = 0;
        let field_chunks = Vec::new();
        self.fields.iter()
            .fold(field_chunks, |mut field_chunks, field| {
                let part_needs_adding = if let Domain::Bit = *field.ty() {
                    let needs_adding = field_chunks.last_mut()
                        .map(|flag| !flag.add_field(field))
                        .unwrap_or(true);

                    if needs_adding {
                        num_flags += 1;
                    }
                    needs_adding
                } else {
                    true
                };

                if part_needs_adding {
                    if let Domain::Bit = *field.ty() {
                        assert_ne!(0, num_flags);
                    }
                    field_chunks.push(FieldChunk::from_field(field, num_flags))
                }
                field_chunks
            })
    }
}

impl<'a> CodeGenerator for MethodModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let struct_name = self.method.name().to_pascal_case();
        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };
        let uses_pool = self.fields.iter().any(|f| f.ty().is_owned());

        try!(write!(writer, "\n\
    impl<'a> ::NomBytes<'a> for ::primitives::{}::{}{} {{\n\
        type Output = Self;\n\
        fn nom_bytes<'b, P>(input: &'a [u8], {}: &'b mut P) -> IResult<&'a [u8], Self>\n\
            where P: ::pool::ParserPool\n\
        {{\n\
            do_parse!(input,\n",
            self.module,
            struct_name,
            lifetimes,
            if uses_pool { "pool" } else { "_" }
        ));

        let field_chunks = self.field_chunks();

        for chunk in &field_chunks {
            let nom_parser = chunk.nom_parser();
            if let Some(name) = chunk.capture_name() {
                try!(writeln!(writer, "{}: {} >>", name, nom_parser));
            }
            else {
                try!(writeln!(writer, "{} >>", nom_parser));
            }
        }

        let arguments = field_chunks.iter()
            .flat_map(|p| p.arg_names())
            .collect::<Vec<_>>()
            .join(", ");

        try!(writeln!(writer, "(::primitives::{}::{}::new({}))", self.module, struct_name, arguments));

        try!(writeln!(writer, ") // do_parse!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(writer, "}} // impl NomBytes"));

        Ok(())
    }
}