use super::diagnostic::Diagnostic;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        Diagnostic(diagnostic: Diagnostic) {
            description(diagnostic.eid().get_reason().unwrap()),
            display("{}", diagnostic),
        }
    }
}
