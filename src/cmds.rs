use crate::args::ArgCommand;
use crate::base::{NAME, VERSION};

pub fn cmd_version() {
    println!("{} Version: v{}", NAME, VERSION);    
}

pub fn cmd_handle_cmd(cmd_arg: &ArgCommand) -> bool {
    if let ArgCommand::Version = cmd_arg {
        cmd_version();
        return true;
    }

    return false;
}