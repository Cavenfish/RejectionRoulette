use std::fs;

use clap::{Subcommand, Parser, Args};
use colored::Colorize;

use backend::{AppDB, Application, Database};

#[derive(Parser)]
pub struct RrArgs {
    #[clap(subcommand)]
    pub command: RrCommands
}

#[derive(Subcommand)]
pub enum RrCommands {
    /// Add
    Add(AddArgs),

    /// Remove
    Remove(RemoveArgs),

    /// Update
    Update(UpdateArgs),

    /// Edit
    Edit(EditArgs),

    /// Show
    Show,

    /// Stats
    Stats
}

#[derive(Args)]
pub struct AddArgs {
    /// Company
    #[arg(short)]
    pub company: String,

    /// Role
    #[arg(short)]
    pub role: String,

    /// Status
    #[arg(short)]
    pub status: String,

    /// Date in YEAR/MONTH/DAY (defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Item ID
    #[arg(short)]
    pub id: i64,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Item ID
    #[arg(short)]
    pub id: i64,

    /// Status
    #[arg(short)]
    pub status: String,
}

#[derive(Args)]
pub struct EditArgs {
    /// Item ID
    #[arg(short)]
    pub id: i64,

    /// Company
    #[arg(short)]
    pub company: String,

    /// Role
    #[arg(short)]
    pub role: String,

    /// Status
    #[arg(short)]
    pub status: String,

    /// Date in YEAR/MONTH/DAY (defaults to today)
    #[arg(long, default_value = "today")]
    pub date: String,
}


fn show() {
    let db = AppDB::new();
    db.scan_for_ghosts().unwrap();

    let apps: Vec<Application> = db.pull_all().unwrap();

    println!(
        "{: <5} {: <15} {: <25} {: <15} {: <10}",
        "ID", "Company", "Role", "Submit Date", "Status"
    );

    println!("{:-<75}", "");

    for app in apps.iter() {
        let company = if app.company.chars().count() > 14 {
            &(app.company.chars().take(11).collect::<String>() + "...")
        } else {
            &app.company
        };

        let role = if app.role.chars().count() > 24 {
            &(app.role.chars().take(21).collect::<String>() + "...")
        } else {
            &app.role
        };

        let status = match app.status.as_str() {
            "Pending" => app.status.yellow(),
            "Rejected" => app.status.red(),
            "Interview" => app.status.green(),
            _ => app.status.white()
        };

        println!(
            "{: <5} {: <15} {: <25} {: <15} {: <10}",
            app.id.unwrap(), company, role, app.date, status
        );

    }

}

fn stats() {
    let db = AppDB::new();
    db.scan_for_ghosts().unwrap();

    let stats = db.get_stats().unwrap();

    for (key, value) in stats {
        println!("{}: {}", key, value);
    }
}

fn add(cmds: AddArgs) {
    let db = AppDB::new();

    let app = Application {
        id: None,
        company: cmds.company,
        role: cmds.role,
        date: cmds.date,
        status: cmds.status
    };

    db.insert(app).unwrap();
}

fn remove(cmds: RemoveArgs) {
    let db = AppDB::new();

    <AppDB as Database<Application>>::delete(&db, cmds.id).unwrap();
}

fn update(cmds: UpdateArgs) {
    let db = AppDB::new();
    <AppDB as Database<Application>>::update(&db, cmds.id, cmds.status).unwrap();
}

fn edit(cmds: EditArgs) {
    let db = AppDB::new();

    let app = Application {
        id: None,
        company: cmds.company,
        role: cmds.role,
        date: cmds.date,
        status: cmds.status
    };

    db.edit(app, cmds.id).unwrap();
}

fn main() {
    let args = RrArgs::parse();

    match args.command {
        RrCommands::Add(cmds) => add(cmds),
        RrCommands::Remove(cmds) => remove(cmds),
        RrCommands::Update(cmds) => update(cmds),
        RrCommands::Edit(cmds) => edit(cmds),
        RrCommands::Show => show(),
        RrCommands::Stats => stats(),
    }
}
