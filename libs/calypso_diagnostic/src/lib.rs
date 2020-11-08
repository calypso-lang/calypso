#[macro_use]
extern crate error_chain;

pub mod diagnostic;
pub mod error;
pub mod types;

pub extern crate codespan_reporting as reporting;
pub type FileMgr = reporting::files::SimpleFiles<String, String>;
