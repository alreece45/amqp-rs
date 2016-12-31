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
use primalgen::ClassMethod;

pub type ClassField<'a> = common::Field<'a, specs::ClassField>;

pub struct Class<'a> {
    class: &'a specs::Class,
    constant_case: String,
    snake_case: String,
    pascal_case: String,
    fields: Vec<ClassField<'a>>,
    methods: Vec<ClassMethod<'a>>,
    has_method_lifetimes: bool,
}

impl<'a> Class<'a> {
    pub fn new(class: &'a specs::Class, domain_mapper: &DomainMapper) -> Self {
        let methods = class.methods().iter()
            .map(|method| ClassMethod::new(method, domain_mapper))
            .collect::<Vec<_>>();
        let fields = class.fields().iter()
            .map(|field| {
                let domain= Domain::new(domain_mapper.map(field.domain()));
                ClassField::from_amqp0_field(field, domain)
            })
            .collect::<Vec<_>>();

        let has_lifetimes = methods.iter()
            .any(|method| method.has_lifetimes());

        Class {
            class: class,
            methods: methods,
            fields: fields,
            constant_case: class.name().to_constant_case(),
            pascal_case: class.name().to_pascal_case(),
            snake_case: class.name().to_snake_case(),
            has_method_lifetimes: has_lifetimes,
        }
    }

    pub fn methods(&self) -> &[ClassMethod] {
        &self.methods
    }

    pub fn fields(&self) -> &[ClassField] {
        &self.fields
    }

    pub fn constant_case(&self) -> &str {
        &self.constant_case
    }

    pub fn pascal_case(&self) -> &str {
        &self.pascal_case
    }

    pub fn snake_case(&self) -> &str {
        &self.snake_case
    }

    pub fn has_method_lifetimes(&self) -> bool {
        self.has_method_lifetimes
    }
}

impl<'a> Deref for Class<'a> {
    type Target = specs::Class;
    fn deref(&self) -> &Self::Target {
        &self.class
    }
}