// Copyright 2016-7 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod method_mod;
mod spec_mod;
mod specs_mod;

use std::io;
use specs;

use {WriteRust, Source, format_files};
use common::Spec;

use self::spec_mod::SpecModuleWriter;
use self::specs_mod::SpecsModuleWriter;

pub struct ModulesWriter<'a, S>
    where S: Source + 'a
{
    specs: Vec<Spec>,
    source: &'a S,
}

impl<'a, S> ModulesWriter<'a, S>
    where S: Source + 'a
{
    pub fn new(source: &'a S, specs: &'a [&'static specs::Spec]) -> Self {
        ModulesWriter {
            source: source,
            specs: specs.iter()
                .map(|spec| Spec::new(spec))
                .collect::<Vec<_>>(),
        }
    }

    pub fn write_files(&self) -> io::Result<()> {
        let mut paths = Vec::with_capacity(self.specs.len() + 1);

        let path = self.source.base_dir().join("mod.rs");
        let writer = SpecsModuleWriter::from_spec_slice(&self.specs[..]);

        debug!("Writing parser-nom module to {}", path.display());
        match writer.write_rust_to_path(self.source, &path) {
            Ok(_) => paths.push(path),
            Err(e) => error!("Failed to write specs to {}: {}", path.display(), e),
        }

        for spec in &self.specs {
            debug!("Generating parser-nom module for {}", spec.name());

            let writer = SpecModuleWriter::from_spec(spec);
            let filename = format!("{}.rs", spec.mod_name());
            let path = self.source.base_dir().join(&filename);

            debug!("Writing {} parser-nom module to {}", spec.name(), path.display());
            match writer.write_rust_to_path(self.source, &path) {
                Ok(_) => paths.push(path),
                Err(e) => error!(
                    "Failed to write spec module {} to {}: {}",
                    spec.name(),
                    path.display(),
                    e
                ),
            }
        }

        if self.source.should_format() {
            debug!("Formatting files");
            format_files(paths);
        }

        Ok(())
    }
}