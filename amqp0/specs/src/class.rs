// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::slice;
use {Class, ClassField, ClassMethod, ClassMethodField};

impl Class {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn fields(&self) -> ClassFields {
        ClassFields(self.fields.iter())
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn methods(&self) -> ClassMethods {
        ClassMethods(self.methods.iter())
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ClassMethodFieldAssertion {
    Null,
    NotNull,
    ChannelMax,
    NotZero,
    Enum(&'static [&'static str]),
    Length(usize),
    Regexp(&'static str),
    Syntax(&'static str),
}

pub struct ClassFields(slice::Iter<'static, ClassField>);
pub struct ClassMethods(slice::Iter<'static, ClassMethod>);
pub struct ClassMethodFields(slice::Iter<'static, ClassMethodField>);
pub struct ClassMethodFieldAssertions(slice::Iter<'static, ClassMethodFieldAssertion>);

impl ClassField {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn domain(&self) -> &'static str {
        self.domain
    }
}

impl ClassMethod {
    pub fn index(&self) -> u16 {
        self.index
    }
    pub fn name(&self) -> &str {
        self.name
    }
    pub fn fields(&self) -> ClassMethodFields {
        ClassMethodFields(self.fields.iter())
    }
    pub fn has_content(&self) -> bool {
        self.has_content
    }
}

impl ClassMethodField {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn domain(&self) -> &'static str {
        self.domain
    }

    pub fn assertions(&self) -> ClassMethodFieldAssertions {
        ClassMethodFieldAssertions(self.assertions.iter())
    }

    pub fn is_reserved(&self) -> bool {
        self.is_reserved
    }
}

impl Iterator for ClassFields {
    type Item = &'static ClassField;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for ClassMethods {
    type Item = &'static ClassMethod;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for ClassMethodFields {
    type Item = &'static ClassMethodField;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}