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
        SpecField::Manual(ref name, _) => name,
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
    Manual(&'static str, bool),
}

#[derive(Debug, Clone)]
pub struct Field {
    field: SpecField,
    var_name: Rc<String>,
    ty: Domain,
}

impl Field {
    pub fn new(name: &'static str, ty: Domain, is_reserved: bool) -> Self {
        Field {
            field: SpecField::Manual(name, is_reserved),
            var_name: Rc::new(name.to_snake_case()),
            ty: ty,
        }
    }

    pub fn from_field<T>(field: T, ty: Domain) -> Self
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

    pub fn is_optional_property(&self) -> bool {
        if self.ty == Domain::Content || self.ty == Domain::Table {
            return false;
        }

        match self.field {
            SpecField::Class(_) => true,
            _ => true,
        }
    }

    pub fn is_reserved(&self) -> bool {
        match self.field {
            SpecField::ClassMethod(ref field) => field.is_reserved(),
            SpecField::Class(ref field) => field.name() == "reserved",
            SpecField::Manual(_, is_reserved) => is_reserved,
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
