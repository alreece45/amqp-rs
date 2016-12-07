
use std::ops::Deref;
use inflections::Inflect;

use amqp0::Domain;
use specs::amqp0::{ClassField, ClassMethodField};

pub struct Field<'a, T: 'a>
    where T: BasicField
{
    field: &'a T,
    var_name: String,
    ty: Domain,
}

impl<'a, T> Field<'a, T>
    where T: BasicField
{
    pub fn from_amqp0_field(
        field: &'a T,
        ty: Domain
    ) -> Self {
        let var_name = match field.name() {
            "type" => "ty".into(),
            name => name.to_snake_case().into(),
        };

        Field {
            field: field,
            var_name: var_name,
            ty: ty,
        }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }

    pub fn ty(&self) -> &Domain {
        &self.ty
    }
}

impl<'a, T> Deref for Field<'a, T>
    where T: BasicField
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

pub trait BasicField {
    fn name(&self) -> &'static str;
}

impl BasicField for ClassMethodField {
    fn name(&self) -> &'static str{
        self.name()
    }
}

impl BasicField for ClassField {
    fn name(&self) -> &'static str{
        self.name()
    }
}