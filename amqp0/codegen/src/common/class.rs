// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

use inflections::Inflect;
use lazycell::LazyCell;

use specs;

use common::{self, ClassMethod, DomainMapper};

pub type ClassField = common::Field<specs::ClassField>;

#[derive(Clone)]
pub struct Class {
    spec: &'static specs::Spec,
    class: &'static specs::Class,

    constant_case: Rc<LazyCell<String>>,
    snake_case: Rc<LazyCell<String>>,
    pascal_case: Rc<LazyCell<String>>,

    fields: Rc<LazyCell<Vec<ClassField>>>,
    methods: Rc<LazyCell<Vec<ClassMethod>>>,

    has_field_lifetimes: Cell<Option<bool>>,
    has_method_lifetimes: Cell<Option<bool>>,
}

impl Class {
    pub fn new(spec: &'static specs::Spec, class: &'static specs::Class) -> Self {
        Class {
            spec: spec,
            class: class,

            constant_case: Rc::new(LazyCell::new()),
            pascal_case: Rc::new(LazyCell::new()),
            snake_case: Rc::new(LazyCell::new()),

            methods: Rc::new(LazyCell::new()),
            fields: Rc::new(LazyCell::new()),

            has_field_lifetimes: Cell::new(None),
            has_method_lifetimes: Cell::new(None),
        }
    }

    pub fn methods(&self) -> &[ClassMethod] {
        self.methods.borrow_with(|| {
            self.class.methods().iter()
                .map(|method| ClassMethod::new(self.spec, method))
                .collect::<Vec<_>>()
        })
    }

    pub fn fields(&self) -> &[ClassField] {
        self.fields.borrow_with(|| {
            let domain_mapper = DomainMapper::from_spec(self.spec);
            self.class.fields().iter()
                .cloned()
                .map(|field| {
                    let domain = domain_mapper.map(field.domain());
                    ClassField::from_amqp0_field(field, domain)
                })
                .collect::<Vec<_>>()
        })
    }

    pub fn constant_case(&self) -> &str {
        self.constant_case.borrow_with(|| {
            self.class.name().to_constant_case()
        })
    }

    pub fn pascal_case(&self) -> &str {
        self.pascal_case.borrow_with(|| {
            self.class.name().to_pascal_case()
        })
    }

    pub fn snake_case(&self) -> &str {
        self.snake_case.borrow_with(|| {
            self.class.name().to_snake_case()
        })
    }

    pub fn has_field_lifetimes(&self) -> bool {
        if self.has_field_lifetimes.get().is_none() {
            let has_field_lifetimes = self.fields().iter()
                .any(|field| !field.ty().is_copy());
            self.has_field_lifetimes.set(Some(has_field_lifetimes));
        }
        self.has_field_lifetimes.get().unwrap()
    }

    pub fn has_method_lifetimes(&self) -> bool {
        if self.has_method_lifetimes.get().is_none() {
            let has_method_lifetimes = self.methods().iter()
                .any(|method| method.has_lifetimes());
            self.has_method_lifetimes.set(Some(has_method_lifetimes));
        }
        self.has_method_lifetimes.get().unwrap()
    }
}

impl Deref for Class {
    type Target = specs::Class;
    fn deref(&self) -> &Self::Target {
        self.class
    }
}