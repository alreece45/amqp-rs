
mod copy {
    mod direct {
        struct CopyStruct {
            val1: u8,
            val2: u8,
            val3: u8
        }

        impl CopyStruct {
            impl_properties!(
                (val1, set_val1) -> u8,
                (val2, set_val2) -> u8
            );
            impl_properties!((val3, set_val3) -> u8);
        }

        #[test]
        fn getters() {
            let test = CopyStruct { val1: 1, val2: 2, val3: 3 };
            assert_eq!(test.val1(), 1);
            assert_eq!(test.val2(), 2);
            assert_eq!(test.val3(), 3);
        }

        #[test]
        fn setters() {
            let mut test = CopyStruct { val1: 0, val2: 0, val3: 0 };
            let test = &mut test;
            test.set_val1(4);
            test.set_val2(5);
            test.set_val3(6);

            assert_eq!(test.val1(), 4);
            assert_eq!(test.val2(), 5);
            assert_eq!(test.val3(), 6);
        }
    }

    mod option {
        struct CopyOptionStruct {
            val1: Option<u8>,
            val2: Option<u8>,
            val3: Option<u8>
        }

        impl CopyOptionStruct {
            impl_properties!(
                (val1, val1_mut, set_val1, take_val1) -> Option<u8>,
                (val2, val2_mut, set_val2, take_val2) -> Option<u8>
            );
            impl_properties!((val3, val3_mut, set_val3, take_val3) -> Option<u8>);
        }

        #[test]
        fn getters() {
            let test = CopyOptionStruct { val1: Some(1), val2: Some(2), val3: Some(3) };
            assert_eq!(test.val1(), Some(1));
            assert_eq!(test.val2(), Some(2));
            assert_eq!(test.val3(), Some(3));
        }

        #[test]
        fn setters() {
            let mut test = CopyOptionStruct { val1: None, val2: None, val3: None };

            {
                let test_ref = &mut test;
                test_ref.set_val1(Some(4));
                test_ref.set_val2(Some(5));
                test_ref.set_val3(Some(6));
            }

            assert_eq!(test.val1(), Some(4));
            assert_eq!(test.val2(), Some(5));
            assert_eq!(test.val3(), Some(6));
        }

        #[test]
        fn mutators() {
            let mut test = CopyOptionStruct { val1: Some(0), val2: Some(0), val3: Some(0) };

            {
                let test_ref = &mut test;
                *test_ref.val1_mut().unwrap() = 7;
                *test_ref.val2_mut().unwrap() = 8;
                *test_ref.val3_mut().unwrap() = 9;
            }

            assert_eq!(test.val1(), Some(7));
            assert_eq!(test.val2(), Some(8));
            assert_eq!(test.val3(), Some(9));
        }

        #[test]
        fn takers() {
            let mut test = CopyOptionStruct { val1: Some(10), val2: Some(11), val3: Some(12) };

            {
                let test_ref = &mut test;
                assert_eq!(test_ref.take_val1(), Some(10));
                assert_eq!(test_ref.take_val2(), Some(11));
                assert_eq!(test_ref.take_val3(), Some(12));
            }

            assert_eq!(test.val1(), None);
            assert_eq!(test.val2(), None);
            assert_eq!(test.val3(), None);
        }
    }
}

mod reference {
    #[derive(Debug, PartialEq)]
    struct RefMember(u8);

    mod direct {
        use super::RefMember;

        struct RefTest {
            member1: RefMember,
            member2: RefMember,
            member3: RefMember
        }

        impl RefTest {
            impl_properties!(
            (member1, member1_mut, set_member1) -> &RefMember,
            (member2, member2_mut, set_member2) -> &RefMember,
        );
            impl_properties!((member3, member3_mut, set_member3) -> &RefMember);
        }

        #[test]
        fn getters() {
            let test1 = RefTest {
                member1: RefMember(1),
                member2: RefMember(2),
                member3: RefMember(3)
            };
            assert_eq!(test1.member1(), &RefMember(1));
            assert_eq!(test1.member2(), &RefMember(2));
            assert_eq!(test1.member3(), &RefMember(3))
        }

