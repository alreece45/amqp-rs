// Copyright 2016-7 Alexander Reece
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod class_mod;
mod method_mod;
mod root_mod;
mod spec_mod;
mod spec_struct;

use std::{fs, io};
use std::path::PathBuf;

use {Source, WriteRust};
use format_files;
use common::{Specs, Spec, Class};

use specs;

use self::class_mod::ClassModuleWriter;
use self::method_mod::MethodModuleWriter;
use self::root_mod::RootModuleWriter;
use self::spec_mod::SpecModuleWriter;

pub struct ModulesWriter<'a, S>
    where S: Source + 'a
{
    specs: Specs<'a>,
    source: &'a S,
}

impl<'a, S> ModulesWriter<'a, S>
    where S: Source + 'a
{
    pub fn new(source: &'a S, specs: &'a [&'static specs::Spec]) -> Self {
        let specs = Specs::new(specs.iter().map(|spec| Spec::new(spec)).collect::<Vec<_>>());
        ModulesWriter {
            specs: specs,
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
        let mod_path = self.source.base_dir().join(spec.mod_name());
        let path = mod_path.join("mod.rs");

        let old_path = self.source.base_dir().join(format!("{}.rs", spec.mod_name()));
        println!("Removing potentially old path: {}", old_path.display());
        let _ = fs::remove_file(old_path);

        info!("Writing primalgen spec module {} to {}", spec.name(), path.display());
        let writer = SpecModuleWriter::new(spec);
        try!(writer.write_rust_to_path(self.source, &path));
        Ok(path)
    }

    fn write_class_mod(&self, spec: &Spec, class: &Class) -> io::Result<PathBuf> {
        debug!("Preparing primalgen class module {}", spec.name());
        let mod_path = self.source.base_dir().join(spec.mod_name());
        let path = mod_path.join(format!("{}.rs", class.snake_case()));

        info!("Writing primalgen class module {} to {}", spec.name(), path.display());
        let writer = ClassModuleWriter::new(&self.specs, spec, class);
        try!(writer.write_rust_to_path(self.source, &path));
        Ok(path)
    }

    pub fn write_files(&self) -> io::Result<()> {
        let paths = {
            let num_classes = self.specs.iter().flat_map(|spec| spec.classes()).count();
            let mut paths = Vec::with_capacity(2 + num_classes);

            paths.push(try!(self.write_root_mod()));
            paths.push(try!(self.write_methods_mod()));

            for spec in &self.specs {
                paths.push(try!(self.write_spec_mod(spec)));
                for class in spec.classes() {
                    paths.push(try!(self.write_class_mod(spec, class)));
                }
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