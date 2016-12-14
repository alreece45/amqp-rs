// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

///
/// Define getters and setters for a struct. Non-stable until lifetime support is added
/// (currently only supports 'a)
///
/// Expects to be given the name of functions, and the return value, e.g:
///
/// `([list of function names`] -> ReturnType
///
/// The number of functions names needed varies, depending on the return type.
///
/// * For copy types `(name|getter, setter) -> Ty`
/// * Borrowed types require a mutator method: `(name|getter, mutator, setters) -> &Ty`
/// * Cow types are detected, don't need a lifetime: `(...) -> Cow<SomeType>`
/// * Option types may require spacing, and also require a taker name: `(name|getter, mutator, setters, taker) -> Option<&Ty>`
/// * Option types that don't return a reference still require a mutator:  `(name|getter, mutator, setters, taker) -> Option<u8>`
/// * Option may require spacing: `(...) -> Option< Cow<SomeType> >`
///
/// # Examples
///
/// By-value copy types (doesn't return a reference, no get_mut())
///
/// ```
/// #[macro_use]
/// extern crate amqp0_primitives;
///
/// use std::borrow::Cow;
///
/// #[derive(Clone)]
/// struct Member;
///
/// struct TestCopy<'a> {
///     val: u8,
///     option_val: Option<u8>,
///     member: Member,
///     option_member: Option<Member>,
///     cow: Cow<'a, Member>,
///     option_cow: Option<Cow<'a, Member>>,
/// }
///
/// impl<'a> TestCopy<'a> {
///     impl_properties! {
///         // copy-values (no reference)
///         (val, set_val) -> u8,
///         // optional copy-values
///         (option_val, mutate_option_val, set_option_val, take_option_val) -> Option<u8>,
///         // reference
///         (member, member_mut, set_member) -> &Member,
///         // optional reference
///         (option_member, option_member_mut, set_option_member, take_option_member) -> Option<&Member>,
///         // Cow
///         (cow, cow_mut, set_cow) -> Cow<Member>,
///         // Optional Cow -- note the spacing around the `Option` <>
///         (option_cow, option_cow_mut, set_option_cow, take_option_cow_mut) -> Option< Cow<Member> >
///     }
/// }
///
/// fn main() {}
/// ```
///
/// # Cow notes
///
/// `Cow`s that return a different type for their mutable reference will need to make note on the
/// mutator method. This is automatically done for `str and `slices` e.g:
///
/// Cow<str> and Cow<Vec<_>> are special cases -- mutable references are different and owned. They
/// should be detected automatically, e.g:
///
/// ```
/// #[macro_use]
/// extern crate amqp0_primitives;
///
/// use std::borrow::{Borrow, Cow};
///
/// struct Borrowed(u8);
/// struct Owned(Borrowed);
///
/// impl Borrow<Borrowed> for Owned {
///     fn borrow(&self) -> &Borrowed {
///         &self.0
///     }
/// }
///
/// impl ToOwned for Borrowed {
/// type Owned = Owned;
///     fn to_owned(&self) -> Owned {
///         Owned(Borrowed(self.0))
///     }
/// }
///
/// struct TestCow<'a> {
///     string: Cow<'a, str>,
///     bytes: Cow<'a, [u8]>,
///     borrow: Cow<'a, Borrowed>,
/// }
///
/// impl<'a> TestCow<'a> {
///     impl_properties! {
///         // strings and slices are automatically detected, no need to do -> String or -> Vec<_>
///         (string, string_mut, set_string) -> Cow<str>,
///         (bytes, bytes_mut, set_bytes) -> Cow<[u8]>,
///         // other types require it to be set
///         (borrow, borrow_mut -> Owned, set_borrow) -> Cow<Borrowed>
///     }
/// }
///
/// fn main() {}
/// ```
///
/// # FAQ
///
/// Error: "only traits may use parentheses" or "undefined or not in scope"
///
/// Ensure you are using commas
///
#[macro_export]
macro_rules! impl_properties {
    () => {};

    // Option<Cow<_>> (different owned type)
    (($property:ident, $mutator:ident -> $owned:ty, $setter:ident, $take:ident) -> Option< Cow<$ty:ty> >) => {
        pub fn $property(&self) -> Option<&$ty> {
            self.$property.as_ref().map(|v| &**v)
        }

        pub fn $setter<O, V>(&mut self, value: O)
            where O: Into<Option<V>>,
                  V: Into<::std::borrow::Cow<'a, $ty>>
        {
            self.$property = value.into().map(|v| v.into());
        }

        pub fn $mutator(&mut self) -> Option<&mut $owned> {
            self.$property.as_mut()
                .map(|v| v.to_mut())
        }

        pub fn $take(&mut self) -> Option<::std::borrow::Cow<'a, $ty>> {
            self.$property.take()
        }
    };
    (($property:ident, $mutator:ident -> $owned:ty, $setter:ident, $take:ident) -> Option< Cow<$ty:ty> >, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> $owned, $setter, $take) -> Option< Cow< $ty> >);
        impl_properties!($($rest)*);
    };

    // Option<Cow<[_]>> (automatically notes that owned type is Vec<_>
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option< Cow<[$ty:ty]> >) => {
        impl_properties!(($property, $mutator -> Vec<$ty>, $setter, $take) -> Option< Cow<[$ty]> >);
    };
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option< Cow<[$ty:ty]> >, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> Vec<$ty>, $setter, $take) -> Option< Cow<[$ty]> >);
        impl_properties!($($rest)*);
    };

    // Option<Cow<str>> (automatically notes that owned type is String
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option< Cow<str> >) => {
        impl_properties!(($property, $mutator -> ::std::string::String, $setter, $take) -> Option< Cow<str> >);
    };
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option< Cow<str> >, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> ::std::string::String, $setter, $take) -> Option< Cow<str> >);
        impl_properties!($($rest)*);
    };

    // Option<Cow<_>> same owned type
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option< Cow<$ty:ty> >) => {
        impl_properties!(($property, $mutator -> $ty, $setter, $take) -> Option< Cow<$ty> >);
    };
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option< Cow<$ty:ty> >, $($rest:tt)*) => {
        impl_properties!(($property, $mutator, $setter, $take) -> Option< Cow< $ty> >);
        impl_properties!($($rest)*);
    };

    // Cow<_> (different owned type)
    (($property:ident, $mutator:ident -> $owned:ty, $setter:ident) -> Cow<$ty:ty>) => {
        pub fn $property(&self) -> &$ty {
            &*self.$property
        }

        pub fn $setter<V>(&mut self, value: V)
            where V: Into<::std::borrow::Cow<'a, $ty>>
        {
            self.$property = value.into()
        }

        pub fn $mutator(&mut self) -> &mut $owned {
            self.$property.to_mut()
        }
    };
    (($property:ident, $mutator:ident -> $owned:ty, $setter:ident) -> Cow<$ty:ty>, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> $owned, $setter) -> Cow<$ty>);
        impl_properties!($($rest)*);
    };

    // Cow<[_]> (automatically notes that owned type is Vec<_>
    (($property:ident, $mutator:ident, $setter:ident) -> Cow<[$ty:ty]>) => {
        impl_properties!(($property, $mutator -> Vec<$ty>, $setter) -> Cow<[$ty]>);
    };
    (($property:ident, $mutator:ident, $setter:ident) -> Cow<[$ty:ty]>, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> Vec<$ty>, $setter) -> Cow<[$ty]>);
        impl_properties!($($rest)*);
    };

    // Cow<str> (automatically notes that owned type is String
    (($property:ident, $mutator:ident, $setter:ident) -> Cow<str>) => {
        impl_properties!(($property, $mutator -> ::std::string::String, $setter) -> Cow<str>);
    };
    (($property:ident, $mutator:ident, $setter:ident) -> Cow<str>, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> ::std::string::String, $setter) -> Cow<str>);
        impl_properties!($($rest)*);
    };

    // Cow<_> same owned type
    (($property:ident, $mutator:ident, $setter:ident) -> Cow<$ty:ty>) => {
        impl_properties!(($property, $mutator -> $ty, $setter) -> Cow<$ty>);
    };
    (($property:ident, $mutator:ident, $setter:ident) -> Cow<$ty:ty>, $($rest:tt)*) => {
        impl_properties!(($property, $mutator -> $ty, $setter) -> Cow<$ty>);
        impl_properties!($($rest)*);
    };

    // Option
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option<&$ty:ty>) => {
        pub fn $property(&self) -> Option<&$ty> {
            self.$property.as_ref()
        }

        pub fn $setter<O>(&mut self, value: O)
            where O: Into<Option<$ty>>
        {
            self.$property = value.into();
        }

        pub fn $mutator(&mut self) -> Option<&mut $ty> {
            self.$property.as_mut()
        }

        pub fn $take(&mut self) -> Option<$ty> {
            self.$property.take()
        }
    };
    (($property:ident, $mutator:ident, $setter:ident, $take:ident) -> Option<&$ty:ty>, $($rest:tt)*) => {
        impl_properties!(($property, $mutator, $setter, $take) -> Option<&$ty>);
        impl_properties!($($rest)*);
    };

    // by reference
    (($property:ident, $mutator:ident, $setter:ident) -> &$ty:ty) => {
        pub fn $property(&self) -> &$ty {
            &self.$property
        }
        pub fn $mutator(&mut self) -> &mut $ty {
            &mut self.$property
        }
        pub fn $setter(&mut self, value: $ty) {
            self.$property = value;
        }
    };
    (($property:ident, $mutator:ident, $setter:ident) -> &$ty:ty, $($rest:tt)*) => {
        impl_properties!(($property, $mutator, $setter) -> &$ty);
        impl_properties!($($rest)*);
    };

    // option by value
    (($property:ident, $mutator:ident, $setter:ident, $taker:ident) -> Option<$ty:ty>) => {
        pub fn $property(&self) -> Option<$ty> {
            self.$property
        }

        pub fn $setter<O>(&mut self, value: O)
            where O: Into<Option<$ty>>
        {
            self.$property = value.into();
        }

        pub fn $mutator(&mut self) -> Option<&mut $ty> {
            self.$property.as_mut()
        }

        pub fn $taker(&mut self) -> Option<$ty> {
            self.$property.take()
        }
    };
    (($property:ident, $mutator:ident, $setter:ident, $taker:ident) -> Option<$ty:ty>, $($rest:tt)*) => {
        impl_properties!(($property, $mutator, $setter, $taker) -> Option<$ty>);
        impl_properties!($($rest)*);
    };

    // by value
    (($property:ident, $setter:ident) -> $ty:ty) => {
        pub fn $property(&self) -> $ty {
            self.$property
        }
        pub fn $setter(&mut self, value: $ty) {
            self.$property = value;
        }
    };
    (($property:ident, $setter:ident) -> $ty:ty, $($rest:tt)*) => {
        impl_properties!(($property, $setter) -> $ty);
        impl_properties!($($rest)*);
    };
}