        #[test]
        fn mutators() {
            let mut test2 = RefTest {
                member1: RefMember(0),
                member2: RefMember(0),
                member3: RefMember(0)
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut() = RefMember(4);
                *test2_ref.member2_mut() = RefMember(5);
                *test2_ref.member3_mut() = RefMember(6);
            }
            assert_eq!(test2.member1(), &RefMember(4));
            assert_eq!(test2.member2(), &RefMember(5));
            assert_eq!(test2.member3(), &RefMember(6));
        }

        #[test]
        fn setters() {
            let mut test2 = RefTest {
                member1: RefMember(0),
                member2: RefMember(0),
                member3: RefMember(0)
            };
            {
                let test2_ref = &mut test2;
                test2_ref.set_member1(RefMember(7));
                test2_ref.set_member2(RefMember(8));
                test2_ref.set_member3(RefMember(9));
            }
            assert_eq!(test2.member1(), &RefMember(7));
            assert_eq!(test2.member2(), &RefMember(8));
            assert_eq!(test2.member3(), &RefMember(9));
        }
    }

    mod option {
        use super::RefMember;
        struct TestOption {
            member1: Option<RefMember>,
            member2: Option<RefMember>,
            member3: Option<RefMember>
        }

        impl TestOption {
            impl_properties!(
            (member1, member1_mut, set_member1, take_member1) -> Option<&RefMember>,
            (member2, member2_mut, set_member2, take_member2) -> Option<&RefMember>,
        );
            impl_properties!((member3, member3_mut, set_member3, take_member3) -> Option<&RefMember>);
        }

        #[test]
        fn getters() {
            let test1 = TestOption {
                member1: Some(RefMember(1)),
                member2: Some(RefMember(2)),
                member3: Some(RefMember(3))
            };
            assert_eq!(test1.member1(), Some(&RefMember(1)));
            assert_eq!(test1.member2(), Some(&RefMember(2)));
            assert_eq!(test1.member3(), Some(&RefMember(3)));
        }
        #[test]
        fn mutators() {
            let mut test2 = TestOption {
                member1: Some(RefMember(0)),
                member2: Some(RefMember(0)),
                member3: Some(RefMember(0))
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut().unwrap() = RefMember(4);
                *test2_ref.member2_mut().unwrap() = RefMember(5);
                *test2_ref.member3_mut().unwrap() = RefMember(6);
            }

            assert_eq!(test2.member1(), Some(&RefMember(4)));
            assert_eq!(test2.member2(), Some(&RefMember(5)));
            assert_eq!(test2.member3(), Some(&RefMember(6)));
        }

        #[test]
        fn setters() {
            let mut test2 = TestOption {
                member1: Some(RefMember(0)),
                member2: Some(RefMember(0)),
                member3: Some(RefMember(0))
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1(Some(RefMember(7)));
                test2_ref.set_member2(Some(RefMember(8)));
                test2_ref.set_member3(Some(RefMember(9)));
            }

            assert_eq!(test2.member1(), Some(&RefMember(7)));
            assert_eq!(test2.member2(), Some(&RefMember(8)));
            assert_eq!(test2.member3(), Some(&RefMember(9)));
        }

        #[test]
        fn takers() {
            let mut test2 = TestOption {
                member1: Some(RefMember(10)),
                member2: Some(RefMember(11)),
                member3: Some(RefMember(12))
            };

            {
                let test2_ref = &mut test2;
                assert_eq!(test2_ref.take_member1(), Some(RefMember(10)));
                assert_eq!(test2_ref.take_member2(), Some(RefMember(11)));
                assert_eq!(test2_ref.take_member3(), Some(RefMember(12)));
            }

            assert_eq!(test2.member1(), None);
            assert_eq!(test2.member2(), None);
            assert_eq!(test2.member3(), None);

        }
    }
}

mod cow_to_owned {
    use std::borrow::{Borrow, Cow};

    #[derive(Debug, PartialEq)]
    struct Borrowed(u8);
    #[derive(Debug, PartialEq)]
    struct Owned(Borrowed);

    impl Borrow<Borrowed> for Owned {
        fn borrow(&self) -> &Borrowed {
            &self.0
        }
    }

    impl ToOwned for Borrowed {
        type Owned = Owned;
        fn to_owned(&self) -> Owned {
            Owned(Borrowed(self.0))
        }
    }

