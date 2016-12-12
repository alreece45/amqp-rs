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

#[cfg(any(feature = "amqp0-build-parser", feature = "amqp0-pregen-parser"))]
extern crate amqp0_codegen as codegen;
#[cfg(any(feature = "amqp0-build-parser", feature = "amqp0-pregen-parser"))]
extern crate amqp0_specs as specs;

fn main() {
    amqp0::build();
}

#[cfg(not(any(feature = "amqp0-build-parser", feature = "amqp0-pregen-parser")))]
mod amqp0 {
    pub fn build() {
        println!("Skipping build (neither amqp0-build-parser nor amqp0-pregen-parser specified)");
    }
}

#[cfg(any(feature = "amqp0-build-parser", feature = "amqp0-pregen-parser"))]
mod amqp0 {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};

    use codegen::{self, Builder, CodeWriter};
    use codegen::parsergen::nom::{SpecsModuleWriter, SpecModuleWriter};
    use specs::specs as amqp0_specs;

    struct ParserBuilder;

    const BUILDER_CRATES: &'static [&'static str] = &["amqp0-codegen"];
    const BUILDER_REBUILD: &'static [&'static str] = &["amqp0-build-parser"];
    const BUILDER_PREGEN: &'static [&'static str] = &["amqp0-build-parser"];

    impl Builder for ParserBuilder {
        fn name(&self) -> &str { "amqp0-parser-nom" }
        fn crates(&self) -> &[&str] { BUILDER_CRATES }
        fn rebuild_features(&self) -> &[&str] { BUILDER_REBUILD }
        fn pregeneration_features(&self) -> &[&str] { BUILDER_PREGEN }
    }

    pub fn build() {
        println!("Building parser from amqpspec");

        let out_path = env::var_os("OUT_DIR").map(PathBuf::from).expect("Error: OUT_DIR not set");
        let specs = amqp0_specs();
        let mut paths: Vec<PathBuf> = Vec::with_capacity(1 + specs.len());

        // mod.rs
        let specs_writer = SpecsModuleWriter::new(&specs[..]);
        let writer = CodeWriter::new(ParserBuilder, specs_writer);
        println!("Generated mod.rs");
        let mod_path = out_path.join("mod.rs");
        writer.write_to_path(&mod_path).expect("Failed to write amqp0.rs");
        paths.push(mod_path);

        // {name}{minor}_{revision}.rs
        for spec in &specs {
            let spec_writer = SpecModuleWriter::new(spec);
            let filename = format!("{}.rs", spec_writer.mod_name());
            let path = out_path.join(&filename);
            let writer = CodeWriter::new(ParserBuilder, spec_writer);
            println!("Generating {}", filename);
            writer.write_to_path(&path).expect("Failed to write spec module");
            paths.push(path);
        }

        if cfg!(feature = "rustfmt") {
            // Formatting is currently broken for these pregenerated files
            // codegen::format_files(paths.clone());
        }

        if cfg!(feature = "amqp0-pregen-parser") {
            let pregen_dir = Path::new("pregen");
            for path in paths {
                let suffix = path.strip_prefix(&out_path).unwrap();
                let dst = pregen_dir.join(suffix);
                println!("Saving to {} to {}", path.display(), dst.display());
                fs::copy(&path, dst).unwrap();
            }
        }
    }
}