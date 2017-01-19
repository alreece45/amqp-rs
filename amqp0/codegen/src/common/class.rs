// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::iter::ExactSizeIterator;
use std::collections::{btree_map, BTreeMap, BTreeSet, HashMap};
use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

use inflections::Inflect;
use lazycell::LazyCell;

use specs;

use common::{ClassMethod, Field, DomainMapper};

#[derive(Clone)]
pub struct Class {
    spec: &'static specs::Spec,
    class: &'static specs::Class,

    constant_case: Rc<LazyCell<String>>,
    snake_case: Rc<LazyCell<String>>,
    pascal_case: Rc<LazyCell<String>>,

    fields: Rc<LazyCell<Vec<Field>>>,
    methods: Rc<LazyCell<BTreeMap<&'static str, ClassMethod>>>,
    method_indexes: Rc<LazyCell<BTreeMap<u16, Vec<&'static str>>>>,

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
            method_indexes: Rc::new(LazyCell::new()),
            fields: Rc::new(LazyCell::new()),

            has_field_lifetimes: Cell::new(None),
            has_method_lifetimes: Cell::new(None),
        }
    }

    pub fn class(&self) -> &'static specs::Class {
        self.class
    }

    pub fn name(&self) -> &'static str {
        self.class.name()
    }

    fn method_map(&self) -> &BTreeMap<&str, ClassMethod> {
        self.methods.borrow_with(|| {
            self.class.methods()
                .map(|method| (method.name(), ClassMethod::new(self.spec, method)))
                .collect::<_>()
        })
    }

    fn method_index_map(&self) -> &BTreeMap<u16, Vec<&str>> {
        self.method_indexes.borrow_with(|| {
            self.class.methods()
                .fold(HashMap::new(), |mut map, method| {
                    map.entry(method.index())
                        .or_insert_with(BTreeSet::new)
                        .insert(method.name());
                    map
                })
                .into_iter()
                .map(|(index, methods)| {
                    (index, methods.into_iter().collect())
                })
                .collect()
        })
    }

    pub fn method<'a>(&self, name: &'a str) -> Option<&ClassMethod> {
        self.method_map().get(name)
    }

    pub fn methods(&self) -> Methods {
        Methods(self.method_map().values())
    }

    pub fn method_indexes<'a>(&'a self) -> MethodIndexes<'a> {
        MethodIndexes(self.method_index_map().iter())
    }

    pub fn fields(&self) -> &[Field] {
        self.fields.borrow_with(|| {
            let domain_mapper = DomainMapper::from_spec(self.spec);
            self.class.fields()
                .map(|field| Field::from_field(field, domain_mapper.map(field.domain())))
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
            let has_method_lifetimes = self.methods()
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

pub struct Methods<'a>(btree_map::Values<'a, &'a str, ClassMethod>);
pub struct MethodIndexes<'a>(btree_map::Iter<'a, u16, Vec<&'a str>>);

impl<'a> Iterator for Methods<'a> {
    type Item = &'a ClassMethod;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> ExactSizeIterator for Methods<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Iterator for MethodIndexes<'a> {
    type Item = (u16, &'a [&'a str]);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(index, method_names)| (*index, &method_names[..]))
    }
}

impl<'a> ExactSizeIterator for MethodIndexes<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}