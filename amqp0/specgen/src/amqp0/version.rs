// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Version {
    minor: u8,
    revision: u8,
}

impl Version {
    pub fn new(major: u8, minor: u8, revision: u8) -> Self {
        // some of the AMQP specs store the "minor" version in the "major "field instead.
        // Since we only parse AMQP 0.x -- we assume that major should always be 0
        let (minor, revision) = if major != 0 {
            (major, minor)
        } else {
            (minor, revision)
        };
        Version {
            minor: minor,
            revision: revision,
        }
    }

    pub fn minor(&self) -> u8 {
        self.minor
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0.{}.{}", self.minor, self.revision)
    }
}
