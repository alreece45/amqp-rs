// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;

use {WriteRust, Error};
use common::ConstantGroup;

mod constant_group;
mod response_code;
mod protocol;
mod constant;
mod class;

use self::class::Class;

pub use self::constant::Constant;
pub use self::protocol::Protocol;
pub use self::response_code::Group as ResponseCodeGroup;

impl<'a> WriteRust<Protocol<'a>> for Protocol<'a> {
    #[allow(match_same_arms)]
    fn write_rust<W>(&self, protocol: &Protocol<'a>, writer: &mut W) -> Result<(), Error>
        where W: io::Write
    {
        let parsed = protocol.parsed_protocol();

        // constants
        {
            // organize the constants
            let (ungrouped, frame_types, response_codes) = {
                let (mut ungrouped, mut frame_types, mut response_codes) = (vec![], vec![], vec![]);

                for c in parsed.constants() {
                    match c.name().to_kebab_case().as_str() {
                        "frame-min-size"                    => &mut ungrouped,
                        "frame-end"                         => &mut ungrouped,
                        name if name.starts_with("frame-")
                            && name != "frame-error"        => &mut frame_types,
                        _ if c.value().len() == 3           => &mut response_codes,
                        _                                   => &mut ungrouped,
                    }.push(c);
                }

                (ungrouped, frame_types, response_codes)
            };

            // print the ungrouped ones
            try!(writeln!(writer, ""));
            try!(ungrouped.iter()
                .map(|c| protocol.map_constant(c))
                .map(|c| c.write_rust(protocol, writer))
                .collect::<Result<Vec<_>, _>>()
                .map(|_| ()));

            try!(writeln!(writer, ""));

            // print the frame constants
            try!(writeln!(writer, "pub mod frame {{"));
            let frame_type_group = ConstantGroup::new("FrameType", "Type", "u16", frame_types.into_iter());
            try!(frame_type_group.write_rust(protocol, writer));
            try!(writeln!(writer, "}}"));
            try!(writeln!(writer, "pub use self::frame::Type as Frame;"));

            // print the group response code constants
            let response_codes = ResponseCodeGroup::new(response_codes.into_iter());
            try!(response_codes.write_rust(protocol, writer));
        }

        try!(writeln!(writer, "pub struct Table;"));

        try!(writeln!(writer, "pub mod class {{"));
        try!(parsed.classes().iter()
            .map(|c| {
                let ty = protocol.map_domain("class-id");
                let value = try!(protocol.map_value(&ty, c.index()));
                let name = c.name().to_constant_case();
                try!(writeln!(writer, "pub const CLASS_{}: {} = {};", name, ty, value));

                Ok(())
            })
            .collect::<Result<Vec<_>, Error>>()
            .map(|_| ())
        );
        try!(writeln!(writer, ""));

        try!(parsed.classes().iter()
            .map(|c| {
                try!(Class::from_parsed(&c).write_rust(protocol, writer));
                Ok(())
            })
            .collect::<Result<Vec<_>, Error>>()
            .map(|_| ())
        );
        try!(writeln!(writer, "}}"));

        Ok(())
    }
}

