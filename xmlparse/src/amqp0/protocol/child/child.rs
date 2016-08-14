// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use amqp0::protocol::{Class, Constant, Domain};

#[derive(Debug)]
pub enum Child<'a> {
    Constant(Constant<'a>),
    Domain(Domain<'a>),
    Class(Class<'a>),
}

impl<'a> From<Constant<'a>> for Child<'a> {
    fn from(child: Constant<'a>) -> Self {
        Child::Constant(child)
    }
}

impl<'a> From<Domain<'a>> for Child<'a> {
    fn from(child: Domain<'a>) -> Self {
        Child::Domain(child)
    }
}

impl<'a> From<Class<'a>> for Child<'a> {
    fn from(child: Class<'a>) -> Self {
        Child::Class(child)
    }
}
