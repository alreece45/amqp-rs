
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use super::{Val, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Table<'a> {
    values: HashMap<Cow<'a, str>, Val<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableBuf {
    values: HashMap<String, Value>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self::from_hashmap(HashMap::new())
    }

    pub fn from_hashmap(hashmap: HashMap<Cow<'a, str>, Val<'a>>) -> Self {
        Table {
            values: hashmap
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::from_hashmap(HashMap::with_capacity(cap))
    }

    pub fn into_owned(self) -> TableBuf {
        TableBuf::from_hashmap(self.values.into_iter()
            .map(|(k, v)| { (k.into_owned(), v.into_owned()) })
            .collect())
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Val<'a>>
        where K: Into<Cow<'a, str>>,
              V: Into<Val<'a>>
    {
        self.values.insert(key.into(), value.into())
    }

    pub fn amqp_size(&self) -> usize {
        self.values.iter()
            .map(|(k, v)| k.len() + v.amqp_size())
            .sum()
    }
}

impl TableBuf {
    pub fn new() -> Self {
        Self::from_hashmap(HashMap::new())
    }

    pub fn from_hashmap(hashmap: HashMap<String, Value>) -> Self {
        TableBuf {
            values: hashmap
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::from_hashmap(HashMap::with_capacity(cap))
    }
}

impl<'a> Deref for Table<'a> {
    type Target = HashMap<Cow<'a, str>, Val<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> DerefMut for Table<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

/*
impl<'a> ToOwned for Table<'a> {
    type Owned = TableBuf;
    fn to_owned(&self) -> Self::Owned {
        self.iter()
    }
}

impl<'a> From<Table<'a>> for Cow<'a, Table<'a>> {
    fn from(table: Table) -> Self {
        Cow::Borrowed(table)
    }
}

impl From<TableBuf> for Cow<'static, Table<'static>> {
    fn from(table: TableBuf) -> Self {
        Cow::owned(table)
    }
}
*/