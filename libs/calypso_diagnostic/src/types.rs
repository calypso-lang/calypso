// # Note to Contributors
// Please follow Rust's RFC 1567 (https://github.com/rust-lang/rfcs/blob/master/text/1567-long-error-codes-explanation-normalization.md).
// It's generally a good style for diagnostic information.
//
// Diagnostic information should not start with a newline, but they should end with a newline.

/// A helper macro to generate a list of diagnostics.
/// (Shamelessly stolen from rustc_error_codes)
macro_rules! register_diagnostics {
    ($($ecode:ident: $format:expr),* $(,)? ; $($ecode_no_msg:ident: $format_no_msg:expr),* $(,)?) => (
        use ::std::collections::HashMap;

        lazy_static! {
            pub static ref DIAGNOSTICS: HashMap<&'static str, (&'static str, Option<&'static str>)> = {
                let mut m = HashMap::new();
                $(
                    m.insert(
                        stringify!($ecode),
                        (
                            $format,
                            Some(
                                include_str!(
                                    concat!(
                                        "./messages/",
                                        stringify!($ecode),
                                        ".md"
                                    )
                                )
                            )
                        )
                    );
                )*
                $(
                    m.insert(
                        stringify!($ecode_no_msg),
                        ($format_no_msg, None)
                    );
                )*
                m
            };
        }
    )
}

register_diagnostics! {
    // These diagnostics have detailed information in messages/<ERROR CODE>.md
    E0001: "No corresponding `/*` for `*/`.",
    E0002: "No corresponding `*/` for `/*`.",
    ;
    // These diagnostics do not have detailed information.
    // If you feel that they are worthy, feel free to move them to the upper category
    // and write detailed information for them. (Most of them probably are, but I just haven't
    // gotten to it yet.)
    E0003: "Encountered an unexpected character.",
    E0004: "Expected two hexadecimal digits in escape sequence, found none.",
    E0005: "Expected two hexadecimal digits in escape sequence, found an invalid digit `{ch}`.",
    E0006: "Expected a valid escape sequence, found `\\{ch}`.",
    E0007: "Expected a valid escape sequence, found EOF.",
    E0008: "Expected a valid escape sequence, found whitespace, found one.",
    E0009: "Expected two hexadecimal digits in escape sequence.",
    E0010: "Expected an opening curly bracket before a Unicode codepoint, found `{ch}`.",
    E0011: "Expected an opening curly bracket before a Unicode codepoint, found EOF.",
    E0012: "Expected an opening curly bracket before a Unicode codepoint, found whitespace.",
    E0014: "Expected up to 6 hexadecimal digits in a Unicode codepoint, found an invalid digit `{ch}`.", // note for detailed information: this can be triggered by <4 characters for codepoint with a wrong bracket e.g. `\u{1234)`
    E0015: "Expected a closing curly bracket after a Unicode codepoint, found EOF.",
    E0016: "Expected a closing curly bracket after a Unicode codepoint, found `{ch}`.", // note for detailed information: this can be triggered by more than 6 codepoint characters
    E0017: "Expected a closing curly bracket after a Unicode codepoint, found whitespace.",
    E0018: "Expected up to 6 hexadecimal digits in a Unicode codepoint, found whitespace.",
    E0019: "Expected at least 1 hexadecimal digit in a Unicode codepoint, found none."
}
