// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;

use parser::amqp0 as parsed;

pub struct Group<'a, 'b: 'a> {
    group: Cow<'b, str>,
    name: Cow<'b, str>,
    ty: Cow<'b, str>,
    constants: Vec<&'a parsed::Constant<'b>>,
}

impl<'a, 'b: 'a> Group<'a, 'b> {
    pub fn new<N, G, T, I, C>(group: G, name: N, ty: T, constants: I) -> Self
        where N: Into<Cow<'b, str>>,
              G: Into<Cow<'b, str>>,
              T: Into<Cow<'b, str>>,
              I: Iterator<Item=C>,
              C: Into<&'a parsed::Constant<'b>>
    {
        Group {
            group: group.into(),
            name: name.into(),
            ty: ty.into(),
            constants: constants.map(|c| c.into()).collect(),
        }
    }

    pub fn constants(&self) -> &Vec<&'a parsed::Constant<'b>> {
        &self.constants
    }

    pub fn group(&self) -> &str {
        &*self.group
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn ty(&self) -> &str {
        &*self.ty
    }
}