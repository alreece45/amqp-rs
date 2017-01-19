// Copyright 2016-17 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::rc::Rc;
use inflections::Inflect;

use specs::{ClassField, ClassMethodField};
use common::domain::Domain;

fn field_name(spec: &SpecField) -> &'static str {
    let name = match *spec {
        SpecField::ClassMethod(method) => method.name(),
        SpecField::Class(class) => class.name(),
    };

    match name {
        "type" => "ty",
        "nowait" => "no-wait",
        name => name,
    }
}

#[derive(Debug, Clone)]
pub enum SpecField {
    ClassMethod(&'static ClassMethodField),
    Class(&'static ClassField),
}

#[derive(Debug, Clone)]
pub struct Field {
    field: SpecField,
    var_name: Rc<String>,
    ty: Domain,
}

impl Field {
    pub fn new<T>(field: T, ty: Domain) -> Self
        where T: Into<SpecField>
    {
        let field = field.into();
        let name = field_name(&field);
        Field {
            field: field,
            var_name: Rc::new(name.to_snake_case()),
            ty: ty,
        }
    }

    pub fn name(&self) -> &'static str {
        field_name(&self.field)
    }

    pub fn var_name(&self) -> &Rc<String> {
        &self.var_name
    }

    pub fn ty(&self) -> &Domain {
        &self.ty
    }

    pub fn is_reserved(&self) -> bool {
        match self.field {
            SpecField::ClassMethod(ref method) => method.is_reserved(),
            SpecField::Class(_) => false,
        }
    }
}

impl From<&'static ClassField> for SpecField {
    fn from(field: &'static ClassField) -> Self {
        SpecField::Class(field)
    }
}

impl From<&'static ClassMethodField> for SpecField {
    fn from(field: &'static ClassMethodField) -> Self {
        SpecField::ClassMethod(field)
    }
}
