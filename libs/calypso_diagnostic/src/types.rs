//! Diagnostic codes and extended information

// # Note to Contributors
// Please follow Rust's RFC 1567 (https://github.com/rust-lang/rfcs/blob/master/text/1567-long-error-codes-explanation-normalization.md).
// It's generally a good style for diagnostic information.
//
// Diagnostic information should not start with a newline and should end with a newline.

/// A helper macro to generate a list of diagnostics.
// todo(@ThePuzzlemaker: diag|frame):
//   convert this to compile-time format strings using hacky macro stuff
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
    ($ecode:ident$(, $($rest:tt)*)?) => {{
        ::std::format($crate::types::diagnostic_fmt!($ecode)$(, $($rest)*)?)
    }}
}

// todo(diag): add more extended diagnostic information. see #28.
register_diagnostics! {
    // These diagnostics have detailed information in messages/<ERROR CODE>.md
    E0001: "No corresponding `/*` for `*/`.",
    E0002: "No corresponding `*/` for `/*`.",
    ;
    // These diagnostics do not have detailed information.
    // If you feel that they are worthy, feel free to move them to the upper category
    // and write detailed information for them. (Most of them probably are, but I just haven't
    // gotten to it yet.)
    E0000: "Testing diagnostic, please ignore. If this is encountered in regular usage, please file an issue.",
    E0003: "Encountered an unexpected character.",
    E0004: "Expected 2 hexadecimal digits in escape sequence, found none.",
    E0005: "Expected 2 hexadecimal digits in escape sequence, found an invalid digit `{ch}`.",
    E0006: "Expected a valid escape sequence, found `\\{ch}`.",
    E0007: "Expected a valid escape sequence, found EOF.",
    E0008: "Expected a valid escape sequence, found whitespace.",
    E0009: "Expected 2 hexadecimal digits in escape sequence.",
    E0010: "Expected an opening curly bracket before a Unicode codepoint, found `{ch}`.",
    E0011: "Expected an opening curly bracket before a Unicode codepoint, found EOF.",
    E0012: "Expected an opening curly bracket before a Unicode codepoint, found whitespace.",
    E0014: "Expected up to 6 hexadecimal digits in a Unicode codepoint, found an invalid digit `{ch}`.", // note for detailed information: this can be triggered by <4 characters for codepoint with a wrong bracket e.g. `\u{1234)`
    E0015: "Expected a closing curly bracket after a Unicode codepoint, found EOF.",
    E0016: "Expected a closing curly bracket after a Unicode codepoint, found `{ch}`.", // note for detailed information: this can be triggered by more than 6 codepoint characters
    E0017: "Expected a closing curly bracket after a Unicode codepoint, found whitespace.",
    E0018: "Expected up to 6 hexadecimal digits in a Unicode codepoint, found whitespace.",
    E0019: "Expected at least 1 hexadecimal digit in a Unicode codepoint, found none.",
    E0020: "Expected 1 character in character literal, found an invalid character.", // note for detailed information: `\n`, `\r`, `\t` are invalid
    E0021: "Expected 1 character in character literal, found more than one character.",
    E0022: "Expected 1 character in character literal, found none.",
    E0023: "Expected a single quote at end of character literal, found EOF.",
    E0024: "Expected a double quote at end of string literal, found EOF.",
    E0025: "Found a leading zero in an integer literal.",
    E0026: "Encountered an underscore at the end of a number literal.",
    E0027: "Expected a valid decimal part of a float, found an invalid digit `{ch}`",
    E0028: "Expected a valid decimal part of a float, found whitespace.",
    E0029: "Expected a valid decimal part of a float, found EOF.",
    E0030: "Expected a valid exponent of a float, found an invalid digit `{ch}`",
    E0031: "Expected a valid exponent of a float, found whitespace.",
    E0032: "Expected a valid exponent of a float, found EOF.",
    E0033: "Cannot use an explicit base for a float.",
    E0034: "Identifiers (and atoms) must be shorter than 255 characters.",
    E0035: "Expected a digit after number base specifier."
}
