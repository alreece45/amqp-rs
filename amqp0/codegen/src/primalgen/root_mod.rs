// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use inflections::Inflect;

use WriteRust;
use common::Specs;

use super::spec_struct::SpecStructWriter;

pub struct RootModuleWriter<'a> {
    specs: &'a Specs<'a>,
}

impl<'a> WriteRust for RootModuleWriter<'a> {
    fn write_rust_to<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        // ensure that class ids remain consistent across the specs
        self.specs.assert_name_indexes_consistent();

        try!(writeln!(writer, "\npub mod method;"));
        for spec in self.specs {
            try!(writeln!(writer, "pub mod {};", spec.mod_name()));
        }
        try!(writeln!(writer, ""));

        try!(self.write_frame_type_constants(writer));
        try!(self.write_class_constants(writer));
        try!(self.write_method_constants(writer));

        for spec in self.specs {
            let spec_struct = SpecStructWriter::new(spec);
            try!(spec_struct.write_rust_to(writer));
        }

        Ok(())
    }
}

impl<'a> RootModuleWriter<'a> {
    pub fn new(specs: &'a Specs) -> Self {
        RootModuleWriter {
            specs: specs
        }
    }

    pub fn write_class_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(
            writer,
            "//\n\
            // Index values for classes shared among multiple specs\n\
            //\n\
            // Sometimes, the index value is repeated in different classes, but these are not reused\n\
            // within a single protocol\n\
            //\n\
            // Classes are currently only considered common if they are used in more than one\n\
            // spec. This behavior *may* change in the future as more specs are added.\n\
            //"
        ));

        let common_classes = {
            let mut classes = self.specs.common_classes().into_iter().collect::<Vec<_>>();
            classes.sort_by_key(|entry| (entry.1, entry.0));
            classes
        };

        for (class_name, index) in common_classes {
            let constant_class = class_name.to_constant_case();
            try!(writeln!(writer, "pub const CLASS_{}: u16 = {};", constant_class, index));
        }
        try!(writeln!(writer, ""));

        Ok(())
    }

    pub fn write_frame_type_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(
            writer,
            "//\n\
            // Frame types ids shared among multiple specs\n\
            //"
        ));

        let common_frame_types = {
            let mut frame_types = self.specs.common_frame_types().into_iter().collect::<Vec<_>>();
            frame_types.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
            frame_types
        };

        for (class_name, index) in common_frame_types {
            let constant_class = class_name.to_constant_case();
            try!(writeln!(writer, "pub const {}: u8 = {};", constant_class, index));
        }
        try!(writeln!(writer, ""));

        Ok(())
    }

    pub fn write_method_constants<W>(&self, writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        try!(writeln!(
            writer,
            "//\n\
            // Index values for methods common among the different specs\n\
            //\n\
            // Methods are only considered common when:\n\
            //\n\
            //   * The index value is consistent across all of the specs\n\
            //   * The method is used in more than one primalgen.spec\n\
            //\n\
            // This may change in the future-- in that case, methods *may* be removed, or\n\
            // one of the requirements may be relaxed.\n\
            //"
        ));

        let common_methods = {
            let mut methods = self.specs.common_methods().into_iter().collect::<Vec<_>>();
            methods.sort_by(|&(a, _), &(b, _)| a.cmp(b));
            methods
        };

        for (class_name, methods) in common_methods {
            if methods.is_empty() {
                continue
            };

            let methods = {
                let mut methods = methods.into_iter().collect::<Vec<_>>();
                methods.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
                methods
            };

            let constant_class = class_name.to_constant_case();

            for (method_name, index) in methods {
                let constant_method = method_name.to_constant_case();

                if constant_method != "_" {
                    try!(writeln!(writer, "pub const METHOD_{}_{}: u16 = {};", constant_class, constant_method, index));
                }
            }
            try!(writeln!(writer, ""));
        }

        Ok(())
    }
}