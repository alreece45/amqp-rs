// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{btree_map, BTreeMap};

use common::Field;

#[derive(Debug)]
pub struct Fields<'a> {
    fields: BTreeMap<&'a str, Field>,
    has_lifetimes: bool,
}

pub struct Iter<'a>(btree_map::Values<'a, &'a str, Field>);

impl<'a> Fields<'a> {
    pub fn new<'b, I>(iter: I) -> Self
        where I: IntoIterator<Item = &'b Field>
    {
        let mut fields = Fields {
            fields: BTreeMap::new(),
            has_lifetimes: false,
        };
        fields.extend(iter);
        fields
    }

    pub fn extend<'b, I>(&mut self, fields: I)
        where I: IntoIterator<Item = &'b Field>,
    {
        for field in fields {
            let ty = field.ty();

            match self.fields.get(field.name()) {
                Some(ref field) if field.ty() == ty => continue,
                Some(ref field) => panic!(
                    "Conflicting field types for {} ({:?} and {:?})",
                    field.name(), ty, field.ty()
                ),
                None => (),
            }

            if !field.ty().is_copy() {
                self.has_lifetimes = true;
            }

            self.fields.insert(field.name(), field.clone());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn has_lifetimes(&self) -> bool {
        self.has_lifetimes
    }

    pub fn iter(&self) -> Iter {
        Iter(self.fields.values())
    }
}

impl<'a> IntoIterator for &'a Fields<'a> {
    type Item = &'a Field;
    type IntoIter = Iter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Field;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}