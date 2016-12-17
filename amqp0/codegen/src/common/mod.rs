// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod field;
pub mod domain;

use specs::Spec;
use inflections::Inflect;

pub use self::field::Field;

pub fn spec_mod_name(spec: &Spec) -> String {
    let (minor, revision) = {
        let version = spec.version();
        (version.minor(), version.revision())
    };
    format!("{}{}_{}", spec.name().to_snake_case(), minor, revision)
}

pub fn frame_type_name(name: &str) -> String {
    let name_start = if name.starts_with("frame-") { 6 } else { 0 };
    (&name[name_start..]).to_pascal_case()
}