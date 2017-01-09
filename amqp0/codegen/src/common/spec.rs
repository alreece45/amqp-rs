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
use std::hash::{Hash, Hasher};

use inflections::Inflect;
use lazycell::LazyCell;

use common::{DomainMapper, Class};

#[derive(Clone)]
pub struct Spec {
    spec: &'static ::specs::Spec,

    classes: Rc<LazyCell<Vec<Class>>>,
    mod_name: Rc<LazyCell<String>>,
    pascal_case: Rc<LazyCell<String>>,
    has_lifetimes: Cell<Option<bool>>,
}

impl Spec {
    pub fn new(spec: &'static ::specs::Spec) -> Self {
        Spec {
            spec: spec,
            classes: Rc::new(LazyCell::new()),
            mod_name: Rc::new(LazyCell::new()),
            pascal_case: Rc::new(LazyCell::new()),
            has_lifetimes: Cell::new(None),
        }
    }

    pub fn domain_mapper(&self) -> DomainMapper {
        DomainMapper::from_spec(self.spec)
    }

    pub fn classes(&self) -> &[Class] {
        self.classes.borrow_with(|| {
            self.spec.classes().values()
                .map(|class| Class::new(self.spec, class))
                .collect::<Vec<_>>()
        })
    }

    pub fn has_lifetimes(&self) -> bool {
        if self.has_lifetimes.get().is_none() {
            let has_lifetime = self.classes().iter()
                .flat_map(|c| c.methods())
                .any(|m| m.has_lifetimes());
            self.has_lifetimes.set(Some(has_lifetime));
        }
        self.has_lifetimes.get().unwrap()
    }

    pub fn pascal_case(&self) -> &str {
        self.pascal_case.borrow_with(|| {
            self.name().to_pascal_case()
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