use calypso_repl::Repl;

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
    repl.run("Preamble!!!!!".to_string(), |ctx| {
        format!("{}:> ", ctx.line_no)
    })
    .expect("oops");
}
