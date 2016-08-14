
use std::io;
use inflections::Inflect;

use {Error, WriteRust};
use common::ConstantGroup;
use super::{Protocol, Constant};

impl<'a, 'b: 'a> WriteRust<Protocol<'b>> for ConstantGroup<'a, 'b>
{
    fn write_rust<W>(&self, protocol: &Protocol, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        let constants = self.constants();

        if constants.is_empty() {
            return Ok(());
        }

        let group_name = self.name().to_pascal_case();

        // constants
        try!(constants.iter()
            .map(|c| {
                let name = (&*protocol.map_group_constant_name(self, c)).to_constant_case();
                let ty = self.ty();
                let value = try!(protocol.map_value(&ty, c.value()));
                let constant = Constant::new(name, ty, value);
                constant.write_rust(protocol, writer)
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
        );

        // enum
        try!(writeln!(writer, ""));
        try!(writeln!(writer, "#[derive(Debug, PartialEq, Eq)]"));
        try!(writeln!(writer, "pub enum {} {{", group_name));
        try!(constants.iter()
            .map(|c| protocol.map_group_enum_name(&self, c))
            .map(|c| writeln!(writer, "{},", c))
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
        );

        try!(writeln!(writer, "}}"));
        try!(writeln!(writer, ""));

        // implementation
        try!(writeln!(writer, "impl {} {{", group_name));
        {
            // fn from_id(u16) -> Option<Self>
            try!(writeln!(writer, "pub fn from_id(id: u16) -> Option<Self> {{"));
            {
                try!(writeln!(writer, "match id {{"));
                try!(constants.iter()
                    .map(|c| {
                        let constant_name = (&*protocol.map_group_constant_name(&self, c)).to_constant_case();
                        let enum_name = (&*protocol.map_group_enum_name(self, c)).to_pascal_case();

                        writeln!(writer, "{} => Some({}::{}),", constant_name, group_name, enum_name)
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|_| ()));
                try!(writeln!(writer, "_ => None"));
                try!(writeln!(writer, "}}"));
            }
            try!(writeln!(writer, "}}"));

            // fn id(&self) -> u16
            try!(writeln!(writer, "pub fn id(&self) -> u16 {{"));
            {
                try!(writeln!(writer, "match *self {{"));
                try!(constants.iter()
                    .map(|c| {
                        let constant_name = (&*protocol.map_group_constant_name(&self, c)).to_constant_case();
                        let enum_name = (&*protocol.map_group_enum_name(self, c)).to_pascal_case();

                        writeln!(writer, "{}::{} => {},", group_name, enum_name, constant_name)
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|_| ()));
                try!(writeln!(writer, "}}"));
            }
            try!(writeln!(writer, "}}"));
        }
        try!(writeln!(writer, "}}"));

        // From<_> for u16
        try!(writeln!(writer, "impl From<{}> for u16 {{", group_name));
        {
            try!(writeln!(writer, "fn from(ty: {}) -> u16 {{", group_name));
            try!(writeln!(writer, "ty.id()"));
            try!(writeln!(writer, "}}"));
        }
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}