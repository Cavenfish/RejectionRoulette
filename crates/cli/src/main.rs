mod args;
mod cmds;

use clap::Parser;

use args::{RrArgs, RrCommands};
use cmds::{add, edit, remove, show, stats, update};

fn main() {
    let args = RrArgs::parse();

    match args.command {
        RrCommands::Add(cmds) => add(cmds),
        RrCommands::Remove(cmds) => remove(cmds),
        RrCommands::Update(cmds) => update(cmds),
        RrCommands::Edit(cmds) => edit(cmds),
        RrCommands::Show(cmds) => show(cmds),
        RrCommands::Stats => stats(),
    }
    .unwrap();
}
