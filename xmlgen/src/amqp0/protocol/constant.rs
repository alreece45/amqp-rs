
use std::borrow::Cow;
use std::io;

use inflections::Inflect;

use {WriteRust, Error};
use super::Protocol;

pub struct Constant<'a> {
    name: Cow<'a, str>,
    domain: Cow<'a, str>,
    value: Cow<'a, str>,
}

impl<'a> Constant<'a> {
    pub fn new<N, D, V>(name: N, domain: D, value: V) -> Constant<'a>
        where N: Into<Cow<'a, str>>,
              D: Into<Cow<'a, str>>,
              V: Into<Cow<'a, str>>,
    {
        Constant {
            name: name.into(),
            domain: domain.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &*self.name
    }

    pub fn domain(&self) -> &str {
        &*self.domain
    }
    pub fn value(&self) -> &str {
        &*self.value
    }
}

impl<'a> WriteRust<Protocol<'a>> for Constant<'a>
{
    fn write_rust<W>(&self, protocol: &Protocol<'a>, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        let ty = protocol.map_domain(self.domain());
        let value = try!(protocol.map_value(&ty, self.value()));
        let name = self.name().to_constant_case();

        try!(writeln!(writer, "pub const {}: {} = {};", name, ty, value));
        Ok(())
    }
}
