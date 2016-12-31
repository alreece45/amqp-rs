// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;

use inflections::Inflect;
use specs;

use common;
use common::domain::{Domain, DomainMapper};

pub type ClassMethodField<'a> = common::Field<'a, specs::ClassMethodField>;

pub struct ClassMethod<'a> {
    method: &'a specs::ClassMethod,
    fields: Vec<ClassMethodField<'a>>,
    constant_case: String,
    pascal_case: String,
    snake_case: String,
    has_lifetimes: bool,
}

impl<'a> ClassMethod<'a> {
    pub fn new(method: &'a specs::ClassMethod, domain_mapper: &DomainMapper) -> Self {
        let fields = method.fields().iter()
            .map(|field| {
                let domain = Domain::new(domain_mapper.map(field.domain()));
                ClassMethodField::from_amqp0_field(field, domain)
            })
            .collect::<Vec<_>>();

        let has_lifetimes = fields.iter()
            .map(|f| !f.is_reserved() && !f.ty().is_copy())
            .any(|is_copy| is_copy);

        ClassMethod {
            method: method,
            fields: fields,
            constant_case: method.name().to_constant_case(),
            pascal_case: method.name().to_pascal_case(),
            snake_case: method.name().to_snake_case(),
            has_lifetimes: has_lifetimes
        }
    }

    pub fn fields(&self) -> &[ClassMethodField] {
        &self.fields
    }

    pub fn constant_case(&self) -> &str {
        &self.constant_case
    }

    pub fn snake_case(&self) -> &str {
        &self.snake_case
    }

    pub fn pascal_case(&self) -> &str {
        &self.pascal_case
    }

    pub fn has_lifetimes(&self) -> bool {
        self.has_lifetimes
    }
}

impl<'a> Deref for ClassMethod<'a> {
    type Target = specs::ClassMethod;

    fn deref(&self) -> &Self::Target {
        self.method
    }
}