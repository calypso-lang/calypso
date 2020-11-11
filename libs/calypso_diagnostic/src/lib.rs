#![doc(
    html_root_url = "https://thepuzzlemaker.github.io/Calypso/rustdoc/calypso_diagnostic/index.html"
)]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

pub mod diagnostic;
pub mod error;
pub mod types;

pub extern crate codespan_reporting as reporting;
pub type FileMgr = reporting::files::SimpleFiles<String, String>;

pub extern crate strfmt;

#[macro_export]
macro_rules! code {
    ($code:ident) => {
        |d| {
            let code = stringify!($code);
            let diagnostic = $crate::types::DIAGNOSTICS.get(code).unwrap();
            let mut d = d.code(code)
                .message(diagnostic.0);
            if diagnostic.1.is_some() {
                d = d.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
            }
            d
        }
    };

    ($code:ident, $($name:ident = $value:expr),*$(,)?) => {
        |d| {
            let code = stringify!($code);
            let diagnostic = $crate::types::DIAGNOSTICS.get(code).unwrap();
            let diagnostic_fmt = diagnostic.0;
            let diagnostic_ext = diagnostic.1;
            let mut d = d.code(code)
                  .message({
                    let mut map = ::std::collections::HashMap::<::std::string::String, ::std::string::String>::new();
                    $(map.insert(stringify!($name).to_string(), $value.to_string());)*
                    $crate::strfmt::strfmt(diagnostic_fmt, &map)
                  }.unwrap());
            if diagnostic_ext.is_some() {
                d = d.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
            }
            d
        }
    }
}
