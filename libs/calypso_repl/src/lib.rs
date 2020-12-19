#![doc(html_root_url = "https://calypso-lang.github.io/rustdoc/calypso_repl/index.html")]
#![warn(clippy::pedantic)]

use std::collections::HashMap;
use std::sync::Arc;

use regex::Regex;
use rustyline::{config::Configurer, error::ReadlineError, Cmd, Editor, KeyEvent, Movement};

/*
== TODOs ==
todo(@ThePuzzlemaker: repl): Color!
todo(@ThePuzzlemaker: repl): Clean this code up
todo(@ThePuzzlemaker: repl): Find if any helpful Rustyline key bindings are missing
todo(@ThePuzzlemaker: repl): Re-implement with actix
*/

/// A struct for doing REPL-like activities.
/// This does not necessarily need to fit the exact definition of REPL (Read, Eval, Print, Loop).
///
/// The `Ctx` type parameter can be any type of REPL-global context, e.g. a parser context or perhaps an environment structure.
///
/// The command `help`, with aliases `['h', '?']` is reserved.
pub struct Repl<Ctx> {
    /// A closure that evaluates the input and returns something implementing `Display`
    eval: Eval<Ctx>,
    /// Meta-command definitions. This is a `Vec` as you may want to dynamically initialize commands.
    cmds: Vec<Arc<Command<Ctx>>>,
    /// A HashMap containing references to the commands. This is cached to allow faster command execution.
    cache: HashMap<String, Arc<Command<Ctx>>>,
    /// The Rustyline context
    editor: Editor<()>,
    /// The context
    ctx: Ctx,
    /// Prefix for commands. Default: `:`
    prefix: String,
    /// Regex for commands
    cmd_regex: Regex,
}

impl<Ctx> Repl<Ctx> {
    pub fn new(eval: Eval<Ctx>, ctx: Ctx) -> Self {
        let mut editor = Editor::new();
        editor.set_auto_add_history(true);
        editor.set_tab_stop(4);
        editor.set_history_ignore_space(false);
        editor.bind_sequence(KeyEvent::ctrl('C'), Cmd::Kill(Movement::WholeLine));
        Self {
            eval,
            cmds: Vec::new(),
            cache: HashMap::new(),
            editor,
            ctx,
            prefix: String::from(":"),
            cmd_regex: Regex::new(r"^:(?P<command>\S*)( (?P<args>.*))?").unwrap(),
        }
    }

    pub fn prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        // We escape the prefix, so it's guaranteed to be valid.
        Regex::new(&format!(
            r"^{}(?P<command>\S*)( (?P<args>.*))?",
            regex::escape(&self.prefix)
        ))
        .unwrap();
        self
    }

    /// Extend the commands vector
    pub fn commands(mut self, commands: Vec<Arc<Command<Ctx>>>) -> Self {
        for command in &commands {
            self.cache_command(&command);
        }
        self.cmds.extend(commands);
        self
    }

    /// Add a command
    pub fn command(mut self, command: Command<Ctx>) -> Self {
        let arc = Arc::new(command);
        self.cache_command(&Arc::clone(&arc));
        self.cmds.push(arc);
        self
    }

    /// Run the REPL.
    ///
    /// # Errors
    /// The only errors currently returned by this function are errors from `rustyline`.
    pub fn run(
        &mut self,
        preamble: &str,
        prompt: impl Fn(&mut Ctx) -> String,
    ) -> Result<(), ReadlineError> {
        let rl = &mut self.editor;
        println!("{}", preamble);
        loop {
            match rl.readline(&(prompt)(&mut self.ctx)) {
                Ok(line) => {
                    let captures = self.cmd_regex.captures(&line);
                    if let Some(captures) = captures {
                        let ctx = &mut self.ctx;
                        let command = captures.name("command");
                        let args = captures.name("args");
                        if let Some(command) = command {
                            let command = command.as_str();
                            let args = args.map_or("", |v| v.as_str());
                            if command == "?" || command == "h" || command == "help" {
                                if args.is_empty() {
                                    for command in &self.cmds {
                                        println!(
                                            "{}{}: {}, aliases: {} (for more info, run {0}? {1})",
                                            self.prefix,
                                            command.name,
                                            command.description,
                                            command
                                                .aliases
                                                .iter()
                                                .map(|v| format!("`{}`", v))
                                                .collect::<Vec<String>>()
                                                .join(", ")
                                        );
                                    }
                                    println!("{}help: show help for a command or list commands, aliases: `?`, `h`", self.prefix);
                                    continue;
                                } else {
                                    let args = args.split_whitespace().collect::<Vec<&str>>();
                                    if args.len() != 1 {
                                        eprintln!("error: usage: {}? [command]", self.prefix);
                                        continue;
                                    }
                                    let first = *args.first().unwrap();
                                    if let Some(command) = self.cache.get(first) {
                                        println!(
                                            "{}{}: {}\n===\n{}\naliases: {}",
                                            self.prefix,
                                            command.name,
                                            command.description,
                                            command.help,
                                            command
                                                .aliases
                                                .iter()
                                                .map(|v| format!("`{}`", v))
                                                .collect::<Vec<String>>()
                                                .join(", ")
                                        );
                                    } else if first == "?" || first == "h" || first == "help" {
                                        println!(
                                            "{}help: show help for a command or list commands\n===\nusage: ? [command]\naliases: `?`, `h`\n",
                                            self.prefix
                                        );
                                    } else {
                                        eprintln!(
                                            "error: no such command: `{}{}`",
                                            self.prefix, first
                                        );
                                    }
                                    continue;
                                }
                            } else if let Some(command) = self.cache.get(command) {
                                let result = (command.eval)(ctx, args.to_string());
                                if result.is_none() {
                                    break;
                                }
                                println!("{}", result.unwrap());
                                continue;
                            } else {
                                eprintln!("error: could not find command `{}`", command);
                                continue;
                            }
                        }
                        // If the command didn't match, then it must be valid syntax.
                    }
                    let ctx = &mut self.ctx;
                    let result = (self.eval)(ctx, line);
                    if result.is_none() {
                        break;
                    }
                    println!("{}", result.unwrap());
                }
                Err(ReadlineError::Eof) => break,
                Err(err) => return Err(err),
            };
        }
        Ok(())
    }

    fn cache_command(&mut self, command: &Arc<Command<Ctx>>) {
        if self.cache.contains_key(&command.name) || &command.name == "help" {
            panic!(
                "adding command would overwrite existing an command named `{}`",
                command.name
            );
        }
        self.cache
            .insert(command.name.clone(), Arc::clone(&command));
        for alias in &command.aliases {
            if self.cache.contains_key(alias) || alias == "?" || alias == "h" {
                panic!(
                    "adding command would overwrite existing an command with the alias `{}`",
                    alias
                );
            }
            self.cache.insert(alias.clone(), Arc::clone(&command));
        }
    }
}

