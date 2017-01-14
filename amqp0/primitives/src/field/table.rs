// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::HashMap;
use std::io;
use std::ops::{Deref, DerefMut};

use byteorder::{WriteBytesExt, BigEndian};

use Encodable;
use super::Value;

pub type TableEntry<'a> = (Cow<'a, str>, Value<'a>);

#[derive(Clone, Debug, PartialEq)]
pub struct TableEntries<'a> {
    entries: CowTableEntries<'a>
}

impl TableEntries<'static> {
    pub fn new() -> Self {
        TableEntries {
            entries: CowTableEntries::Borrowed(&[])
        }
    }
}

impl<'a> TableEntries<'a> {
    pub fn from_entries<T>(entries: T) -> Self
        where T: Into<Cow<'a, [TableEntry<'a>]>>
    {
        TableEntries {
            entries: CowTableEntries::from_cow(entries)
        }
    }

    pub fn into_static(self) -> TableEntries<'static> {
        TableEntries {
            entries: self.entries.into_static()
        }
    }

    pub fn to_hashmap(&self) -> HashMap<Cow<'a, str>, Value<'a>> {
        self.entries.iter().cloned().collect()
    }

    pub fn to_table(&self) -> Table<'a> {
        Table::from_hashmap(self.to_hashmap())
    }

    pub fn encoded_entries_size(&self) -> usize {
        self.entries.iter()
            .map(|&(ref k, ref v)| k.encoded_size() + v.encoded_size())
            .sum()
    }
}

impl<'a> Encodable for TableEntries<'a> {
    fn encoded_size(&self) -> usize {
        4 + self.encoded_entries_size()
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writer.write_u32::<BigEndian>(self.encoded_entries_size() as u32));

        for &(ref key, ref value) in self.iter() {
            try!(key.write_encoded_to(writer));
            try!(value.write_encoded_to(writer));
        }

        Ok(())
    }
}

impl<'a> Deref for TableEntries<'a> {
    type Target = [TableEntry<'a>];
    fn deref(&self) -> &Self::Target {
        &*self.entries
    }
}

impl<'a> DerefMut for TableEntries<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entries.to_mut()
    }
}

impl Default for TableEntries<'static> {
    fn default() -> Self {
        TableEntries::new()
    }
}

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

///
/// Workaround `Cow`-like enum: Contains a collection of `TableEntry` that may or may not be owned.
///
/// This *should* simply be a struct with a `Cow<'a, [TableEntry<'a>]>`, but that currently
/// overflows the requirement checking. Defining this as an enum works around the bug, This works around the bug-- but you should use the methods
/// rather than rely on this being an enum-- as it should be a struct.
///
///   Bug: https://github.com/rust-lang/rust/issues/23714
///   Playground: https://play.rust-lang.org/?gist=9ed87cb1b6b890d155e47e8d3f7cacd8
///
/// # Bug test
///
/// This tests to make sure the bug is still present. If this library is "stable" when fixed,
/// It should be replaced with Cow, but should be done behind a cfg. possibly configured in build.rs
///
/// ```should_fail
///
/// use std::borrow::Cow;
///
/// fn main() {
///
///     #[derive(Clone)]
///     pub enum Value<'a> {
///         Table(Entries<'a>),
///     }
///
///     #[derive(Clone)]
///     pub struct Entries<'a> {
///         entries: Cow<'a, [(Cow<'a, str>, Value<'a>)]>,
///     }
///
///     println!("This work-around is no longer needed.")
/// }
/// ```
///
#[derive(Debug, Clone, PartialEq)]
enum CowTableEntries<'a> {
    Borrowed(&'a [TableEntry<'a>]),
    Owned(Vec<TableEntry<'a>>)
}

impl<'a> CowTableEntries<'a> {
    fn from_cow<T>(cow: T) -> Self
        where T: Into<Cow<'a, [TableEntry<'a>]>>
    {
        match cow.into() {
            Cow::Borrowed(entries) => CowTableEntries::Borrowed(entries),
            Cow::Owned(entries) => CowTableEntries::Owned(entries),
        }
    }

    // name is taken from Cow::to_mut()
    #[allow(wrong_self_convention)]
    fn to_mut(&mut self) -> &mut Vec<TableEntry<'a>> {
        match *self {
            CowTableEntries::Borrowed(entries) => {
                *self = CowTableEntries::Owned(entries.to_owned());
                self.to_mut()
            }
            CowTableEntries::Owned(ref mut entries) => entries,
        }
    }

    fn into_static(self) -> CowTableEntries<'static> {
        let entries = match self {
            CowTableEntries::Borrowed(entries) => {
                entries.iter()
                    .cloned()
                    .map(|(k, v)| {
                        (Cow::Owned(k.into_owned()), v.into_static())
                    })
                    .collect()
            },
            CowTableEntries::Owned(entries) =>  {
                entries.into_iter()
                    .map(|(k, v)| { (Cow::Owned(k.into_owned()), v.into_static()) })
                    .collect()
            },
        };

        CowTableEntries::Owned(entries)
    }
}

impl<'a> Deref for CowTableEntries<'a> {
    type Target = [TableEntry<'a>];
    fn deref(&self) -> &Self::Target {
        match *self {
            CowTableEntries::Borrowed(entries) => entries,
            CowTableEntries::Owned(ref entries) => entries
        }
    }
}