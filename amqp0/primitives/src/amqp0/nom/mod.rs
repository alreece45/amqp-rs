// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(not(feature = "lifeguard"))]
mod lifeguard;

use std::str;
use nom::IResult;
use nom::{be_u8, be_u32};

#[cfg(not(feature = "lifeguard"))]
pub use self::lifeguard::LifeguardParserPool;

use super::field::{Table, List, Value};

pub trait NomBytes<'a>: Sized {
    fn nom_bytes<'b, P>(&'a [u8], &'b mut P) -> IResult<&'a [u8], Self>
        where P: ParserPool,
              Self: 'a;
}

named!(pub bool_bit<(&[u8], usize), bool>,
    map!(take_bits!(u8, 1), |b: u8| -> bool { b != 0 })
);

named!(pub shortstr<&str>, map_res!(
    length_bytes!(be_u8),
    str::from_utf8
));

named!(pub longstr, length_bytes!(be_u32));

/// Creates the objects that may be needed for parsing
///
/// When parsing AMQP "fields", some values may require dynamic allocation-- at least the
/// ones can't just reference into the byte-string (Table and List rather than &str or &[u8]).
///
/// Rather than allocate a new one each time, we let the user decide how to allocate these, based
/// on the needed capacity.
pub trait ParserPool {
    /// Given a capacity, returns a Table
    fn new_table(&mut self, usize) -> Table<'static>;
    /// Given a capacity, returns a vector for Vals
    /// Most likely to be used to assemble a List
    fn new_values_vec(&mut self, &[u8]) -> Vec<Value<'static>>;

    /// Given the bytes for a table, returns a Vec to accept table entries
    /// Most likely to be used to assemble a table
    fn new_table_entries_vec(&mut self, &[u8]) -> Vec<(&'static str, Value<'static>)>;

    fn return_list(&mut self, _: List<'static>) {}
    fn return_table(&mut self, _: Table<'static>) {}
    fn return_table_entries_vec(&mut self, _: Vec<(&'static str, Value<'static>)>) {}
    fn return_vec(&mut self, _: Vec<Value<'static>>) {}
}

/// Creates objects as needed (no pools, no configuration, no attributes)
pub struct NoParserPool;

impl ParserPool for NoParserPool {
    fn new_table(&mut self, cap: usize) -> Table<'static> {
        Table::with_capacity(cap)
    }
    fn new_values_vec(&mut self, _: &[u8]) -> Vec<Value<'static>> {
        Vec::with_capacity(10)
    }
    fn new_table_entries_vec(&mut self, _: &[u8]) -> Vec<(&'static str, Value<'static>)> {
        Vec::with_capacity(10)
    }
}