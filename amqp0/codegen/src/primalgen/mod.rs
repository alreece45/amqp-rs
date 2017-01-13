// Copyright 2016-7 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod method_mod;
mod root_mod;
mod specs_mod;

use std::io;
use std::path::PathBuf;

use {Source, WriteRust};
use format_files;
use common::Spec;

use specs;

use self::method_mod::MethodModuleWriter;
use self::root_mod::SpecModuleWriter;
use self::specs_mod::RootModuleWriter;

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
            specs: specs.iter().map(|spec| Spec::new(spec)).collect(),
            source: source,
        }
    }

    fn write_root_mod(&self) -> io::Result<PathBuf> {
        debug!("Preparing primalgen root module");
        let path = self.source.base_dir().join("mod.rs");
        let writer = RootModuleWriter::new(&self.specs);

        info!("Writing primalgen root module to {}", path.display());
        try!(writer.write_rust_to_path(self.source, &path));
        Ok(path)
    }

    fn write_methods_mod(&self) -> io::Result<PathBuf> {
        debug!("Preparing primalgen methods module");
        let path = self.source.base_dir().join("method.rs");
        let writer = MethodModuleWriter::new(&self.specs);

        info!("Writing primalgen methods module to {}", path.display());
        try!(writer.write_rust_to_path(self.source, &path));
        Ok(path)
    }

    fn write_spec_mod(&self, spec: &Spec) -> io::Result<PathBuf> {
        debug!("Preparing primalgen spec module {}", spec.name());
        let writer = SpecModuleWriter::new(spec);
        let filename = format!("{}.rs", writer.mod_name());
        let path = self.source.base_dir().join(&filename);

        info!("Writing primalgen spec module {} to {}", spec.name(), path.display());
        try!(writer.write_rust_to_path(self.source, &path));
        Ok(path)
    }

    pub fn write_files(&self) -> io::Result<()> {
        let paths = {
            let mut paths = Vec::with_capacity(self.specs.len() + 2);
            paths.push(try!(self.write_root_mod()));
            paths.push(try!(self.write_methods_mod()));

            for spec in &self.specs {
                paths.push(try!(self.write_spec_mod(spec)));
            }
            paths
        };

        if self.source.should_format() {
            debug!("Formatting files");
            format_files(paths);
        }

        Ok(())
    }
}