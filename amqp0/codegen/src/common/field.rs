// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Deref;
use std::rc::Rc;
use inflections::Inflect;

use specs::{ClassField, ClassMethodField};
use common::domain::Domain;

#[derive(Debug, Clone)]
pub struct Field<T>
    where T: BasicField
{
    field: T,
    name: &'static str,
    var_name: Rc<String>,
    ty: Domain,
}

impl<T> Field<T>
    where T: BasicField
{
    pub fn from_amqp0_field(
        field: T,
        ty: Domain
    ) -> Self {
        let name = match field.name() {
            "type" => "ty".into(),
            "nowait" => "no-wait",
            name => name,
        };

        Field {
            field: field,
            name: name,
            var_name: Rc::new(name.to_snake_case()),
            ty: ty,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn var_name(&self) -> &Rc<String> {
        &self.var_name
    }

    pub fn ty(&self) -> &Domain {
        &self.ty
    }
}

impl<T> Deref for Field<T>
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
    fn name(&self) -> &'static str {
        self.name()
    }
}

impl BasicField for ClassField {
    fn name(&self) -> &'static str {
        self.name()
    }
}