    impl<'a> From<Owned> for Cow<'a, Borrowed> {
        fn from(owned: Owned) -> Self {
            Cow::Owned(owned)
        }
    }

    mod direct {
        use std::borrow::Cow;
        use super::{Borrowed, Owned};

        struct CowDirectStruct<'a> {
            member1: Cow<'a, Borrowed>,
            member2: Cow<'a, Borrowed>,
            member3: Cow<'a, Borrowed>
        }

        impl<'a> CowDirectStruct<'a> {
            impl_properties!(
                (member1, member1_mut -> Owned, set_member1) -> Cow<Borrowed>,
                (member2, member2_mut -> Owned, set_member2) -> Cow<Borrowed>,
            );
            impl_properties!((member3, member3_mut -> Owned, set_member3) -> Cow<Borrowed>);
        }

        #[test]
        fn getters() {
            let test1 = CowDirectStruct {
                member1: Cow::Owned(Owned(Borrowed(1))),
                member2: Cow::Owned(Owned(Borrowed(2))),
                member3: Cow::Owned(Owned(Borrowed(3)))
            };
            assert_eq!(test1.member1(), &Borrowed(1));
            assert_eq!(test1.member2(), &Borrowed(2));
            assert_eq!(test1.member3(), &Borrowed(3));
        }

        #[test]
        fn mutators() {
            let borrowed = Borrowed(0);
            let mut test2 = CowDirectStruct {
                member1: Cow::Borrowed(&borrowed),
                member2: Cow::Borrowed(&borrowed),
                member3: Cow::Borrowed(&borrowed)
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut() = Owned(Borrowed(4));
                *test2_ref.member2_mut() = Owned(Borrowed(5));
                *test2_ref.member3_mut() = Owned(Borrowed(6));
            }

            assert_eq!(test2.member1(), &Borrowed(4));
            assert_eq!(test2.member2(), &Borrowed(5));
            assert_eq!(test2.member3(), &Borrowed(6));
        }

        #[test]
        fn setters() {
            let borrowed = Borrowed(0);
            let mut test2 = CowDirectStruct {
                member1: Cow::Borrowed(&borrowed),
                member2: Cow::Borrowed(&borrowed),
                member3: Cow::Borrowed(&borrowed)
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1(Owned(Borrowed(7)));
                test2_ref.set_member2(Owned(Borrowed(8)));
                test2_ref.set_member3(Owned(Borrowed(9)));
            }

            assert_eq!(test2.member1(), &Borrowed(7));
            assert_eq!(test2.member2(), &Borrowed(8));
            assert_eq!(test2.member3(), &Borrowed(9));
        }
    }

    mod option {
        use std::borrow::Cow;
        use super::{Borrowed, Owned};

        struct TestOptionCowDifferent<'a> {
            member1: Option<Cow<'a, Borrowed>>,
            member2: Option<Cow<'a, Borrowed>>,
            member3: Option<Cow<'a, Borrowed>>
        }

        impl<'a> TestOptionCowDifferent<'a> {
            impl_properties!(
            (member1, member1_mut -> Owned, set_member1, take_member1) -> Option< Cow<Borrowed> >,
            (member2, member2_mut -> Owned, set_member2, take_member2) -> Option< Cow<Borrowed> >,
        );
            impl_properties!((member3, member3_mut -> Owned, set_member3, take_member3) -> Option< Cow<Borrowed> >);
        }

        #[test]
        fn getters() {
            let test1 = TestOptionCowDifferent {
                member1: Some(Cow::Owned(Owned(Borrowed(1)))),
                member2: Some(Cow::Owned(Owned(Borrowed(2)))),
                member3: Some(Cow::Owned(Owned(Borrowed(3))))
            };
            assert_eq!(test1.member1(), Some(&Borrowed(1)));
            assert_eq!(test1.member2(), Some(&Borrowed(2)));
            assert_eq!(test1.member3(), Some(&Borrowed(3)));
        }

        #[test]
        fn mutators() {
            let borrowed = Borrowed(0);
            let mut test2 = TestOptionCowDifferent {
                member1: Some(Cow::Owned(Owned(Borrowed(100)))),
                member2: Some(Cow::Owned(Owned(Borrowed(255)))),
                member3: Some(Cow::Borrowed(&borrowed)),
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut().unwrap() = Owned(Borrowed(4));
                *test2_ref.member2_mut().unwrap() = Owned(Borrowed(5));
                *test2_ref.member3_mut().unwrap() = Owned(Borrowed(6));
            }

            assert_eq!(test2.member1(), Some(&Borrowed(4)));
            assert_eq!(test2.member2(), Some(&Borrowed(5)));
            assert_eq!(test2.member3(), Some(&Borrowed(6)));

        }

        #[test]
        fn setters() {
            let borrowed = Borrowed(0);
            let mut test2 = TestOptionCowDifferent {
                member1: Some(Cow::Owned(Owned(Borrowed(100)))),
                member2: Some(Cow::Owned(Owned(Borrowed(255)))),
                member3: Some(Cow::Borrowed(&borrowed)),
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1(Owned(Borrowed(7)));
                test2_ref.set_member2(Owned(Borrowed(8)));
                test2_ref.set_member3(Owned(Borrowed(9)));
            }
            assert_eq!(test2.member1(), Some(&Borrowed(7)));
            assert_eq!(test2.member2(), Some(&Borrowed(8)));
            assert_eq!(test2.member3(), Some(&Borrowed(9)));
        }

        #[test]
        fn takers() {
            let mut test2 = TestOptionCowDifferent {
                member1: Some(Cow::Owned(Owned(Borrowed(10)))),
                member2: Some(Cow::Owned(Owned(Borrowed(11)))),
                member3: Some(Cow::Owned(Owned(Borrowed(12)))),
            };

            {
                let test2_ref = &mut test2;
                assert_eq!(test2_ref.take_member1(), Some(Cow::Owned(Owned(Borrowed(10)))));
                assert_eq!(test2_ref.take_member2(), Some(Cow::Owned(Owned(Borrowed(11)))));
                assert_eq!(test2_ref.take_member3(), Some(Cow::Owned(Owned(Borrowed(12)))));
            }

            assert_eq!(test2.member1(), None);
            assert_eq!(test2.member2(), None);
            assert_eq!(test2.member3(), None);
        }
    }
}

