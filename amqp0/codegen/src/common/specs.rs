// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use common::Spec;

#[derive(Clone)]
pub struct Specs<'a> {
    /// TODO: is Cow<> really the appropriate type, here?
    /// Would AsRef or Borrow be better, since we never take ownership?
    specs: Cow<'a, [Spec]>
}

impl<'a> Specs<'a> {
    pub fn new<S>(specs: S) -> Self
        where S: Into<Cow<'a, [Spec]>>
    {
        Specs {
            specs: specs.into()
        }
    }

    ///
    /// Some of the class indexes change their purpose, based on the protocol version (e.g: 160 is
    /// sometimes the "message" class, and sometimes the "test" class. This isn't usually a problem,
    /// since its clear, from the spec, what the intended use is.
    ///
    /// We only define the constants in the common namespace, so it is important than a name should
    /// NOT have different indexes, e.g: if CLASS_ABC is 0x01 in one version, and 0x02 in another--
    /// we can't reliably define them here (they must be defined in the spec-specific module, rather
    /// than the generic one). here.
    ///
    /// In AMQP, this never happens, so we don't worry about defining the spec-specific
    /// versions. Rather than implement functionality that will never be used, we assert that
    /// our expectation is true (the index for a given name is constant across all of the
    /// primalgen.spec versions).
    ///
    /// No breakage is expected if the assumption ever changes (e.g: if a 0.9.2 is ever released).
    /// The expected behavior, in such a case, is already used when defining common methods. The
    /// common methods are defined in both the common/shared namespace, and in the version-specific
    /// namespaces. See get_common_methods() for more details.
    ///
    pub fn assert_name_indexes_consistent(&self) {
        let mut defined_classes = HashMap::<&str, u16>::new();

        for spec in self.specs.iter() {
            for class in spec.classes() {
                let index = class.index();
                if let Some(old_index) = defined_classes.get(class.name()) {
                    if index != *old_index {
                        panic!(
                            "Unexpected class index change (name: {}, old index: {}, new index: {})",
                            class.name(),
                            old_index,
                            index
                        );
                    }
                }
                defined_classes.insert(class.name(), index);
            }
        }
    }

    /// Finds classes that exist in more than one primalgen.spec
    ///
    /// The only requirement here is for a class to exist in more than one primalgen.spec.
    ///
    pub fn common_classes(&self) -> HashMap<&'a str, u16> {
        let mut classes = HashMap::new();

        for spec in self.specs.iter() {
            for class in spec.classes() {
                let entry = classes.entry(class.name())
                    .or_insert((1, class.index()));
                entry.0 += 1;
            }
        }

        classes.into_iter()
            .filter(|&(_, v)| v.0 > 1)
            .map(|(k, v)| (k, v.1))
            .collect()
    }

    /// Finds classes that exist in more than one primalgen.spec
    ///
    /// The only requirement here is for a class to exist in more than one primalgen.spec.
    ///
    pub fn common_frame_types(&self) -> HashMap<&str, u8> {
        let mut frame_types = HashMap::new();
        for spec in self.specs.iter() {
            for frame_type in spec.frame_types().values() {
                let entry = frame_types.entry(frame_type.name()).or_insert((1, frame_type.value() as u8));
                entry.0 += 1;
            }
        }

        frame_types.into_iter()
            .filter(|&(_, v)| v.0 > 1)
            .map(|(k, v)| (k, v.1))
            .collect()
    }

    ///
    /// Loop through each specs class and method, noting potentially common methods, removing
    /// them if the index doesn't match.
    ///
    /// Like above, its problematic if an index value for a name changes. This happens fairly frequently
    /// in the 0.x.y versions (before 0.10). For methods, we must either:
    ///
    ///  * Duplication: Redefining constants in each specification
    ///  * Usability: Create common constants when possible
    ///
    /// The former results in more constants available. The latter makes it clear what's the same between
    /// all of the specifications, but it adds complexity for those only caring about one constant.
    ///
    /// We implement the benefits of both: detect and redefine the common constants, but keep them
    /// defined in each specification as well (users don't need to know which are common, but can clearly
    /// see it if needed.
    ///
    /// This function does not take into account the arguments and behavior may vary between versions.
    ///
    pub fn common_methods(&self) -> HashMap<&str, HashMap<&str, u16>> {
        // internal structs
        struct MethodGroups<'b> {
            maybe_common: HashMap<&'b str, MethodGroup<'b>>,
            uncommon: HashSet<&'b str>,
        }
        struct MethodGroup<'b> {
            index: u16,
            specs: HashSet<&'b Spec>,
        }

        let mut groups = HashMap::new();

        for spec in self.specs.iter() {
            for class in spec.classes() {
                let mut groups = groups.entry(class.name())
                    .or_insert_with(|| {
                        MethodGroups {
                            maybe_common: HashMap::new(),
                            uncommon: HashSet::new(),
                        }
                    });

                for method in class.methods() {
                    let method_name = method.name();
                    if groups.uncommon.contains(method_name) {
                        continue;
                    }

                    // Open reference prevents removing the hashmap while a reference is open in the
                    // match. Note the action to take and process it after the match.
                    enum MethodAction {
                        Insert,
                        Remove,
                    }

                    let method_index: u16 = method.index();
                    let action = match groups.maybe_common.get_mut(method_name) {
                        Some(ref mut group) if group.index == method_index => {
                            group.specs.insert(spec);
                            None
                        },
                        None => Some(MethodAction::Insert),
                        Some(_) => Some(MethodAction::Remove),
                    };

                    match action {
                        Some(MethodAction::Insert) => {
                            groups.maybe_common.insert(method_name, MethodGroup {
                                index: method_index,
                                specs: vec![spec].into_iter().collect(),
                            });
                        },
                        Some(MethodAction::Remove) => {
                            groups.uncommon.insert(method_name);
                            groups.maybe_common.remove(method_name);
                        },
                        _ => {},
                    }
                }
            }
        }

        groups.into_iter()
            .filter(|&(_, ref group)| !group.maybe_common.is_empty())
            .map(|(class_name, group)| {
                let methods = group.maybe_common.into_iter()
                    .map(|(method_name, group)| (method_name, group.index))
                    .collect::<HashMap<_, _>>();
                (class_name, methods)
            })
            .collect()
    }
}

impl<'a> Deref for Specs<'a> {
    type Target = [Spec];

    fn deref(&self) -> &Self::Target {
        &*self.specs
    }
}
