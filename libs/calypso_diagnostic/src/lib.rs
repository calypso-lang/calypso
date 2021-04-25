#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_diagnostic/index.html")]
#![warn(clippy::pedantic)]

#[macro_use]
extern crate lazy_static;

pub extern crate calypso_error;
pub extern crate codespan_reporting as reporting;
pub extern crate strfmt;

pub mod diagnostic;
pub mod error;
pub mod report;
pub mod types;

pub type FileMgr = reporting::files::SimpleFiles<String, String>;

/// Generate errors or report synchronized errors.
///
/// Panic errors are handled with [`Result`]s.
/// They can be generated as follows:
/// ```rust,ignore
/// gen_error!(Err(self => {
///     E0005, fmt_values = "values";
///     labels: [
///         LabelStyle::Primary =>
///             (source_id, span);
///             "label message",
///         LabelStyle::Secondary =>
///             (source_id2, span2);
///             "label message 2"
///     ],
///     notes: [
///         "note 1",
///         "note 2"
///     ]
/// }) as ())?
/// ```
/// The `as ()` at the end is to change the `Ok` type of the result, if necessary.
/// Panic errors should only be used if there is **ABSOLUTE CONFIDENCE** that
/// there is **NO** way to recover from this error.
// todo(@ThePuzzlemaker: diag|frame):
//   convert this to compile-time format strings using hacky macro stuff
#[macro_export]
macro_rules! gen_error {
    (Err($($rest:tt)*) as $ty:ty) => {
        $crate::calypso_error::CalResult::<$ty>::Err($crate::error::DiagnosticError::from($crate::gen_error!(@i1 $($rest)*)).into())
    };

    (sync $grcx:expr, $($rest:tt)*) => {{
        $grcx.report_syncd($crate::gen_error!(@i1 $($rest)*));
    }};

    (@i3 $diagnostic:ident; notes: [$($note:expr),*$(,)?]) => {{
       $diagnostic = $diagnostic$(.note($note))*;
    }};

    (@i2 $diagnostic:ident; labels: [$($style:expr => ($source:expr, $span:expr); $message:expr),*$(,)?]$(, $($rest:tt)*)?) => {{
        $diagnostic = $diagnostic$(.label($style, $message, $span, $source))*;
        $($crate::gen_error!(@i3 $diagnostic; $($rest)*))?;
    }};

    (@fmt $diag:expr, $($name:ident = $value:expr),*) => {{
        let mut map = ::std::collections::HashMap::<::std::string::String, ::std::string::String>::new();
        $(map.insert(stringify!($name).to_string(), $value.to_string());)*
        $crate::strfmt::strfmt($diag, &map).unwrap()
    }};

    (@fmt $diag:expr) => {{
        $diag
    }};

    (@i1 $self:expr => { $code:ident$(, $($name:ident = $value:expr),*)?; $($rest:tt)* }) => {{
        let mut diagnostic = $crate::diagnostic::Builder::new(
            $crate::diagnostic::Severity::Error,
            &($self).files,
        );
        let code = stringify!($code);
        let diag = $crate::types::DIAGNOSTICS.get(code).unwrap();
        diagnostic = diagnostic.code(code)
            .message($crate::gen_error!(@fmt diag.0$($(,$name = $value)*)?));
        if diag.1.is_some() {
            diagnostic = diagnostic.note(format!("note: this error has more details for troubleshooting, run `calypso explain {}`", code))
        };

        $crate::gen_error!(@i2 diagnostic; $($rest)*);
        diagnostic.build()?
    }};
}

pub mod prelude {
    pub use super::calypso_error::{CalError, CalResult};
    pub use super::diagnostic::LabelStyle;
    pub use super::error::DiagnosticError;
    pub use super::gen_error;
    pub use super::FileMgr;
}
