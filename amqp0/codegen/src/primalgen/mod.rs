// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod codegen;

mod class;
mod class_method;
mod specs;

use std::ops::Deref;
use inflections::Inflect;
use common::domain::DomainMapper;

use self::class_method::ClassMethod;
use self::specs::Specs;

pub use self::class::Class;
pub use self::codegen::SpecModuleWriter;
pub use self::codegen::SpecsModuleWriter;

pub struct Spec<'a> {
    spec: &'a ::specs::Spec,
    classes: Vec<Class<'a>>,
    has_lifetimes: bool,

    pascal_case: String,
}

impl<'a> Spec<'a> {
    pub fn new(spec: &'a ::specs::Spec, domain_mapper: &DomainMapper) -> Self {
        let classes = spec.classes().values()
            .map(|c| Class::new(c, domain_mapper))
            .collect::<Vec<_>>();
        let has_lifetimes = classes.iter()
            .flat_map(|c| c.methods())
            .any(|m| m.has_lifetimes());

        Spec {
            spec: spec,
            classes: classes,
            has_lifetimes: has_lifetimes,

            pascal_case: spec.name().to_pascal_case()
        }
    }

    pub fn classes(&self) -> &[Class] {
        &self.classes
    }

    pub fn has_lifetimes(&self) -> bool {
        self.has_lifetimes
    }

    pub fn pascal_case(&self) -> &str {
        &self.pascal_case
    }
}

impl<'a> Deref for Spec<'a> {
    type Target = ::specs::Spec;
    fn deref(&self) -> &Self::Target {
        &self.spec
    }
}
