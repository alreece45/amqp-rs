// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

///
/// Define getters and setters for a struct.
///
/// # FAQ
///
/// Error: "only traits may use parentheses" or "undefined or not in scope"
///
/// Ensure you are using commas
///
///
/// # Examples
///
/// By-value copy types (doesn't return a reference, no get_mut())
///
/// ```
/// struct TestCopy { val}
///
/// impl TestCopy {
///     impl_properties!(
///         (val, set_val) -> u8,
///     )
/// }
/// ```
///
/// By-reference values (owned structs)
///
/// ```
/// struct Member;
/// struct TestRef { member: Member }
///
/// impl TestRef {
///     impl_properties!(
///         (member, member_mut, set_member) -> &Member
///     )
/// }
/// ```
///
/// By-reference `Option`s (owned options)
///
/// ```
/// struct Member;
/// struct TestRef { member: Option<Member> }
///
/// impl TestRef {
///     impl_properties!(
///         (member, member_mut, set_member) -> Option<&Member>
///     )
/// }
/// ```
///
/// `Cow`s (where ToOwned::Target == Self)
/// ```
/// #[derive(Clone)]
/// struct Cloneable;
/// struct TestRef { member: Cow<'a, Member> }
///
/// // currently only works with 'a
/// impl TestRef<'a> {
///     impl_properties!(
///         (member, member_mut, set_member) -> Cow<Member>
///     )
/// }
/// ```
///
/// `Cow`s (where ToOwned::Target != Self)
/// ```
/// #[derive(Clone)]
/// struct Cloneable;
/// struct TestRef { member: Cow<'a, Member> }
///
/// // currently only works with 'a
/// impl TestRef<'a> {
///     impl_properties!(
///         (member, member_mut -> Owned, set_member) -> Cow<Borrowed>
///     )
/// }
/// ```
///
/// `Option<Cow<_>>`'s (add -> Owned if ToOwned::Target != Self
///
///
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