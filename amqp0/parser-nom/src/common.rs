// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;
use nom::IResult;
use nom::{be_u8, be_u32};

named!(pub bool_bit<(&[u8], usize), bool>,
    map!(take_bits!(u8, 1), |b: u8| -> bool { b != 0 })
);

named!(pub shortstr<&str>, map_res!(
    length_bytes!(be_u8),
    str::from_utf8
));

named!(pub longstr, length_bytes!(be_u32));