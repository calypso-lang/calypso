//! Diagnostic codes and extended information

// # Note to Contributors
// Please follow Rust's RFC 1567 (https://github.com/rust-lang/rfcs/blob/master/text/1567-long-error-codes-explanation-normalization.md).
// It's generally a good style for diagnostic information.
//
// Diagnostic information should not start with a newline and should end with a newline.

/// A helper macro to generate a list of diagnostics.
macro_rules! register_diagnostics {
    ($($ecode:ident: $format:expr),* $(,)?) => {
        use std::collections::HashMap;
	use once_cell::sync::Lazy;

        /// A map from error codes to optional extended information.
	#[allow(warnings)]
	#[allow(clippy::pedantic)]
	pub static DIAGNOSTICS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
            let mut m = HashMap::new();
            $( m.insert(stringify!($ecode), include_str!(concat!("./messages/", stringify!($ecode), ".md"))); )*
            m
        });
    };
}

// // todo(diag): add more extended diagnostic information. see #28.
register_diagnostics! {
    // These diagnostics have detailed information in messages/<ERROR CODE>.md
}
