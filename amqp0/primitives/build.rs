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
        println!("Skipping build (neither amqp0-build-primitives nor amqp0-pregen-primitives specified)");
    }
}

#[cfg(feature = "amqp0-specs")]
mod amqp0 {
    use std::env;
    use std::path::{Path, PathBuf};

    use env_logger;
    use codegen;
    use codegen::primalgen::ModulesWriter;
    use specs::specs as amqp0_specs;

    const BUILDER_CRATES: &'static [&'static str] = &["amqp0-codegen"];
    const BUILDER_REBUILD: &'static [&'static str] = &["amqp0-build-primitives"];
    const BUILDER_PREGEN: &'static [&'static str] = &["amqp0-pregen-primitives"];

    struct PrimitivesSource {
        base_dir: PathBuf,
    }

    impl codegen::Source for PrimitivesSource {
        fn name(&self) -> &str { "amqp0-primitives" }
        fn crates(&self) -> &[&str] { BUILDER_CRATES }

        fn rebuild_features(&self) -> &[&str] { BUILDER_REBUILD }
        fn pregeneration_features(&self) -> &[&str] { BUILDER_PREGEN }

        fn base_dir(&self) -> &Path { &self.base_dir }
        fn should_format(&self) -> bool { cfg!(feature = "amqp0-pregen-primitives") }
    }

    pub fn build() {
        let base_dir = if cfg!(feature = "amqp0-pregen-primitives") {
            PathBuf::from("pregen")
        } else {
            env::var_os("OUT_DIR")
                .map(PathBuf::from)
                .expect("Error: OUT_DIR not set")
        };

        env_logger::init().unwrap();
        println!("Building amqp0-primitives");

        let source = PrimitivesSource { base_dir: base_dir };
        let writer = ModulesWriter::new(&source, amqp0_specs());
        writer.write_files().unwrap();
    }
}