
use std::collections::BTreeMap;

#[cfg(feature = "amqp-build-specs")]
include!(concat!(env!("OUT_DIR"), "/amqp0.rs"));
#[cfg(not(feature = "amqp-build-specs"))]
include!(concat!("mod.pregen.rs"));

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Version {
    minor: u16,
    revision: u16,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Spec {
    name: &'static str,
    classes: BTreeMap<&'static str, Class>,
    constants: BTreeMap<&'static str, Constant>,
    domains: BTreeMap<&'static str, &'static str>,
    frame_types: BTreeMap<&'static str, Constant>,
    response_codes: BTreeMap<&'static str, Constant>,
    version: Version,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Class {
    name: &'static str,
    fields: Vec<ClassField>,
    index: u16,
    methods: Vec<ClassMethod>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassMethod {
    name: &'static str,
    index: u16,
    response: Option<&'static str>,
    fields: Vec<ClassMethodField>,
    is_synchronous: bool,

    chassis_server: Option<&'static str>,
    chassis_client: Option<&'static str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassField {
    name: &'static str,
    domain: &'static str,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ClassMethodField {
    name: &'static str,
    domain: &'static str,
    assertions: Vec<ClassMethodFieldAssertion>,
    is_reserved: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ClassMethodFieldAssertion {
    Null,
    NotNull,
    ChannelMax,
    NotZero,
    Enum(Vec<&'static str>),
    Length(usize),
    Regexp(&'static str),
    Syntax(&'static str),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Constant {
    name: &'static str,
    value: u32,
    class: Option<&'static str>,
}

impl Spec {
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn classes(&self) -> &BTreeMap<&'static str, Class> {
        &self.classes
    }
    pub fn domains(&self) -> &BTreeMap<&'static str, &'static str> {
        &self.domains
    }
    pub fn frame_types(&self) -> &BTreeMap<&'static str, Constant> {
        &self.frame_types
    }
    pub fn response_codes(&self) -> &BTreeMap<&'static str, Constant> {
        &self.response_codes
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl Class {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn fields(&self) -> &[ClassField] {
        &self.fields
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn methods(&self) -> &[ClassMethod] {
        &self.methods
    }
}

impl ClassField {
    pub fn name(&self) -> &'static str {
        &self.name
    }

    pub fn domain(&self) -> &'static str {
        self.domain
    }
}

impl ClassMethod {
    pub fn index(&self) -> u16 {
        self.index
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn fields(&self) -> &[ClassMethodField] {
        &self.fields
    }
}

impl ClassMethodField {
    pub fn name(&self) -> &'static str {
        &self.name
    }

    pub fn domain(&self) -> &'static str {
        self.domain
    }

    pub fn assertions(&self) -> &[ClassMethodFieldAssertion] {
        &self.assertions
    }

    pub fn is_reserved(&self) -> bool {
        self.is_reserved
    }
}

impl Constant {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn class(&self) -> Option<&'static str> {
        self.class
    }
}

impl Version {
    pub fn minor(&self) -> u16 {
        self.minor
    }

    pub fn revision(&self) -> u16 {
        self.revision
    }
}