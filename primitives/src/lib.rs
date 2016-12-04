// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate nom;

use std::fmt;

pub mod amqp0;
pub mod amqp0_10;
pub mod amqp1;

pub struct Version {
    major: u8,
    minor: u8,
    revision: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, revision: u8) -> Version {
        Version {
            major: major,
            minor: minor,
            revision: revision,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.revision)
    }
}