mod cow_str {
    mod direct {
        use std::borrow::Cow;

        struct TestCowStr<'a> {
            member1: Cow<'a, str>,
            member2: Cow<'a, str>,
            member3: Cow<'a, str>
        }

        impl<'a> TestCowStr<'a> {
            impl_properties!(
                (member1, member1_mut, set_member1) -> Cow<str>,
                (member2, member2_mut, set_member2) -> Cow<str>,
            );
            impl_properties!(
                (member3, member3_mut, set_member3) -> Cow<str>,
            );
        }

        #[test]
        fn getters() {
            let test1 = TestCowStr {
                member1: Cow::Owned("Test1".to_owned()),
                member2: Cow::Owned("Test2".to_owned()),
                member3: Cow::Owned("Test3".to_owned())
            };
            assert_eq!(test1.member1(), "Test1");
            assert_eq!(test1.member2(), "Test2");
            assert_eq!(test1.member3(), "Test3");
        }

        #[test]
        fn mutators() {
            let mut test2 = TestCowStr {
                member1: Cow::Owned("Test0".to_owned()),
                member2: Cow::Owned("Test0".to_owned()),
                member3: Cow::Owned("Test0".to_owned())
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut() = "Test4".to_owned();
                *test2_ref.member2_mut() = "Test5".to_owned();
                *test2_ref.member3_mut() = "Test6".to_owned();
            }

            assert_eq!(test2.member1(), "Test4");
            assert_eq!(test2.member2(), "Test5");
            assert_eq!(test2.member3(), "Test6");
        }


