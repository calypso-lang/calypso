use calypso_base::span::*;
use calypso_diagnostic::diagnostic::*;
use calypso_diagnostic::types::*;

fn main() {
    //                 0123456789ABCDEF
    let input = "\n\ntest thing ERROR";
    let buffer = input.to_string().chars().collect::<Vec<char>>();
    let span = Span::new(13, 5);
    let diag = Diagnostic::new(
        span,
        &buffer,
        String::from("<anon>"),
        String::from("it's an error"),
        0,
    );
    println!("{:?}", diag);
    println!("{}", diag);
}
