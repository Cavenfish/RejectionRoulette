use anyhow::Result;
use backend::database::{
    AppDB, Application, Interview, NewApplication, NewInterview, NewOffer, Offer,
};
use chrono::Local;
use colored::Colorize;

use crate::args::{AddCommand, AddSubcommand, ShowArgs};

use super::args::{EditArgs, RemoveArgs, UpdateArgs};

pub fn stats() -> Result<()> {
    let db = AppDB::new();
    db.scan_for_ghosts()?;

    let stats = db.get_stats()?;

    for (key, value) in stats {
        println!("{}: {}", key, value);
    }

    Ok(())
}

pub fn add(cmd: AddCommand) -> Result<()> {
    let db = AppDB::new();

    match cmd.command {
        AddSubcommand::Application(cmds) => {
            let status = if cmds.rejected {
                "Rejected".to_string()
            } else {
                "Pending".to_string()
            };

            let submit_date = match cmds.date.as_str() {
                // Handle default value
                "today" => {
                    let tmp = Local::now();

                    tmp.format("%Y/%m/%d").to_string()
                }

                // Handle user input
                _ => cmds.date,
            };

            let app = NewApplication {
                resume_id: None,
                company: cmds.company,
                role: cmds.role,
                location: cmds.location,
                status,
                submit_date,
            };

            db.add_application(app)
        }
        AddSubcommand::Interview(cmds) => {
            let interview = NewInterview {
                application_id: cmds.id,
                interview_date: cmds.date,
                interview_type: cmds.category,
                notes: cmds.notes,
            };

            db.add_interview(interview)
        }
        AddSubcommand::Offer(cmds) => {
            let offer = NewOffer {
                application_id: cmds.id,
                base_salary: cmds.salary,
                bonus: cmds.bonus,
                equity_details: cmds.equity,
                expiration_date: cmds.date,
                is_accepted: cmds.accepted,
            };

            db.add_offer(offer)
        }
    }
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

    let app = NewApplication {
        resume_id: None,
        company: cmds.company,
        role: cmds.role,
        location: cmds.location,
        status: cmds.status,
        submit_date: cmds.date,
    };

    db.edit_application(app, cmds.id)
}

pub fn show(cmds: ShowArgs) -> Result<()> {
    let db = AppDB::new();
    db.scan_for_ghosts()?;

    if cmds.applications {
        show_applications(&db)?;
    };

    if cmds.interviews {
        show_interviews(&db)?;
    }

    if cmds.offers {
        todo!();
    }

    Ok(())
}

fn show_applications(db: &AppDB) -> Result<()> {
    let apps = db.get_applications()?;

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
            app.id, company, role, app.submit_date, status
        );
    }

    Ok(())
}

fn show_interviews(db: &AppDB) -> Result<()> {
    let interviews = db.get_interviews()?;

    println!(
        "{: <5} {: <15} {: <25} {: <10}",
        "ID", "Company", "Type", "Date"
    );

    println!("{:-<75}", "");

    for item in interviews.iter() {
        let company = if item.company.chars().count() > 14 {
            &(item.company.chars().take(11).collect::<String>() + "...")
        } else {
            &item.company
        };

        println!(
            "{: <5} {: <15} {: <25} {: <10}",
            item.id, company, item.interview_type, item.interview_date
        );
    }

    Ok(())
}
