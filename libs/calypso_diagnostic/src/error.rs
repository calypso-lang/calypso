use super::diagnostic::Diagnostic;

// todo(@ThePuzzlemaker: diag): Diagnostic reports - colletion of various diagnostics and lexer synchronization

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
    }

    errors {
        Diagnostic(diagnostic: Diagnostic) {
            description(diagnostic.reason()),
            display("{}", diagnostic),
        }
    }
}
