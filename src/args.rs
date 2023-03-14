use std::env;
use std::collections::HashSet;
use std::mem::{Discriminant, discriminant};

pub enum ArgCommand {
    Run(String),
    Version,
}

pub fn get_env_arg_cmds() -> Vec<ArgCommand> {
    let args: Vec<String> = env::args().collect();
    let mut commands: Vec<ArgCommand> = Vec::new();
    let mut used_commands: HashSet<Discriminant<ArgCommand>> = HashSet::new();
    for arg in args.iter().skip(1) {
        if arg == "--version" || arg == "-v" {
            let value = ArgCommand::Version;
            if !used_commands.contains(&discriminant(&value)) {
                used_commands.insert(discriminant(&value));
                commands.push(value);
            }
        } else {
            let value = ArgCommand::Run(arg.clone());
            if !used_commands.contains(&discriminant(&value)) {
                used_commands.insert(discriminant(&value));
                commands.push(value);
            }
        }
    }

    return commands;
}