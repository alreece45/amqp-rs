
use std::io;
use inflections::Inflect;

use {Error, WriteRust};
use common::ConstantGroup;
use amqp0::Protocol;
use super::Group;

pub struct Unified<'a>(&'a Group<'a>);

impl<'a> Unified<'a> {
    pub fn new(group: &'a Group) -> Self{
        Unified(group)
    }
}

impl<'a> WriteRust<Protocol<'a>> for Unified<'a> {
    fn write_rust<W>(&self, protocol: &Protocol<'a>, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        // enum Type
        try!(writeln!(writer, "#[derive(Debug, PartialEq, Eq)]"));
        try!(writeln!(writer, "pub enum Type {{"));
        try!(self.0.groups().iter()
            .map(|(response_type, constants)| {
                let name = response_type.name().to_pascal_case();
                if constants.len() > 1 {
                    let module = response_type.name().to_snake_case();
                    try!(writeln!(writer, "{}(self::{}::Type),", name, module));
                }
                else if constants.len() == 1 {
                    try!(writeln!(writer, "{},", name));
                }
                Ok(())
            })
            .collect::<Result<Vec<_>, Error>>()
            .map(|_| ())
        );
        try!(writeln!(writer, "}}"));

        // impl Type
        try!(writeln!(writer, "impl Type {{"));
        {
            // fn from_id(u16) -> Option<Type>
            try!(writeln!(writer, "pub fn from_id(id: u16) -> Option<Type> {{"));
            {
                let empty_vec = vec![];
                try!(writeln!(writer, "match id {{"));
                try!(self.0.groups().iter()
                    .map(|(response_type, constants)| {
                        let response_type_name = response_type.name().to_pascal_case();
                        let group = ConstantGroup::new("ResponseCode", "Type", "u16", empty_vec.iter());
                        constants.iter()
                            .map(|c| {
                                let module = response_type.name().to_snake_case();
                                if constants.len() > 1 {
                                    let constant_name = (&*protocol.map_group_constant_name(&group, c)).to_constant_case();
                                    let constant_name = format!("{}::{}", module, constant_name);
                                    let enum_name = protocol.map_group_enum_name(&group, c);
                                    try!(writeln!(writer, "{} => Some(Type::{}({}::Type::{})),", constant_name, response_type_name, module, enum_name));
                                }
                                else if constants.len() == 1 {
                                    let constant_name =c.name().to_constant_case();
                                    try!(writeln!(writer, "{} => Some(Type::{}),", constant_name, response_type_name));
                                }
                                Ok(())
                            })
                            .collect::<Result<Vec<_>, Error>>()
                            .map(|_| ())
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|_| ())
                );
                try!(writeln!(writer, "_ => None,"));
                try!(writeln!(writer, "}}"))
            }
            try!(writeln!(writer, "}}"));

            // fn error_class(&self) -> Option<ErrorClass>
            try!(writeln!(writer, "pub fn id(&self) -> u16 {{"));
            {
                try!(writeln!(writer, "match *self {{"));
                try!(self.0.groups().iter()
                    .map(|(response_type, constants)| {
                        let response_type_name = response_type.name().to_pascal_case();
                        if constants.len() > 1 {
                            try!(writeln!(writer, "Type::{}(ref ty) => ty.id(),", response_type_name))
                        }
                        else if constants.len() == 1 {
                            let constant_name = constants[0].name().to_constant_case();
                            try!(writeln!(writer, "Type::{} => {},", response_type_name, constant_name))
                        }
                        Ok(())
                    })
                    .collect::<Result<Vec<_>, Error>>()
                    .map(|_| ())
                );
                try!(writeln!(writer, "}}"));
            }
            try!(writeln!(writer, "}}"));

            // fn error_class(&self) -> Option<ErrorClass>
            try!(writeln!(writer, "pub fn error_class(&self) -> Option<ErrorClass> {{"));
            {
                try!(writeln!(writer, "match *self {{"));
                try!(self.0.groups().iter()
                    .map(|(response_type, constants)| {
                        if constants.len() > 0 {
                            let is_all_errors = constants.iter().all(|c| c.class().is_some());
                            let response_type_name = response_type.name().to_pascal_case();
                            if is_all_errors {
                                try!(writeln!(writer, "Type::{}(ref ty) => Some(ty.error_class()),", response_type_name));
                            }
                            else if constants.iter().any(|c| c.class().is_some()) {
                                try!(writeln!(writer, "Type::{}(ref ty) => ty.error_class(),", response_type_name));
                            }
                        }
                        Ok(())
                    })
                    .collect::<Result<Vec<_>, Error>>()
                    .map(|_| ())
                );
                try!(writeln!(writer, "_ => None"));
                try!(writeln!(writer, "}}"))
            }
            try!(writeln!(writer, "}}"));
        }
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}