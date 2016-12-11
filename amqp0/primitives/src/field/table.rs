// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Table<'a> {
    values: HashMap<Cow<'a, str>, Value<'a>>,
}

impl Table<'static> {
    pub fn new() -> Self {
        Self::from_hashmap(HashMap::new())
    }
}

impl<'a> Table<'a> {

    pub fn from_hashmap(hashmap: HashMap<Cow<'a, str>, Value<'a>>) -> Self {
        Table {
            values: hashmap
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::from_hashmap(HashMap::with_capacity(cap))
    }

    pub fn into_static(self) -> Table<'static> {
        let hashmap = self.values.into_iter()
            .map(|(k, v)| { (k.into_owned().into(), v.into_static()) })
            .collect();

        Table::from_hashmap(hashmap)
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value<'a>>
        where K: Into<Cow<'a, str>>,
              V: Into<Value<'a>>
    {
        self.values.insert(key.into(), value.into())
    }

    pub fn amqp_size(&self) -> usize {
        self.values.iter()
            .map(|(k, v)| k.len() + v.amqp_size())
            .sum()
    }
}

impl Default for Table<'static> {
    fn default() -> Self {
        Table::new()
    }
}

impl<'a> Deref for Table<'a> {
    type Target = HashMap<Cow<'a, str>, Value<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> DerefMut for Table<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
