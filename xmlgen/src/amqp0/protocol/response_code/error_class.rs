
use std::io;
use std::collections::HashSet;

use {Error, WriteRust};
use amqp0::Protocol;
use super::Group;

pub struct ErrorClass<'a>(&'a Group<'a>);

impl<'a> ErrorClass<'a> {
    pub fn new(group: &'a Group) -> Self {
        ErrorClass(group)
    }
}

impl<'a> WriteRust<Protocol<'a>> for ErrorClass<'a> {
    fn write_rust<W>(&self, protocol: &Protocol, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        let error_types: HashSet<_> = self.0.groups().iter()
            .flat_map(|(_, v)| v)
            .fold(HashSet::new(), |mut set, c| {
                if let Some(class) = c.class() {
                    set.insert(class);
                }
                set
            });

        // trait Error
        try!(writeln!(writer, "pub trait Error {{"));
        try!(writeln!(writer, "fn error_class(&self) -> ErrorClass;"));
        try!(writeln!(writer, "}}"));

        // enum ErrorClass
        try!(writeln!(writer, "#[derive(Debug, PartialEq, Eq)]"));
        try!(writeln!(writer, "pub enum ErrorClass {{"));
        try!(error_types.iter()
            .map(|class| {
                let name = protocol.map_error_class_enum_name(class);
                writeln!(writer, "{},", name)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ()));
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}