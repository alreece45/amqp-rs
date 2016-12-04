
use std::ops::{Deref, DerefMut};

use super::{Val, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    values: Vec<Val<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListBuf {
    values: Vec<Value>,
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }

    pub fn from_vec(values: Vec<Val<'a>>) -> Self {
        List {
            values: values
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::from_vec(Vec::with_capacity(cap))
    }

    pub fn into_owned(self) -> ListBuf {
        ListBuf::from_vec(self.values.into_iter()
            .map(|v| v.into_owned())
            .collect())
    }

    pub fn push<V>(&mut self, value: V)
        where V: Into<Val<'a>>
    {
        self.values.push(value.into())
    }

    pub fn amqp_size(&self) -> usize {
        self.values.iter()
            .map(|v| v.amqp_size())
            .sum()
    }
}

impl ListBuf {
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }
    pub fn from_vec(values: Vec<Value>) -> Self {
        ListBuf {
            values: values
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self::from_vec(Vec::with_capacity(cap))
    }
}

impl<'a> Deref for List<'a> {
    type Target = Vec<Val<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> DerefMut for List<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

/*
impl<'a> ToOwned for List<'a> {
    type Owned = ListBuf;
    fn to_owned(&self) -> Self::Owned {
        self.iter()
    }
}

impl<'a> From<List<'a>> for Cow<'a, List<'a>> {
    fn from(list: List) -> Self {
        Cow::Borrowed(list)
    }
}

impl From<ListBuf> for Cow<'static, List<'static>> {
    fn from(list: ListBuf) -> Self {
        Cow::Owned(list)
    }
}
*/