/// A closure that evaluates some input with some context type,
/// and returns either `Some(String)` or `None`. `None` indicates to the
/// REPL handler that it should break the loop.
pub type Eval<Ctx> = Box<dyn Fn(&mut Ctx, String) -> Option<String>>;

pub struct Command<Ctx> {
    /// The command's name
    name: String,
    /// The description of the command
    description: String,
    /// The help description of the command
    help: String,
    /// Aliases for this command
    aliases: Vec<String>,
    /// A closure that evaluates the command's input (excluding the command name and leading space) and returns something implementing `Display`.
    eval: Eval<Ctx>,
}

impl<Ctx> Command<Ctx> {
    #[must_use]
    pub fn new(name: String, description: String, help: String, eval: Eval<Ctx>) -> Self {
        Self {
            name,
            description,
            help,
            aliases: Vec::new(),
            eval,
        }
    }

    #[must_use]
    /// Extend the aliases vector
    pub fn aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases.extend(aliases);
        self
    }

    #[must_use]
    /// Add an alias
    pub fn alias(mut self, alias: String) -> Self {
        self.aliases.push(alias);
        self
    }
}

/*
pub extern crate actix;
pub extern crate actix_rt;
use actix::prelude::*;
use futures::executor::block_on;
use std::time::Duration;

/// An Actix message to evaulate an input. For commands, this is given the
/// arguments to the command (if none, then just an empty string), minus the
/// space at the beginning. For the main evaluator, this is given the input to
/// the REPL.
#[derive(Message)]
#[rtype(result = "String")]
pub struct Evaluate(pub String);

/// A marker trait for an evaluator actor. These either handle evaluating
/// commands or the main input from the REPL. It must implement `Actor`
/// and `Handler<Evaluate>`. It is automatically implemented for all types
/// that meet these requirements.
pub trait Evaluator: Actor + Handler<Evaluate> {}

impl<T> Evaluator for T where T: Actor + Handler<Evaluate> {}

/// An Actix message to load code or a commmand. This is sent to the control
/// thread so that it can decide where to route it.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Load(pub String);

/// An Actix message to tell the REPL to quit. This is sent to the control
/// thread to tell the REPL to quit.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Quit;

/// An internal Actix message telling the User actor to start accepting input.
#[derive(Message)]
#[rtype(result = "()")]
struct StartUserInput(Addr<Control>);

pub struct User {
    pub rl: Editor<()>,
}

impl Actor for User {
    type Context = SyncContext<Self>;
}

impl Handler<StartUserInput> for User {
    type Result = ();
    fn handle(&mut self, msg: StartUserInput, _ctx: &mut Self::Context) -> Self::Result {
        for _ in 0..10 {
            msg.0.do_send(Load("a".to_string()));
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}

#[derive(Default)]
pub struct Control;

impl Actor for Control {
    type Context = Context<Self>;
}

impl Supervised for Control {}

impl ArbiterService for Control {
    fn service_started(&mut self, ctx: &mut Context<Self>) {
        let addr = SyncArbiter::start(1, || User {
            rl: Editor::<()>::new(),
        });
        addr.do_send(StartUserInput(ctx.address()));
        // Arbiter::set_item(addr);
    }
}

impl Handler<Load> for Control {
    type Result = ();

    fn handle(&mut self, msg: Load, _ctx: &mut Context<Self>) {
        println!("{}", msg.0);
    }
}*/
