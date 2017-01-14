// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::ops::{Deref, DerefMut};
use byteorder::{WriteBytesExt, BigEndian};

use Encodable;
use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct List<'a> {
    values: Vec<Value<'a>>,
}

impl List<'static> {
    pub fn new() -> Self {
        Self::from_vec(Vec::new())
    }
}

impl<'a> List<'a> {
    pub fn from_vec(values: Vec<Value<'a>>) -> Self {
        List {
            values: values
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self::from_vec(Vec::with_capacity(cap))
    }

    pub fn into_static(self) -> List<'static> {
        List::from_vec(self.values.into_iter()
            .map(|v| v.into_static())
            .collect())
    }

    pub fn push<V>(&mut self, value: V)
        where V: Into<Value<'a>>
    {
        self.values.push(value.into())
    }

    pub fn values_encoded_size(&self) -> usize {
        self.values.iter()
            .map(|v| v.encoded_size())
            .sum()
    }
}

impl<'a> Encodable for List<'a> {
    fn encoded_size(&self) -> usize {
        4 + self.values_encoded_size()
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.values_encoded_size() > (::std::u32::MAX as usize) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "AMQP list too large"));
        }

        try!(writer.write_u32::<BigEndian>(self.values_encoded_size() as u32));
        for value in self.iter() {
            try!(value.write_encoded_to(writer));
        }

        Ok(())
    }
}

impl<'a> Deref for List<'a> {
    type Target = Vec<Value<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> DerefMut for List<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}

impl Default for List<'static> {
    fn default() -> Self {
        List::new()
    }
}