// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::ParserPool;
use lifeguard::Pool;

struct SimpleParserPool {
    values_cap: usize,
    table_entries_cap: usize,

    table_entries: Pool<List<'static>>,
    table_hashmaps: Pool<Table<'static>>,
    values: Pool<Vec<Val<'static>>>,
}

impl SimpleParserPool {
    pub fn new() -> Self {
        SimpleParserPool {
            values_cap: 10,
            table_entries_cap: 10,

            table_entries: Pool::with_size(10),
            table_hashmaps: Pool::with_size(10),
            values: Pool::with_size(10),
        }
    }

    pub fn set_default_table_entries_cap(&mut self, cap: usize) {
        self.table_entries_cap = cap;
    }

    pub fn set_default_values_cap(&mut self, cap: usize) {
        self.values_cap = cap;
    }
}

impl ParserPool for SimpleParserPool {
    fn new_table_hashmap(&mut self, cap: usize) -> HashMap<&'static str, &'static str> {
        if self.table_hashmaps.size() == 0 {
            HashMap::with_capacity(cap)
        }
        else {
            self.table_hashmaps.new().detach()
        }
    }

    fn new_table_entries_vec(&mut self, _: &[u8]) -> Vec<(&'static str, Value<'static>)> {
        if self.table_entries.size() == 0 {
            Vec::with_capacity(self.table_entries_cap)
        }
        else {
            self.table_entries.new().detach()
        }
    }

    fn new_values_vec(&mut self, _: &[u8]) -> Vec<Val<'static>> {
        if self.values.size() == 0 {
            Vec::with_capacity(self.values_cap)
        }
        else {
            self.values.new().detach()
        }
    }

    fn return_table_hashmap(&mut self, hashmap: HashMap<&'static str, &'static str>) {
        self.table_hashmaps.attach(hashmap);
    }

    fn return_table_entries_vec(&mut self, entries: Vec<(&'static str, Value<'static>)>) {
        self.table_entries.attach(entries);
    }

    fn return_values_vec(&mut self, values: Vec<Value<'static>>) {
        self.values.attach(values);
    }
}