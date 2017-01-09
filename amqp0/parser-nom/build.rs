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

extern crate env_logger;

#[cfg(feature = "amqp0-codegen")]
extern crate amqp0_codegen as codegen;
#[cfg(feature = "amqp0-specs")]
extern crate amqp0_specs as specs;

fn main() {
    amqp0::build();
}

#[cfg(not(feature = "amqp0-specs"))]
mod amqp0 {
    pub fn build() {
        println!("Skipping build (neither amqp0-build-parser nor amqp0-pregen-parser specified)");
    }
}

#[cfg(feature = "amqp0-specs")]
mod amqp0 {
    use std::env;
    use std::path::{Path, PathBuf};

    use env_logger;
    use codegen::Source;
    use codegen::parsergen::nom::ModulesWriter;
    use specs::specs as amqp0_specs;

    struct NomParserSource  {
        base_dir: PathBuf,
    }

    const SOURCE_CRATES: &'static [&'static str] = &["amqp0-codegen"];
    const SOURCE_REBUILD: &'static [&'static str] = &["amqp0-build-parser"];
    const SOURCE_PREGEN: &'static [&'static str] = &["amqp0-pregen-parser"];

    impl Source for NomParserSource {
        fn name(&self) -> &str { "amqp0-parser-nom" }
        fn crates(&self) -> &[&str] { SOURCE_CRATES }

        fn rebuild_features(&self) -> &[&str] { SOURCE_REBUILD }
        fn pregeneration_features(&self) -> &[&str] { SOURCE_PREGEN }

        fn base_dir(&self) -> &Path { &self.base_dir }

        // rustfmt bug
        fn should_format(&self) -> bool { false }
    }

    pub fn build() {
        let base_dir = if cfg!(feature = "amqp0-pregen-parser") {
            PathBuf::from("pregen")
        } else {
            env::var_os("OUT_DIR")
                .map(PathBuf::from)
                .expect("Error: OUT_DIR not set")
        };

        env_logger::init().unwrap();
        println!("Building amqp0-parser-nom");

        let source = NomParserSource { base_dir: base_dir };
        let writer = ModulesWriter::new(&source, amqp0_specs());
        writer.write_files().unwrap();
    }
}