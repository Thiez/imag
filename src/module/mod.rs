use runtime::Runtime;
use std::error::Error;
use std::fmt::Formatter;
use std::fmt::Result as FMTResult;
use std::fmt::Display;
use std::fmt::Debug;
use std::path::Path;
use std::result::Result;
use std::collections::HashMap;

use clap::{App, ArgMatches};

use storage::backend::{StorageBackend, StorageBackendError};

pub mod bm;

#[derive(Debug)]
pub struct ModuleError {
    desc: String,
    caused_by: Option<Box<Error>>,
}

impl ModuleError {
    pub fn new(desc: &'static str) -> ModuleError {
        ModuleError {
            desc: desc.to_owned().to_string(),
            caused_by: None,
        }
    }
}

impl Error for ModuleError {

    fn description(&self) -> &str {
        &self.desc[..]
    }

    fn cause(&self) -> Option<&Error> {
        self.caused_by.as_ref().map(|e| &**e)
    }

}

impl Display for ModuleError {
    fn fmt(&self, f: &mut Formatter) -> FMTResult {
        write!(f, "ModuleError: {}", self.description())
    }
}

pub struct CommandEnv<'a> {
    pub rt:         &'a Runtime<'a>,
    pub bk:         &'a StorageBackend,
    pub matches:    &'a ArgMatches<'a, 'a>,
}

pub type ModuleResult = Result<(), ModuleError>;
pub type CommandResult  = ModuleResult;
pub type CommandMap<'a> = HashMap<&'a str, fn(&Module, CommandEnv) -> CommandResult>;

pub trait Module : Debug {

    fn callnames(&self) -> &'static [&'static str];
    fn name(&self) -> &'static str;
    fn shutdown(&self, rt : &Runtime) -> ModuleResult;

    fn get_commands(&self, rt: &Runtime) -> CommandMap;

}
