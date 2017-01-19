// Copyright 2017 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::hash::{Hash, Hasher};

use phf::{ordered_map, OrderedMap, PhfHash};
use {Spec, Class, Constant, Version};

pub struct Classes(ordered_map::Values<'static, &'static str, Class>);
pub struct Constants(ordered_map::Values<'static, &'static str, Constant>);
pub struct Domains(ordered_map::Entries<'static, &'static str, &'static str>);

impl Spec {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn classes(&self) -> Classes {
        Classes(self.classes.values())
    }
    pub fn constants(&self) -> Constants {
        Constants(self.constants.values())
    }
    pub fn domains(&self) -> Domains {
        Domains(self.domains.entries())
    }
    pub fn frame_types(&self) -> Constants {
        Constants(self.frame_types.values())
    }
    pub fn response_codes(&self) -> Constants {
        Constants(self.response_codes.values())
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl PartialEq for Spec {
    fn eq(&self, other: &Spec) -> bool {
        fn is_map_eq<K, V>(map1: &OrderedMap<K, V>, map2: &OrderedMap<K, V>) -> bool
            where K: Eq + PhfHash,
                  V: Eq
        {
            map1.entries().all(|(k, v1)| {
                match map2.get(k) {
                    Some(v2) if v1 == v2 => true,
                    _ => false,
                }
            })
        }

        self.name == other.name
            && self.version == other.version
            && is_map_eq(self.classes, other.classes)
            && is_map_eq(self.constants, other.constants)
            && is_map_eq(self.domains, other.domains)
            && is_map_eq(self.frame_types, other.frame_types)
            && is_map_eq(self.response_codes, other.response_codes)
    }
}

impl Eq for Spec {}

impl Hash for Spec {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        for (name, class) in self.classes {
            name.hash(state);
            class.hash(state);
        }
        for (name, value) in self.constants {
            name.hash(state);
            value.hash(state);
        }
        for (name, mapping) in self.domains {
            name.hash(state);
            mapping.hash(state);
        }
        for (name, value) in self.frame_types {
            name.hash(state);
            value.hash(state);
        }
        for (name, response_code) in self.response_codes {
            name.hash(state);
            response_code.hash(state);
        }
        self.version.hash(state)
    }
}

impl Iterator for Classes {
    type Item = &'static Class;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for Constants {
    type Item = &'static Constant;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iterator for Domains {
    type Item = (&'static str, &'static str);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(name, mapping)| (*name, *mapping))
    }
}
