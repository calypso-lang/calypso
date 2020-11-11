use calypso_repl::*;

use std::sync::Arc;

struct Ctx {
    line_no: usize,
}

pub fn main() {
    let mut repl = Repl::<Ctx>::new(
        Box::new(|ctx, input| {
            ctx.line_no += 1;
            Some(input)
        }),
        Ctx { line_no: 0 },
    );

    // You could do something more complicated with the input variable here
    let hello_cmd = Command::new(
        "hello".to_string(),
        "shows a hello message to the user".to_string(),
        "usage: hello".to_string(),
        Box::new(|_ctx, _input| Some(Box::new("hi there user"))),
    );
    let exit_cmd = Command::new(
        "exit".to_string(),
        "exits the repl".to_string(),
        "usage: exit".to_string(),
        Box::new(|_ctx, _input| None),
    )
    .alias("x".to_string())
    .alias("ex".to_string())
    .alias("quit".to_string())
    .alias("q".to_string());
    repl = repl.commands(vec![Arc::new(hello_cmd), Arc::new(exit_cmd)]);
    repl.run("Preamble!!!!!".to_string(), |ctx| {
        format!("{}:> ", ctx.line_no)
    })
    .expect("oops");
}
