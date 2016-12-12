// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(not(feature = "lifeguard"))]
mod lifeguard;

use std::borrow::Cow;
use std::collections::HashMap;
use primitives::field::Value;

/// Creates the objects that may be needed for parsing
///
/// When parsing AMQP "fields", some values may require dynamic allocation-- at least the
/// ones can't just reference into the byte-string (Table and List rather than &str or &[u8]).
///
/// Rather than allocate a new one each time, we let the user decide how to allocate these, based
/// on the needed capacity.
pub trait ParserPool {
    /// Given a capacity, returns a Table
    fn new_table_hashmap(&mut self, usize) -> HashMap<Cow<'static, str>, Value<'static>>;

    /// Given the bytes for a table, returns a Vec to accept table entries
    /// Most likely to be used to assemble a table
    fn new_table_entries_vec(&mut self, &[u8]) -> Vec<(&'static str, Value<'static>)>;

    /// Given a capacity, returns a vector for Vals
    /// Most likely to be used to assemble a List
    fn new_values_vec(&mut self, &[u8]) -> Vec<Value<'static>>;

    fn return_table_hashmap(&mut self, _: HashMap<&'static str, &'static str>) {}
    fn return_table_entries_vec(&mut self, _: Vec<(&'static str, Value<'static>)>) {}
    fn return_values_vec(&mut self, _: Vec<Value<'static>>) {}
}

/// Creates objects as needed (no pools, no configuration, no attributes)
pub struct NoParserPool;

impl ParserPool for NoParserPool {
    fn new_table_hashmap(&mut self, cap: usize) -> HashMap<Cow<'static, str>, Value<'static>> {
        HashMap::with_capacity(cap)
    }
    fn new_values_vec(&mut self, _: &[u8]) -> Vec<Value<'static>> {
        Vec::with_capacity(10)
    }
    fn new_table_entries_vec(&mut self, _: &[u8]) -> Vec<(&'static str, Value<'static>)> {
        Vec::with_capacity(10)
    }
}