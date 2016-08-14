
use amqp0::protocol::{Class, Constant, Domain};

#[derive(Debug)]
pub enum Child<'a> {
    Constant(Constant<'a>),
    Domain(Domain<'a>),
    Class(Class<'a>),
}

impl<'a> From<Constant<'a>> for Child<'a> {
    fn from(child: Constant<'a>) -> Self {
        Child::Constant(child)
    }
}

impl<'a> From<Domain<'a>> for Child<'a> {
    fn from(child: Domain<'a>) -> Self {
        Child::Domain(child)
    }
}

impl<'a> From<Class<'a>> for Child<'a> {
    fn from(child: Class<'a>) -> Self {
        Child::Class(child)
    }
}