
use std::io;
use super::Part;
use primalgen::spec::ModuleWriter;

trait WriteNomImplementation {
    fn write_nom_impl_to(&self, writer: &mut W) -> io::Result<()>;
}

impl WriteNomImplementation for ModuleWriter {
    fn write_nom_impl_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        let parts: Vec<Part> = {
            let mut num_flags = 0;
            let parts: Vec<Part> = Vec::new();
            self.fields.iter()
                .fold(parts, |mut parts, field| {
                    let part_needs_adding = if let Domain::Bit = *field.ty() {
                        let needs_adding = parts.last_mut()
                            .map(|flag| !flag.add_field(field))
                            .unwrap_or(true);

                        if needs_adding {
                            num_flags += 1;
                        }
                        needs_adding
                    }
                        else {
                            true
                        };

                    if part_needs_adding {
                        if let Domain::Bit = *field.ty() {
                            assert_ne!(0, num_flags);
                        }
                        parts.push(Part::from_field(field, num_flags))
                    }
                    parts
                })
        };

        let lifetimes = if self.has_lifetimes { "<'a>" } else { "" };

        try!(writeln!(writer, "use nom::IResult;"));
        try!(writeln!(writer, "use nom::{{be_u8, be_u16, be_u32, be_u64}};\n"));
        try!(writeln!(writer, "impl<'a> ::amqp0::nom::NomBytes<'a> for {}{} {{", self.struct_name, lifetimes));
        try!(writeln!(writer, "fn nom_bytes<'b, P>(input: &'a [u8], pool: &'b mut P) -> IResult<&'a [u8], Self>"));
        try!(writeln!(writer, "    where P: ::amqp0::nom::ParserPool"));
        try!(writeln!(writer, "{{"));
        try!(writeln!(writer, "do_parse!(input, "));

        for part in &parts {
            let nom_parser = part.nom_parser();
            if let Some(name) = part.capture_name() {
                try!(writeln!(writer, "{}: {} >>", name, nom_parser));
            }
                else {
                    try!(writeln!(writer, "{} >>", nom_parser));
                }
        }

        let arguments = parts.iter()
            .flat_map(|p| p.arg_names())
            .collect::<Vec<_>>()
            .join(", ");

        try!(writeln!(writer, "({}::new({}))", self.struct_name, arguments));

        try!(writeln!(writer, ")")); // do_parse!
        try!(writeln!(writer, "}}")); // fn nom_bytes
        try!(writeln!(writer, "}}")); // impl NomBytes

        Ok(())
    }
}