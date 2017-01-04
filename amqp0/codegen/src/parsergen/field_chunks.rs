// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use common::Field;

pub struct MethodModuleWriter<'a, F> {
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