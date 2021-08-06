//! Diagnostic codes and extended information

// # Note to Contributors
// Please follow Rust's RFC 1567 (https://github.com/rust-lang/rfcs/blob/master/text/1567-long-error-codes-explanation-normalization.md).
// It's generally a good style for diagnostic information.
//
// Diagnostic information should not start with a newline and should end with a newline.

/// A helper macro to generate a list of diagnostics.
macro_rules! register_diagnostics {
    ($($ecode:ident: $format:expr),* $(,)? ; $($ecode_no_msg:ident: $format_no_msg:expr),* $(,)?) => {
        /// The registerred format string for a diagnostic.
        #[macro_export]
        #[allow(unused_macros)]
        macro_rules! diagnostic_fmt {
            $(($ecode) => {$format};)*
            $(($ecode_no_msg) => {$format_no_msg};)*
        }

        use ::std::collections::HashMap;

        ::lazy_static::lazy_static! {
            /// A map from error codes to optional extended information.
            pub static ref DIAGNOSTICS: HashMap<&'static str, Option<&'static str>> = {
                let mut m = HashMap::new();
                $( m.insert(stringify!($ecode), Some(include_str!(concat!("./messages/", stringify!($ecode), ".md")))); )*
                $( m.insert(stringify!($ecode_no_msg), None); )*
                m
            };
        }
    };
}

/// Format a diagnostic short message based on its registerred format string.
#[macro_export]
#[allow(unused_macros)]
macro_rules! err {
    ($ecode:ident, $($rest:tt)*) => {{
        format!($crate::diagnostic_fmt!($ecode), $($rest)*)
    }};

    ($ecode:ident) => {{
        $crate::diagnostic_fmt!($ecode)
    }};
}

// // todo(diag): add more extended diagnostic information. see #28.
register_diagnostics! {
    // These diagnostics have detailed information in messages/<ERROR CODE>.md
    ;
    E0000: "Testing diagnostic, please ignore. If this is encountered in regular usage, please file an issue.",
    E0001: "A syntax error was encountered.",
}
