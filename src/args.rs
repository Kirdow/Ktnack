use std::env;
use std::collections::HashSet;
use std::mem::{Discriminant, discriminant};

pub enum ArgCommand {
    Run(String),
    Version,
}

struct ArgsParse {
    commands: Vec<ArgCommand>,
    used_commands: HashSet<Discriminant<ArgCommand>>
}

impl ArgsParse {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
            used_commands: HashSet::new(),
        }
    }

    fn add(&mut self, cmd: ArgCommand) -> bool {
        if self.used_commands.contains(&discriminant(&cmd)) { return false; }

        self.used_commands.insert(discriminant(&cmd));
        self.commands.push(cmd);
        return true;
    }

    fn complete(self) -> Vec<ArgCommand> {
        return self.commands;
    }
}

pub fn get_env_arg_cmds() -> Vec<ArgCommand> {
    let args: Vec<String> = env::args().collect();
    let mut parse = ArgsParse::new();
    
    for arg in args.iter().skip(1) {
        if arg == "--version" || arg == "-v" {
            parse.add(ArgCommand::Version);
        } else {
            parse.add(ArgCommand::Run(arg.clone()));
        }
    }

    return parse.complete();
}