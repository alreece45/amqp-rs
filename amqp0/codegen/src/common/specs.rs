// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::collections::{btree_map, btree_set, BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter;
use std::ops::Deref;

use common::{Spec, Fields};
use lazycell::LazyCell;

pub struct Specs<'a> {
    /// TODO: is Cow<> really the appropriate type, here?
    /// Would AsRef or Borrow be better, since we never take ownership?
    specs: Cow<'a, [Spec]>,
    class_set: LazyCell<BTreeSet<&'static str>>,
    methods: LazyCell<BTreeMap<(&'static str, &'static str), SpecMethod>>,
}

#[derive(Debug)]
pub struct SpecMethod {
    class_name: &'static str,
    method_name: &'static str,
    fields: Fields<'static>,
    has_content: bool,
}

pub struct Methods<'a>(btree_map::Values<'a, (&'a str, &'a str), SpecMethod>);
pub struct ClassMethods<'a>(&'a str, btree_map::Iter<'a, (&'a str, &'a str), SpecMethod>);
pub struct ClassNames<'a>(btree_set::Iter<'a, &'a str>);

impl<'a> Specs<'a> {
    pub fn new<S>(specs: S) -> Self
        where S: Into<Cow<'a, [Spec]>>
    {
        Specs {
            specs: specs.into(),
            class_set: LazyCell::new(),
            methods: LazyCell::new(),
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
            for frame_type in spec.frame_types() {
                let entry = frame_types.entry(frame_type.name()).or_insert((1, frame_type.value() as u8));
                entry.0 += 1;
            }
        }

        frame_types.into_iter()
            .filter(|&(_, v)| v.0 > 1)
            .map(|(k, v)| (k, v.1))
            .collect()
    }

    fn class_set(&self) -> &BTreeSet<&'static str> {
        self.class_set.borrow_with(|| {
            self.specs.iter()
                .flat_map(|spec| spec.classes())
                .map(|class| class.name())
                .collect::<BTreeSet<_>>()
        })
    }

    pub fn class_names(&self) -> ClassNames {
        ClassNames(self.class_set().iter())
    }

    pub fn class_methods<'b>(&'b self, class_name: &'b str) -> ClassMethods<'b> {
        ClassMethods(class_name, self.method_map().iter())
    }

    fn method_map(&self) -> &BTreeMap<(&'static str, &'static str), SpecMethod> {
        self.methods.borrow_with(|| {
            let class_methods = self.specs.iter()
                .flat_map(|spec| spec.classes())
                .flat_map(|class| iter::repeat(class).zip(class.methods()))
                .map(|(class, method)| (class.name(), method.name()))
                .collect::<BTreeSet<_>>();

            class_methods.into_iter()
                .map(|(class, method)| ((class, method), SpecMethod::new(self, class, method)))
                .collect()
        })
    }

    pub fn methods(&self) -> Methods {
        Methods(self.method_map().values())
    }

    pub fn method<'b>(&'b self, class_name: &'b str, method_name: &'b str) -> Option<&SpecMethod> {
        let key = (class_name, method_name);
        self.method_map().get(&key)
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

impl SpecMethod {
    fn new<'a>(specs: &'a Specs<'a>, class_name: &'static str, method_name: &'static str) -> Self {
        let methods = specs.iter()
            .filter_map(|spec| spec.class(class_name))
            .filter_map(|class| class.method(method_name))
            .collect::<Vec<_>>();
        let has_content = methods.iter().all(|method| (*method).has_content());

        let mut fields = Fields::new(methods.iter()
            .flat_map(|method| method.fields())
            .filter(|field| !field.is_reserved()));

        if has_content {
            fields.extend(specs.iter()
                .filter_map(|spec| spec.class(class_name))
                .flat_map(|class| class.fields()))
        }

        SpecMethod {
            class_name: class_name,
            method_name: method_name,
            fields: fields,
            has_content: has_content,
        }
    }

    pub fn class_name(&self) -> &'static str {
        self.class_name
    }

    pub fn method_name(&self) -> &'static str {
        self.method_name
    }

    pub fn has_lifetimes(&self) -> bool {
        self.has_content() || self.fields().has_lifetimes()
    }

    pub fn has_content(&self) -> bool {
        self.has_content
    }

    pub fn has_usable_fields(&self) -> bool {
        !self.fields.is_empty()
    }

    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    pub fn method_traits(&self) -> &'static str {
        if self.has_content() || self.fields().has_lifetimes() {
            "::Encodable + ::Content<'a>"
        } else {
            "::Encodable"
        }
    }
}

use std::slice::Iter;

impl<'a> IntoIterator for &'a Specs<'a> {
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = Iter<'a, Spec>;
    fn into_iter(self) -> Self::IntoIter {
        self.specs.iter()
    }
}

impl<'a> Deref for Specs<'a> {
    type Target = [Spec];

    fn deref(&self) -> &Self::Target {
        &*self.specs
    }
}

impl<'a> Iterator for Methods<'a> {
    type Item = &'a SpecMethod;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a> Iterator for ClassNames<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|name| *name)
    }
}

impl<'a> Iterator for ClassMethods<'a> {
    type Item = &'a SpecMethod;

    fn next(&mut self) -> Option<Self::Item> {
        for (&(class, _), method) in self.1.by_ref() {
            if self.0 == class {
                return Some(method);
            }
        }
        None
    }
}

impl<'a> ExactSizeIterator for Methods<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}