// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;

use CodeGenerator;
use common::{Spec, Class, ClassMethod, Domain};
use parsergen::FieldChunk;

pub struct MethodModuleWriter<'a> {
    spec: &'a Spec,
    class: &'a Class,
    method: &'a ClassMethod,
}

impl<'a> MethodModuleWriter<'a> {
    pub fn new(spec: &'a Spec, class: &'a Class, method: &'a ClassMethod) -> Self {
        MethodModuleWriter {
            spec: spec,
            class: class,
            method: method,
        }
    }

    fn field_chunks(&self) -> Vec<FieldChunk> {
        let mut num_flags = 0;
        let field_chunks = Vec::new();
        self.method.fields().iter()
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
        let struct_name = self.method.pascal_case();
        let lifetimes = if self.method.has_lifetimes() { "<'a>" } else { "" };
        let uses_pool = self.method.fields().iter().any(|f| f.ty().is_owned());

        try!(write!(writer, "\n\
    impl<'a> ::NomBytes<'a> for ::primitives::{}::{}::{}{} {{\n\
        type Output = Self;\n\
        fn nom_bytes<'b, P>(input: &'a [u8], {}: &'b mut P) -> IResult<&'a [u8], Self>\n\
            where P: ::pool::ParserPool\n\
        {{\n\
            do_parse!(input,",
            self.spec.mod_name(),
            self.class.snake_case(),
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

        try!(writeln!(writer, "(::primitives::{}::{}::{}::new({}))", self.spec.mod_name(), self.class.snake_case(), struct_name, arguments));

        try!(writeln!(writer, ") // do_parse!"));
        try!(writeln!(writer, "}} // fn nom_bytes"));
        try!(writeln!(writer, "}} // impl NomBytes"));

        Ok(())
    }
}