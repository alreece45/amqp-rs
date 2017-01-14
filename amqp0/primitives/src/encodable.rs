// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::io;

use bit_vec::BitVec;
use byteorder::{WriteBytesExt, BigEndian};

pub trait Encodable {
    fn encoded_size(&self) -> usize;
    fn write_encoded_to<W>(&self, &mut W) -> io::Result<()>
        where W: io::Write;
}

impl<'a, T: Encodable> Encodable for &'a T {
    fn encoded_size(&self) -> usize {
        (*self).encoded_size()
    }
    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        (*self).write_encoded_to(writer)
    }
}

impl Encodable for u8 {
    fn encoded_size(&self) -> usize {
        1
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writer.write_u8(*self)
    }
}

impl Encodable for u16 {
    fn encoded_size(&self) -> usize {
        2
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writer.write_u16::<BigEndian>(*self)
    }
}

impl Encodable for u32 {
    fn encoded_size(&self) -> usize {
        4
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writer.write_u32::<BigEndian>(*self)
    }
}

impl Encodable for u64 {
    fn encoded_size(&self) -> usize {
        4
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writer.write_u64::<BigEndian>(*self)
    }
}

impl<'a> Encodable for Cow<'a, [u8]> {
    fn encoded_size(&self) -> usize {
        1 + self.len()
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.len() > (::std::u32::MAX as usize) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "AMQP long string too large"));
        }

        try!(writer.write_u32::<BigEndian>(self.len() as u32));
        try!(writer.write_all(&self[..]));

        Ok(())
    }
}

impl<'a> Encodable for Cow<'a, str> {
    fn encoded_size(&self) -> usize {
        4 + self.len()
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        if self.len() > (::std::u8::MAX as usize) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "AMQP short string too large"));
        }

        try!(writer.write_u8(self.len() as u8));
        try!(writer.write_all(self.as_bytes()));

        Ok(())
    }
}

impl<T: Encodable> Encodable for Option<T> {
    fn encoded_size(&self) -> usize {
        self.as_ref().map(|val| val.encoded_size()).unwrap_or(0)
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        match self.as_ref() {
            Some(val) => val.write_encoded_to(writer),
            _ => Ok(()),
        }
    }
}

impl Encodable for BitVec {
    fn encoded_size(&self) -> usize {
        self.len() / 8
    }

    fn write_encoded_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writer.write_all(&self.to_bytes()[..])
    }
}

