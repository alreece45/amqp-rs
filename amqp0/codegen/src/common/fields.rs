// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::{btree_map, BTreeMap};
use std::rc::Rc;

use super::field::BasicField;
use common::{Domain, Field};

pub struct FieldVars<'a>(btree_map::Values<'a, &'a str, (Rc<String>, Domain)>);

#[derive(Debug)]
pub struct Fields<'a> {
    fields: BTreeMap<&'a str, (Rc<String>, Domain)>,
    has_lifetimes: bool,
}

impl<'a> Fields<'a> {
    pub fn new<'b, I, T>(iter: I) -> Self
        where I: IntoIterator<Item = &'b Field<T>>,
              T: BasicField + 'b
    {
        let mut fields = Fields {
            fields: BTreeMap::new(),
            has_lifetimes: false,
        };
        fields.extend(iter);
        fields
    }

    pub fn extend<'b, I, T>(&mut self, fields: I)
        where I: IntoIterator<Item = &'b Field<T>>,
            T: BasicField + 'b
    {
        for field in fields {
            let ty = field.ty();

            match self.fields.get(field.name()) {
                Some(&(_, ref ty)) if ty == ty => continue,
                Some(&(_, ref ty)) => panic!(
                    "Conflicting field types for {} ({:?} and {:?})",
                    field.name(), ty, field.ty()
                ),
                None => (),
            }

            if !field.ty().is_copy() {
                self.has_lifetimes = true;
            }

            self.fields.insert(field.name(), (field.var_name().clone(), ty.clone()));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn has_lifetimes(&self) -> bool {
        self.has_lifetimes
    }

    pub fn vars(&self) -> FieldVars {
        FieldVars(self.fields.values())
    }
}

impl<'a> Iterator for FieldVars<'a> {
    type Item = (&'a str, &'a Domain);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|&(ref name, ref ty)| (name.as_str(), ty))
    }
}