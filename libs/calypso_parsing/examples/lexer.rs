use std::io::{self, BufRead, Read, Write};

use calypso_parsing::token::{Lexer, TokenType};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut n = 0;
        let mut stdin_lock = stdin.lock();
        print!(">>> ");
        stdout.flush();
        buffer.clear();
        let n_read = stdin_lock.read_line(&mut buffer)?;
        if n_read == 0 {
            break;
        }
        drop(stdin_lock);
        println!("input: `{}`", &buffer.trim_end());
        let chars = buffer.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(&chars);
        'inner: loop {
            let lexed = lexer.scan().unwrap();
            if lexed.value().0 == TokenType::Eof {
                break 'inner;
            }
            println!("token {}: {:#?}", n, lexed);
            n += 1;
        }
    }
    Ok(())
}