        #[test]
        fn setters() {
            let mut test2 = TestCowStr {
                member1: Cow::Owned("Test0".to_owned()),
                member2: Cow::Owned("Test0".to_owned()),
                member3: Cow::Owned("Test0".to_owned())
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1("Test7");
                test2_ref.set_member2("Test8");
                test2_ref.set_member3("Test9");
            }

            assert_eq!(test2.member1(), "Test7");
            assert_eq!(test2.member2(), "Test8");
            assert_eq!(test2.member3(), "Test9");
        }
    }
    mod option {

        use std::borrow::Cow;

        struct TestCowStr<'a> {
            member1: Option<Cow<'a, str>>,
            member2: Option<Cow<'a, str>>,
            member3: Option<Cow<'a, str>>,
        }

        impl<'a> TestCowStr<'a> {
            impl_properties!(
            (member1, member1_mut, set_member1, take_member1) -> Option< Cow<str> >,
            (member2, member2_mut, set_member2, take_member2) -> Option< Cow<str> >,
        );
            impl_properties!(
            (member3, member3_mut, set_member3, take_member3) -> Option< Cow<str> >,
        );
        }

        #[test]
        fn getters() {
            let test1 = TestCowStr {
                member1: Some(Cow::Owned("Test1".to_owned())),
                member2: Some(Cow::Owned("Test2".to_owned())),
                member3: Some(Cow::Owned("Test3".to_owned()))
            };
            assert_eq!(test1.member1(), Some("Test1"));
            assert_eq!(test1.member2(), Some("Test2"));
            assert_eq!(test1.member3(), Some("Test3"));
        }

        #[test]
        fn mutators() {
            let mut test2 = TestCowStr {
                member1: Some(Cow::Owned("Test0".to_owned())),
                member2: Some(Cow::Owned("Test0".to_owned())),
                member3: Some(Cow::Owned("Test0".to_owned()))
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut().unwrap() = "Test4".to_owned();
                *test2_ref.member2_mut().unwrap() = "Test5".to_owned();
                *test2_ref.member3_mut().unwrap() = "Test6".to_owned();
            }

            assert_eq!(test2.member1(), Some("Test4"));
            assert_eq!(test2.member2(), Some("Test5"));
            assert_eq!(test2.member3(), Some("Test6"));
        }

        #[test]
        fn setters() {
            let mut test2 = TestCowStr {
                member1: Some(Cow::Owned("Test0".to_owned())),
                member2: Some(Cow::Owned("Test0".to_owned())),
                member3: Some(Cow::Owned("Test0".to_owned()))
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1("Test7");
                test2_ref.set_member2("Test8");
                test2_ref.set_member3("Test9");
            }
            assert_eq!(test2.member1(), Some("Test7"));
            assert_eq!(test2.member2(), Some("Test8"));
            assert_eq!(test2.member3(), Some("Test9"))

        }

        #[test]
        fn takers() {
            let mut test2 = TestCowStr {
                member1: Some(Cow::Owned("Test10".to_owned())),
                member2: Some(Cow::Owned("Test11".to_owned())),
                member3: Some(Cow::Owned("Test12".to_owned()))
            };

            {
                let test2_ref = &mut test2;
                assert_eq!(test2_ref.take_member1(), Some(Cow::Borrowed("Test10")));
                assert_eq!(test2_ref.take_member2(), Some(Cow::Borrowed("Test11")));
                assert_eq!(test2_ref.take_member3(), Some(Cow::Borrowed("Test12")));
            }

            assert_eq!(test2.member1(), None);
            assert_eq!(test2.member2(), None);
            assert_eq!(test2.member3(), None);

        }
    }
}

mod cow_clone {
    #[derive(Clone, Debug, PartialEq)]
    struct Member(u8);

    mod direct{
        use std::borrow::Cow;
        use super::Member;

        struct TestOptionCow<'a> {
            member1: Cow<'a, Member>,
            member2: Cow<'a, Member>,
            member3: Cow<'a, Member>
        }

        #[test]
        fn getters() {
            impl<'a> TestOptionCow<'a> {
                impl_properties!(
                    (member1, member1_mut, set_member1) -> Cow<Member>,
                    (member2, member2_mut, set_member2) -> Cow<Member>,
                );
                impl_properties!((member3, member3_mut, set_member3) -> Cow<Member>);
            }

            let test1 = TestOptionCow {
                member1: Cow::Owned(Member(1)),
                member2: Cow::Owned(Member(2)),
                member3: Cow::Owned(Member(3))
            };

            assert_eq!(test1.member1(), &Member(1));
            assert_eq!(test1.member2(), &Member(2));
            assert_eq!(test1.member3(), &Member(3));
        }

