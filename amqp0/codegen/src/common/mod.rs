// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod domain;
mod field;
mod fields;
mod class;
mod class_method;
mod spec;
mod specs;

use inflections::Inflect;

pub use self::class::{Class, ClassField};
pub use self::class_method::{ClassMethod, ClassMethodField};
pub use self::domain::{Domain, DomainMapper};
pub use self::field::Field;
pub use self::fields::Fields;
pub use self::spec::Spec;
pub use self::specs::{Specs, SpecMethod};

pub fn frame_type_name(name: &str) -> String {
    let name_start = if name.starts_with("frame-") { 6 } else { 0 };
    (&name[name_start..]).to_pascal_case()
}