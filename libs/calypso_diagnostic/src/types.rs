// # Note to Contributors
// Please follow Rust's RFC 1567 (https://github.com/rust-lang/rfcs/blob/master/text/1567-long-error-codes-explanation-normalization.md).
// It's generally a good style for diagnostic information.

/// A helper macro to generate a list of diagnostics.
/// (Shamelessly stolen from rustc_error_codes)
macro_rules! register_diagnostics {
    ($($ecode:ident),* ; $($ecode_no_msg:ident),* $(,)?) => (
        pub static DIAGNOSTICS: &[(&str, Option<&str>)] = &[
            $( (stringify!($ecode), Some(include_str!(concat!("./messages/", stringify!($ecode), ".md"))), ))*
            $( (stringify!($ecode_no_msg), None), )*
        ];
    )
}

register_diagnostics! {
    // These diagnostics have detailed information in messages/<ERROR CODE>.md
    ;
    // These diagnostics do not have detailed information.
}