        #[test]
        fn mutators() {
            let borrowed = Member(0);
            let mut test2 = TestOptionCow {
                member1: Cow::Owned(Member(100)),
                member2: Cow::Owned(Member(255)),
                member3: Cow::Borrowed(&borrowed),
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut() = Member(4);
                *test2_ref.member2_mut() = Member(5);
                *test2_ref.member3_mut() = Member(6);
            }

            assert_eq!(test2.member1(), &Member(4));
            assert_eq!(test2.member2(), &Member(5));
            assert_eq!(test2.member3(), &Member(6));
        }

        #[test]
        fn setters() {
            let borrowed = Member(0);
            let mut test2 = TestOptionCow {
                member1: Cow::Owned(Member(100)),
                member2: Cow::Owned(Member(255)),
                member3: Cow::Borrowed(&borrowed),
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1(Cow::Owned(Member(7)));
                test2_ref.set_member2(Cow::Owned(Member(8)));
                test2_ref.set_member3(Cow::Owned(Member(9)));
            }
            assert_eq!(test2.member1(), &Member(7));
            assert_eq!(test2.member2(), &Member(8));
            assert_eq!(test2.member3(), &Member(9));
        }
    }

    mod option {
        use std::borrow::Cow;
        use super::Member;

        struct TestOptionCow<'a> {
            member1: Option<Cow<'a, Member>>,
            member2: Option<Cow<'a, Member>>,
            member3: Option<Cow<'a, Member>>
        }

        #[test]
        fn getters() {
            impl<'a> TestOptionCow<'a> {
                impl_properties!(
                (member1, member1_mut, set_member1, take_member1) -> Option< Cow<Member> >,
                (member2, member2_mut, set_member2, take_member2) -> Option< Cow<Member> >,
            );
                impl_properties!((member3, member3_mut, set_member3, take_member3) -> Option< Cow<Member> >);
            }

            let test1 = TestOptionCow {
                member1: Some(Cow::Owned(Member(1))),
                member2: Some(Cow::Owned(Member(2))),
                member3: Some(Cow::Owned(Member(3)))
            };

            assert_eq!(test1.member1(), Some(&Member(1)));
            assert_eq!(test1.member2(), Some(&Member(2)));
            assert_eq!(test1.member3(), Some(&Member(3)));
        }

        #[test]
        fn mutators() {
            let borrowed = Member(0);
            let mut test2 = TestOptionCow {
                member1: Some(Cow::Owned(Member(100))),
                member2: Some(Cow::Owned(Member(255))),
                member3: Some(Cow::Borrowed(&borrowed)),
            };

            {
                let test2_ref = &mut test2;
                *test2_ref.member1_mut().unwrap() = Member(4);
                *test2_ref.member2_mut().unwrap() = Member(5);
                *test2_ref.member3_mut().unwrap() = Member(6);
            }

            assert_eq!(test2.member1(), Some(&Member(4)));
            assert_eq!(test2.member2(), Some(&Member(5)));
            assert_eq!(test2.member3(), Some(&Member(6)));
        }

        #[test]
        fn setters() {
            let borrowed = Member(0);
            let mut test2 = TestOptionCow {
                member1: Some(Cow::Owned(Member(100))),
                member2: Some(Cow::Owned(Member(255))),
                member3: Some(Cow::Borrowed(&borrowed)),
            };

            {
                let test2_ref = &mut test2;
                test2_ref.set_member1(Cow::Owned(Member(7)));
                test2_ref.set_member2(Cow::Owned(Member(8)));
                test2_ref.set_member3(Cow::Owned(Member(9)));
            }
            assert_eq!(test2.member1(), Some(&Member(7)));
            assert_eq!(test2.member2(), Some(&Member(8)));
            assert_eq!(test2.member3(), Some(&Member(9)));
        }

        #[test]
        fn takers() {
            let mut test2 = TestOptionCow {
                member1: Some(Cow::Owned(Member(10))),
                member2: Some(Cow::Owned(Member(11))),
                member3: Some(Cow::Owned(Member(12))),
            };

            {
                let test2_ref = &mut test2;
                assert_eq!(test2_ref.take_member1(), Some(Cow::Owned(Member(10))));
                assert_eq!(test2_ref.take_member2(), Some(Cow::Owned(Member(11))));
                assert_eq!(test2_ref.take_member3(), Some(Cow::Owned(Member(12))));
            }

            assert_eq!(test2.member1(), None);
            assert_eq!(test2.member2(), None);
            assert_eq!(test2.member3(), None);
        }
    }
}