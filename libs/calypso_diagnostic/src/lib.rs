#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_diagnostic/index.html")]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

pub mod diagnostic;
pub mod error;
pub mod report;
pub mod sync;
pub mod types;

pub extern crate codespan_reporting as reporting;
pub type FileMgr = reporting::files::SimpleFiles<String, String>;

pub extern crate strfmt;

#[macro_export]
macro_rules! gen_error {
    (Err($($rest:tt)*) as $ty:ty) => {
        $crate::error::Result::<$ty>::Err($crate::real_gen_error!($($rest)*).into())
    };

    ($report:ident, $($rest:tt)*) => {{
        $report.add($crate::real_gen_error!($($rest)*));
    }};

    ($report:expr, $($rest:tt)*) => {{
        let mut report = $report;
        report.add($crate::real_gen_error!($($rest)*));
        report
    }}
}

#[macro_export]
macro_rules! real_gen_error {

    (@labels $diagnostic:ident, [$($style:expr => ($source:expr, $span:expr); $message:expr),*$(,)?]) => {{
        $diagnostic = $diagnostic$(.label($style, $message, $span, $source))*;
    }};

    (@notes $diagnostic:ident, [$($note:expr),*$(,)?]) => {{
       $diagnostic = $diagnostic$(.note($note))*;
    }};

    (@fmt $diag:expr, $($name:ident = $value:expr),*) => {{
        let mut map = ::std::collections::HashMap::<::std::string::String, ::std::string::String>::new();
        $(map.insert(stringify!($name).to_string(), $value.to_string());)*
        $crate::strfmt::strfmt($diag, &map).unwrap()
    }};

    (@fmt $diag:expr) => {{
        $diag
    }};

    ($self:expr => { $code:ident$(, $($name:ident = $value:expr),*)?; labels: $labels:tt, notes: $notes:tt$(,)? }) => {{
        let mut diagnostic = $crate::diagnostic::DiagnosticBuilder::new(
            $crate::diagnostic::Severity::Error,
            &($self).files,
        );
        let code = stringify!($code);
        let diag = $crate::types::DIAGNOSTICS.get(code).unwrap();
        diagnostic = diagnostic.code(code)
            .message($crate::real_gen_error!(@fmt diag.0$($(,$name = $value)*)?));

        $crate::real_gen_error!(@labels diagnostic, $labels);
        $crate::real_gen_error!(@notes diagnostic, $notes);

        if diag.1.is_some() {
            diagnostic = diagnostic.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
        };

        diagnostic.build()?
    }};

    ($self:expr => { $code:ident$(, $($name:ident = $value:expr),*)?; labels: $labels:tt$(,)?}) => {{
        let mut diagnostic = $crate::diagnostic::DiagnosticBuilder::new(
            $crate::diagnostic::Severity::Error,
            &($self).files,
        );
        let code = stringify!($code);
        let diag = $crate::types::DIAGNOSTICS.get(code).unwrap();
        diagnostic = diagnostic.code(code)
            .message($crate::real_gen_error!(@fmt diag.0$($(,$name = $value)*)?));

        $crate::real_gen_error!(@labels diagnostic, $labels);

        if diag.1.is_some() {
            diagnostic = diagnostic.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
        };

        diagnostic.build()?
    }};

    ($self:expr => { $code:ident$(, $($name:ident = $value:expr),*)?; notes: $notes:tt$(,)? }) => {{
        let mut diagnostic = $crate::diagnostic::DiagnosticBuilder::new(
            $crate::diagnostic::Severity::Error,
            &($self).files,
        );
        let code = stringify!($code);
        let diag = $crate::types::DIAGNOSTICS.get(code).unwrap();
        diagnostic = diagnostic.code(code)
            .message($crate::real_gen_error!(@fmt diag.0$($(,$name = $value)*)?));

        $crate::real_gen_error!(@notes diagnostic, $notes);

        if diag.1.is_some() {
            diagnostic = diagnostic.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
        };

        diagnostic.build()?
    }};

    ($self:expr => { $code:ident$(, $($name:ident = $value:expr),*)?$(;)? }) => {{
        let mut diagnostic = $crate::diagnostic::DiagnosticBuilder::new(
            $crate::diagnostic::Severity::Error,
            &($self).files,
        );
        let code = stringify!($code);
        let diag = $crate::types::DIAGNOSTICS.get(code).unwrap();
        diagnostic = diagnostic.code(code)
            .message($crate::real_gen_error!(@fmt diag.0$($(,$name = $value)*)?));

        if diag.1.is_some() {
            diagnostic = diagnostic.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
        };

        diagnostic.build()?
    }}
}

pub mod prelude {
    pub use super::diagnostic::LabelStyle;
    pub use super::error::{
        Error as CalError, ErrorKind as CalErrorKind, Result as CalResult,
        ResultExt as CalResultExt,
    };
    pub use super::gen_error;
    pub use super::sync::prelude::*;
    pub use super::FileMgr;
    pub use error_chain::ChainedError;
}
