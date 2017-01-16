// Copyright 2016 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![cfg_attr(not(feature="clippy"), allow(unknown_lints))]

#[cfg(feature = "rustfmt")]
extern crate rustfmt;

#[cfg(any(feature = "amqp0-build-specs", feature = "amqp0-pregen-specs"))]
extern crate amqp0_specgen as specgen;

fn main() {
    spec0_builder::build().unwrap();
}

#[cfg(not(any(feature = "amqp0-build-specs", feature = "amqp0-pregen-specs")))]
mod spec0_builder {
    use std::io;
    pub fn build() -> io::Result<()> {
        println!("Skipping build (neither amqp0-build-specs nor amqp0-pregen-specs specified)");

        Ok(())
    }
}

#[cfg(any(feature = "amqp0-build-specs", feature = "amqp0-pregen-specs"))]
mod spec0_builder {
    extern crate env_logger;

    use std::env;
    use std::fs::{self, File};
    use std::io::{self, Write, BufWriter};
    use std::path::PathBuf;
    use specgen::{Spec, ParseError};

    #[cfg(feature = "rustfmt")]
    use rustfmt;

    pub fn build() -> io::Result<()> {
        env_logger::init().unwrap();

        let root_out = if cfg!(feature = "amqp0-pregen-specs") {
            PathBuf::from("pregen")
        } else {
            env::var_os("OUT_DIR").map(PathBuf::from).expect("Error: OUT_DIR not set")
        };

        // remove old path, if needed
        let old_path = if cfg!(feature = "amqp0-pregen-specs") {
            PathBuf::from("src/lib.pregen.rs")
        }
        else {
            root_out.join("amqp0.rs")
        };
        println!("Removing potentially old path: {}", old_path.display());
        let _ = fs::remove_file(old_path);

        // load the xml specs
        let cwd = env::current_dir().expect("Unable to get current directory");
        let xml_dir = cwd.join("xml");

        let specs = amqp0_specs().into_iter()
            .map(|(name, filename)| {
                let path = xml_dir.join(filename.to_string());
                println!("cargo:rerun-if-changed={}", path.display());
                let spec = try!(Spec::parse_xml_path(path));
                let snake_name = format!(
                    "{}{}_{}",
                    name,
                    spec.version().minor(),
                    spec.version().revision()
                );
                let constant_case = snake_name.to_uppercase();
                Ok((name, snake_name, constant_case, spec))
            })
            .collect::<Result<Vec<_>, ParseError>>()
            .unwrap();
        let mut paths = Vec::with_capacity(1 + specs.len());

        // mod.rs
        paths.push({
            let mod_path = root_out.join("mod.rs");
            let mut writer = BufWriter::new(try!(File::create(&mod_path)));
            try!(write_header(&mut writer));

            for &(_, ref snake_name, _, _) in &specs {
                try!(writeln!(writer, "mod {};", snake_name));
            }

            try!(writeln!(writer, "\nstatic SPECS: &'static [&'static ::Spec] = &["));
            for &(_, ref snake_name, ref constant_case, _) in &specs {
                try!(writeln!(writer, "&{}::{},", snake_name, constant_case));
            }
            writeln!(writer, "];").unwrap();

            try!(writeln!(
                writer,
                "pub fn specs() -> &'static [&'static ::Spec] {{\n\
                    &SPECS\n\
                }}"
            ));

            for &(_, ref snake_name, ref constant_name, _) in &specs {
                try!(writeln!(
                    writer,
                    "pub fn {snake_name}() -> &'static Spec {{\
                        &{snake_name}::{constant_name}\
                    }}",
                    snake_name = snake_name,
                    constant_name = constant_name
                ));
            }

            mod_path
        });

        for &(name, ref snake_name, ref constant_name, ref spec) in &specs {
            let spec_path = root_out.join(format!("{}.rs", snake_name));
            let mut writer = BufWriter::new(try!(File::create(&spec_path)));
            try!(write_header(&mut writer));

            writeln!(writer, "pub static {}: ::Spec = ", constant_name).unwrap();
            spec.write_generated(name, &mut writer).unwrap();
            writeln!(writer, ";").unwrap();
            paths.push(spec_path)
        }

        // file(s) needs to be dropped before reaching here
        if cfg!(feature = "rustfmt") {
            format_files(paths.into_iter());
        }

        Ok(())
    }

    fn amqp0_specs() -> Vec<(&'static str, &'static str)> {
        vec![
            ("amqp", "amqp0-9-1.stripped.xml"),
            ("amqp","amqp0-9.stripped.xml"),
            ("amqp","amqp0-8.stripped.xml"),
            ("rabbitmq", "amqp0-9-1.stripped.rabbitmq.xml"),
            ("qpid", "amqp0-9-qpid.stripped.xml"),
            ("qpid", "amqp0-8-qpid.stripped.xml"),
        ]
    }

    fn write_header<W>(writer: &mut W) -> io::Result<()>
        where W: io::Write
    {
        writeln!(
            writer,
            "\
            /// Generated by build script in amqp0-specs\n\
            /// Pregenerated files are used by default.\n\
            ///\n\
            /// Build using amqp0-pregen crate using: cargo --features=\"amqp0-build-specs\"\n\
            /// Regenerate pregenerated scripts using: cargo --features=\"amqp0-pregen-specs\"\n\
            ///\n\
            /// EDITORS BEWARE: Your modifications may be overridden\n"
        )
    }

    #[cfg(not(feature = "rustfmt"))]
    fn format_files<I>(_: I)
        where I: Iterator<Item = PathBuf>
    {}

    #[cfg(feature = "rustfmt")]
    fn format_files<I>(paths: I)
        where I: Iterator<Item = PathBuf>,
    {
        use rustfmt::Input;
        use rustfmt::config::{self as fmtconfig};

        let config = {
            let mut config = fmtconfig::Config::default();
            config.write_mode = fmtconfig::WriteMode::Overwrite;
            config
        };

        for path in paths {
            println!("Formatting {}", path.display());
            let summary = rustfmt::run(Input::File(path), &config);
            println!("Summary: {:?}", summary)
        }
    }
}