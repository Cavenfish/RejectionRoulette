use anyhow::Result;
use backend::database::{AppDB, Application};
use colored::Colorize;

use super::args::{AddArgs, EditArgs, RemoveArgs, UpdateArgs};

pub fn show() -> Result<()> {
    let db = AppDB::new();
    db.scan_for_ghosts()?;

    let apps: Vec<Application> = db.get_applications()?;

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
            _ => app.status.white(),
        };

        println!(
            "{: <5} {: <15} {: <25} {: <15} {: <10}",
            app.id.unwrap(),
            company,
            role,
            app.submit_date,
            status
        );
    }

    Ok(())
}

pub fn stats() -> Result<()> {
    let db = AppDB::new();
    db.scan_for_ghosts()?;

    let stats = db.get_stats()?;

    for (key, value) in stats {
        println!("{}: {}", key, value);
    }

    Ok(())
}

pub fn add(cmds: AddArgs) -> Result<()> {
    let db = AppDB::new();

    let app = Application {
        id: None,
        company: cmds.company,
        role: cmds.role,
        status: cmds.status,
        submit_date: cmds.date,
    };

    db.add_application(app)
}

pub fn remove(cmds: RemoveArgs) -> Result<()> {
    let db = AppDB::new();

    db.delete(cmds.id)
}

pub fn update(cmds: UpdateArgs) -> Result<()> {
    let db = AppDB::new();

    db.update_application(cmds.id, cmds.status)
}

pub fn edit(cmds: EditArgs) -> Result<()> {
    let db = AppDB::new();

    let app = Application {
        id: None,
        company: cmds.company,
        role: cmds.role,
        status: cmds.status,
        submit_date: cmds.date,
    };

    db.edit_application(app, cmds.id)
}
