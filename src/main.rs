#![allow(unused)]

mod utils;
mod ltypes;
mod stack;
mod src;
mod args;
mod cmds;
mod base;
mod compile;
mod asm;

use utils::{IS_DEBUG, file_exists};
use args::{get_env_arg_cmds, ArgCommand};
use cmds::cmd_handle_cmd;
use compile::Compiler;
use asm::*;

fn main() {
    let commands = get_env_arg_cmds();
    if commands.len() == 0 {
        println!("No Ktnack file specified!");
        return;
    }

    let mut run_arg: Option<&String> = Option::None;

    for cmd in commands.iter() {
        if cmd_handle_cmd(cmd) {
            continue;
        } else if let ArgCommand::Run(file_name) = cmd {
            run_arg = Option::Some(file_name);
        }
    }

    if let Option::Some(file_name) = run_arg {
        run(file_name);
    }
}

fn run(file_name: &String) {
    if !file_exists(file_name) {
        println!("Ktnack file not found: {}", file_name);
        return;
    }

    let mut compiler = Compiler::new(file_name.as_str());
    compiler.compile();    
}
