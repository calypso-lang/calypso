use super::diagnostic::Diagnostic;

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Io(::std::io::Error);
        FromUtf8(::std::string::FromUtf8Error);
        DiagnosticRendering(::codespan_reporting::files::Error);
    }

    errors {
        Diagnostic(diagnostic: Diagnostic) {
            description(diagnostic.reason()),
            display("{}", diagnostic),
        }
    }
}
