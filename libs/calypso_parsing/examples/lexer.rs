use std::io::{self, BufRead, Read, Write};

use calypso_parsing::token::{Lexer, TokenType};

use calypso_diagnostic::diagnostic::{csr, SimpleFiles};
use calypso_diagnostic::error::Result as CalResult;

fn main() -> CalResult<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut n = 0;
        let mut stdin = stdin.lock();
        print!(">>> ");
        stdout.flush()?;
        buffer.clear();
        let n_read = stdin.read_line(&mut buffer)?;
        if n_read == 0 {
            break;
        }
        let chars = buffer.chars().collect::<Vec<char>>();
        let mut files = SimpleFiles::new();
        let source_id = files.add("<anon>".to_string(), buffer.clone());
        let mut lexer = Lexer::new(source_id, &chars, &files);
        'inner: loop {
            let lexed = lexer.scan();
            if let Err(err) = lexed {
                println!("Error occured: {}", err);
                break;
            } else if let Ok(lexed) = lexed {
                if lexed.value().0 == TokenType::Eof {
                    break 'inner;
                }
                println!("token {}: {:#?}", n, lexed);
                n += 1;
            }
        }
    }
    Ok(())
}
