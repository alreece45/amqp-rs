// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::collections::{btree_map, BTreeMap, BTreeSet, HashMap};
use std::iter::ExactSizeIterator;
use std::ops::Deref;
use std::rc::Rc;
use std::hash::{Hash, Hasher};

use inflections::Inflect;
use lazycell::LazyCell;

use common::{DomainMapper, Class};

#[derive(Clone)]
pub struct Spec {
    spec: &'static ::specs::Spec,

    class_indexes: Rc<LazyCell<BTreeMap<u16, Vec<&'static str>>>>,
    class_map: Rc<LazyCell<BTreeMap<&'static str, Class>>>,
    mod_name: Rc<LazyCell<String>>,
    pascal_case: Rc<LazyCell<String>>,
    has_lifetimes: Cell<Option<bool>>,
}

impl Spec {
    pub fn new(spec: &'static ::specs::Spec) -> Self {
        Spec {
            spec: spec,
            class_indexes: Rc::new(LazyCell::new()),
            class_map: Rc::new(LazyCell::new()),
            mod_name: Rc::new(LazyCell::new()),
            pascal_case: Rc::new(LazyCell::new()),
            has_lifetimes: Cell::new(None),
        }
    }

    pub fn domain_mapper(&self) -> DomainMapper {
        DomainMapper::from_spec(self.spec)
    }

    pub fn classes<'a>(&'a self) -> Classes<'a> {
        Classes {
            classes: self.class_name_map().values()
        }
    }

    pub fn class_indexes<'a>(&'a self) -> ClassIndexes<'a> {
        ClassIndexes {
            class_indexes: self.class_index_map().iter()
        }
    }

    pub fn class<'a>(&self, name: &'a str) -> Option<&Class> {
        self.class_name_map().get(name)
    }

    fn class_name_map(&self) -> &BTreeMap<&'static str, Class> {
        self.class_map.borrow_with(|| {
            self.spec.classes().values()
                .map(|class| (class.name(), Class::new(self.spec, class)))
                .collect::<BTreeMap<_, _>>()
        })
    }

    fn class_index_map<'a>(&'a self) -> &BTreeMap<u16, Vec<&'static str>> {
        self.class_indexes.borrow_with(|| {
            self.spec.classes().entries()
                .fold(HashMap::new(), |mut map, (name, class)| {
                    map.entry(class.index())
                        .or_insert(BTreeSet::new())
                        .insert(*name);
                    map
                })
                .into_iter()
                .map(|(index, classes)| (index, classes.into_iter().collect()))
                .collect()
        })
    }

    pub fn has_lifetimes(&self) -> bool {
        if self.has_lifetimes.get().is_none() {
            let has_lifetime = self.classes()
                .flat_map(|c| c.methods())
                .any(|m| m.has_lifetimes());
            self.has_lifetimes.set(Some(has_lifetime));
        }
        self.has_lifetimes.get().unwrap()
    }

    pub fn pascal_case(&self) -> &str {
        self.pascal_case.borrow_with(|| {
            format!(
                "{}{}_{}",
                self.name().to_pascal_case(),
                self.version().minor(),
                self.version().revision()
            )
        })
    }

    pub fn mod_name(&self) -> &str {
        self.mod_name.borrow_with(|| {
            format!(
                "{}{}_{}",
                self.name().to_snake_case(),
                self.version().minor(),
                self.version().revision()
            )
        })
    }
}

pub struct Classes<'a> {
    classes: btree_map::Values<'a, &'a str, Class>,
}

impl<'a> Iterator for Classes<'a> {
    type Item = &'a Class;

    fn next(&mut self) -> Option<Self::Item> {
        self.classes.next()
    }
}

impl<'a> ExactSizeIterator for Classes<'a> {
    fn len(&self) -> usize {
        self.classes.len()
    }
}

pub struct ClassIndexes<'a> {
    class_indexes: btree_map::Iter<'a, u16, Vec<&'a str>>,
}

impl<'a> Iterator for ClassIndexes<'a> {
    type Item = (u16, &'a [&'a str]);

    fn next(&mut self) -> Option<Self::Item> {
        self.class_indexes.next()
            .map(|(index, class_names)| (*index, &class_names[..]))
    }
}
impl<'a> ExactSizeIterator for ClassIndexes<'a> {
    fn len(&self) -> usize {
        self.class_indexes.len()
    }
}

impl Deref for Spec {
    type Target = ::specs::Spec;
    fn deref(&self) -> &Self::Target {
        self.spec
    }
}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.spec.hash(state);
    }
}

impl PartialEq for Spec {
    fn eq(&self, other: &Self) -> bool {
        self.spec.eq(other.spec)
    }
}
impl Eq for Spec {}