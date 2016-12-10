// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use super::FormatRustCode;
use super::format_to_vec;

impl<'a, T: ?Sized> FormatRustCode for &'a T
    where T: FormatRustCode
{
    fn format_rust(&self) -> String {
        (*self).format_rust()
    }
}

impl<'a> FormatRustCode for bool {
    fn format_rust(&self) -> String {
        if *self {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }
}

impl<'a> FormatRustCode for usize {
    fn format_rust(&self) -> String {
        format!("{:?}usize", self)
    }
}

impl<'a> FormatRustCode for &'a str {
    fn format_rust(&self) -> String {
        format!("{:?}", self)
    }
}

impl FormatRustCode for String {
    fn format_rust(&self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> FormatRustCode for Cow<'a, str> {
    fn format_rust(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T> FormatRustCode for Option<T>
    where T: FormatRustCode
{
    fn format_rust(&self) -> String {
        self.as_ref()
            .map_or_else(|| "None".to_string(), |v| {
                format!("Some({})", v.format_rust())
            })
    }
}

impl<A, B> FormatRustCode for (A, B)
    where A: FormatRustCode,
          B: FormatRustCode
{
    fn format_rust(&self) -> String {
        format!("({}, {})", self.0.format_rust(), self.1.format_rust())
    }
}

impl<T: FormatRustCode> FormatRustCode for Vec<T> {
    fn format_rust(&self) -> String {
        format_to_vec(self.iter())
    }
}


impl<'a, T: FormatRustCode> FormatRustCode for &'a [T] {
    fn format_rust(&self) -> String {
        format_to_vec(self.iter())
    }
}