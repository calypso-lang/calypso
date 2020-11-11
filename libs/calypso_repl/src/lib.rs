#![doc(html_root_url = "https://thepuzzlemaker.github.io/Calypso/rustdoc/calypso_repl/index.html")]

use std::collections::HashMap;
use std::sync::Arc;

use rustyline::{config::Configurer, error::ReadlineError, Cmd, Editor, KeyPress, Movement};

use regex::Regex;

/*
== TODOs ==
- [ ] Color!
- [ ] Clean this code up
- [ ] Find if any helpful Rustyline key bindings are missing
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
        editor.bind_sequence(KeyPress::Ctrl('C'), Cmd::Kill(Movement::WholeLine));
        Self {
            eval,
            cmds: Vec::new(),
            cache: HashMap::new(),
            editor,
            ctx,
            prefix: String::from(":"),
            cmd_regex: Regex::new(r":(?P<command>\S*)( (?P<args>.*))?").unwrap(),
        }
    }

    pub fn prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        // We escape the prefix, so it's guaranteed to be valid.
        Regex::new(&format!(
            r"{}(?P<command>\S*)( (?P<args>.*))?",
            regex::escape(&self.prefix)
        ))
        .unwrap();
        self
    }

    /// Extend the commands vector
    pub fn commands(mut self, commands: Vec<Arc<Command<Ctx>>>) -> Self {
        for command in &commands {
            self.cache_command(Arc::clone(command));
        }
        self.cmds.extend(commands);
        self
    }

    /// Add a command
    pub fn command(mut self, command: Command<Ctx>) -> Self {
        let arc = Arc::new(command);
        self.cache_command(Arc::clone(&arc));
        self.cmds.push(arc);
        self
    }

    pub fn run(
        &mut self,
        preamble: String,
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
                            let args = if let Some(args) = args {
                                args.as_str()
                            } else {
                                ""
                            };
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

    fn cache_command(&mut self, command: Arc<Command<Ctx>>) {
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
// and returns either `Some(String)` or `None`. `None` indicates to the
// REPL handler that it should break the loop.
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
    pub fn new(name: String, description: String, help: String, eval: Eval<Ctx>) -> Self {
        Self {
            name,
            description,
            help,
            aliases: Vec::new(),
            eval,
        }
    }

    /// Extend the aliases vector
    pub fn aliases(mut self, aliases: Vec<String>) -> Self {
        self.aliases.extend(aliases);
        self
    }

    /// Add an alias
    pub fn alias(mut self, alias: String) -> Self {
        self.aliases.push(alias);
        self
    }
}
