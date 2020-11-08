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
macro_rules! strfmt {
    ($fmt:expr, $($name:ident = $value:expr),*) => {{
        let map = ::std::collections::HashMap::<::std::string::String, ::std::string::String>::new();
        $(map.insert(stringify!($name), $value);)*
        $crate::strfmt::strfmt($fmt, &map)
    }}
}

#[macro_export]
macro_rules! code {
    ($code:ident$(, $args:tt)?) => {
        |d| {
            let code = stringify!($code);
            let diagnostic = $crate::types::DIAGNOSTICS.get(code).unwrap();
            let diagnostic_fmt = diagnostic.0;
            let diagnostic_ext = diagnostic.1;
            let mut d = d.code(code)
                  .message($crate::strfmt!(diagnostic_fmt, $($args)?).unwrap());
            if let Some(ext) = diagnostic_ext {
                d = d.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
            }
            d
        }
    }
}
