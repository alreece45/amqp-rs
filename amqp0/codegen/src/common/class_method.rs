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

use common::{self, DomainMapper};

pub type ClassMethodField = common::Field<specs::ClassMethodField>;

#[derive(Debug, Clone)]
pub struct ClassMethod {
    method: &'static specs::ClassMethod,
    fields: Vec<ClassMethodField>,
    constant_case: String,
    pascal_case: String,
    snake_case: String,
    has_lifetimes: bool,
}

impl ClassMethod {
    pub fn new(spec: &'static specs::Spec, method: &'static specs::ClassMethod) -> Self {
        let domain_mapper = DomainMapper::from_spec(spec);
        let fields = method.fields().iter()
            .cloned()
            .map(|field| {
                let domain = domain_mapper.map(field.domain());
                ClassMethodField::from_amqp0_field(field, domain)
            })
            .collect::<Vec<_>>();

        let has_lifetimes = fields.iter()
            .filter(|field| !field.is_reserved())
            .any(|field| !field.ty().is_copy());

        let constant_case = method.name().to_constant_case();
        let pascal_case = method.name().to_pascal_case();
        let snake_case = method.name().to_snake_case();

        ClassMethod {
            method: method,
            fields: fields,
            constant_case: constant_case,
            pascal_case: pascal_case,
            snake_case: snake_case,
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

impl Deref for ClassMethod {
    type Target = specs::ClassMethod;

    fn deref(&self) -> &Self::Target {
        self.method
    }
}