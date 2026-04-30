use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate};
use dirs::data_dir;
use rusqlite::{Connection, params};

use crate::{
    database::schema::{NewApplication, NewInterview, NewOffer, NewResume, Resume},
    plots::{Stats, StatusData},
};

use super::schema::{AllTables, Application, Interview, Offer, RowInsert, RowRead, init_db};

#[derive(Debug)]
pub struct AppDB {
    connection: Connection,
    resume_dir: PathBuf,
}

impl AppDB {
    pub fn new() -> Self {
        let filename = data_dir().unwrap().join("RejectionRoulette/main.db");
        let save_dir = filename.parent().unwrap();

        if !save_dir.exists() {
            fs::create_dir_all(&save_dir).expect("Failed to create data dir");
        }

        let resume_dir = save_dir.join("Resumes");

        if !resume_dir.exists() {
            fs::create_dir_all(&resume_dir).expect("Failed to create resume dir");
        }

        let connection = Connection::open(&filename).unwrap();

        init_db(&connection).expect("Failed to init db");

        Self {
            connection,
            resume_dir,
        }
    }

    pub fn get_all_tables(&self) -> Result<AllTables> {
        let applications = self.get_applications()?;
        let interviews = self.get_interviews()?;
        let offers = self.get_offers()?;
        let resumes = self.get_resumes()?;

        Ok(AllTables {
            applications,
            interviews,
            offers,
            resumes,
        })
    }

    pub fn scan_for_ghosts(&self, num_weeks: i64) -> Result<()> {
        let mut applications = self.get_applications()?;
        let today = Local::now().date_naive();

        for app in applications.iter_mut() {
            let sent = NaiveDate::parse_from_str(&app.submit_date, "%Y/%m/%d")?;

            if today.signed_duration_since(sent).num_weeks() >= num_weeks
                && app.status.as_str() == "Pending"
            {
                self.update_application(app.id, "Ghost".to_string())?;
            };
        }

        Ok(())
    }

    pub fn get_date_range(&self) -> Result<(String, String)> {
        let this_year = Local::now().date_naive().year();
        let first: String = self.connection.query_one(
            "SELECT submit_date FROM Applications WHERE id=1",
            [],
            |r| r.get(0),
        )?;

        let start = first
            .splitn(3, "/")
            .enumerate()
            .map(|(i, value)| {
                if i == 2 {
                    "01".to_string()
                } else {
                    value.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("-");

        let end = format!("{}-12-31", this_year);

        Ok((start, end))
    }

    pub fn get_stats(&self) -> Result<Stats> {
        let applications = self.get_applications()?;
        let mut sankey = StatusData::new();
        let mut resumes: HashMap<String, i64> = HashMap::new();
        let mut dates: HashMap<String, i64> = HashMap::new();

        for app in applications.iter() {
            sankey.add_one(&app.status);
            *dates.entry(app.submit_date.clone()).or_insert(0) += 1;

            if app.status.as_str() == "Interview" {
                if let Some(name) = &app.resume {
                    *resumes.entry(name.clone()).or_insert(0) += 1;
                } else {
                    *resumes.entry("Unknown".to_string()).or_insert(0) += 1;
                }
            }
        }

        Ok(Stats {
            sankey,
            resumes,
            dates,
        })
    }

    pub fn get_ghost_alerts(&self, num_weeks: i64) -> Result<Vec<Application>> {
        let mut applications = self.get_applications()?;
        let today = Local::now().date_naive();

        applications.retain(|a| {
            let sent = NaiveDate::parse_from_str(&a.submit_date, "%Y/%m/%d").unwrap();

            today.signed_duration_since(sent).num_weeks() >= num_weeks - 1
                && today.signed_duration_since(sent).num_weeks() <= num_weeks
                && a.status == "Pending"
        });

        Ok(applications)
    }

    pub fn get_upcoming_interviews(&self) -> Result<Vec<Interview>> {
        let mut interviews = self.get_interviews()?;
        let today = Local::now().date_naive();

        interviews.retain(|i| {
            let date = NaiveDate::parse_from_str(&i.interview_date, "%Y/%m/%d").unwrap();
            date > today
        });

        Ok(interviews)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        // Deletes cascading
        self.connection
            .execute("DELETE FROM Applications WHERE id=?1", params![id])?;

        Ok(())
    }

    pub fn get_applications(&self) -> Result<Vec<Application>> {
        let mut stmt = self.connection.prepare(
            "SELECT a.id, r.id, a.company, a.role, a.location, a.status, r.name, a.submit_date
            FROM Applications AS a LEFT JOIN Resumes AS r ON r.id = a.resume_id",
        )?;

        let tmp = stmt.query_map([], |row| Ok(Application::from_row(row)?))?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }

    pub fn get_recent_applications(&self, n: i64) -> Result<Vec<Application>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM Applications ORDER BY id DESC LIMIT ?1;")?;

        let tmp = stmt.query_map(params![n], |row| Ok(Application::from_row(row)?))?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }

    pub fn add_application(&self, item: NewApplication) -> Result<()> {
        item.add_row(&self.connection)?;

        Ok(())
    }

    pub fn update_application(&self, id: i64, status: String) -> Result<()> {
        self.connection.execute(
            "UPDATE Applications
            SET status=?1 WHERE id=?2",
            params![status, id],
        )?;

        Ok(())
    }

    pub fn edit_application(&self, item: NewApplication, id: i64) -> Result<()> {
        self.connection.execute(
            "UPDATE Applications
            SET resume_id=?1, company=?2, role=?3, location=?4, status=?5, submit_date=?6
            WHERE id=?7
            ",
            params![
                item.resume_id,
                item.company,
                item.role,
                item.location,
                item.status,
                item.submit_date,
                id
            ],
        )?;

        Ok(())
    }

    pub fn get_interviews(&self) -> Result<Vec<Interview>> {
        let mut stmt = self.connection.prepare(
            "SELECT i.id, a.id, a.company, a.role, i.interview_date, i.interview_type, i.notes
            FROM Interviews AS i JOIN Applications AS a ON a.id = i.application_id",
        )?;

        let tmp = stmt.query_map([], |row| Ok(Interview::from_row(row)?))?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }

    pub fn get_recent_interviews(&self, n: i64) -> Result<Vec<Interview>> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM Interviews ORDER BY id DESC LIMIT ?1;")?;

        let tmp = stmt.query_map(params![n], |row| Ok(Interview::from_row(row)?))?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }

    pub fn add_interview(&self, item: NewInterview) -> Result<()> {
        item.add_row(&self.connection)?;

        Ok(())
    }

    pub fn delete_interview(&self, id: i64) -> Result<()> {
        self.connection
            .execute("DELETE FROM Interviews WHERE id=?1", params![id])?;

        Ok(())
    }

    pub fn update_interview(&self, id: i64, notes: String) -> Result<()> {
        self.connection.execute(
            "UPDATE Interviews SET notes=?1 WHERE id=?2",
            params![notes, id],
        )?;

        Ok(())
    }

    pub fn edit_interview(&self, item: NewInterview, id: i64) -> Result<()> {
        self.connection.execute(
            "UPDATE Interviews
            SET application_id=?1, interview_date=?2, interview_type=?3, notes=?4
            WHERE id=?5
            ",
            params![
                item.application_id,
                item.interview_date,
                item.interview_type,
                item.notes,
                id
            ],
        )?;

        Ok(())
    }

    pub fn get_offers(&self) -> Result<Vec<Offer>> {
        let mut stmt = self.connection.prepare(
            "SELECT o.id, a.id, a.company, a.role, o.base_salary, o.bonus, o.equity_details, o.expiration_date, o.is_accepted
            FROM Offers AS o JOIN Applications AS a ON a.id = o.application_id",
        )?;

        let tmp = stmt.query_map([], |row| Ok(Offer::from_row(row)?))?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }

    pub fn add_offer(&self, item: NewOffer) -> Result<()> {
        item.add_row(&self.connection)?;

        Ok(())
    }

    pub fn delete_offer(&self, id: i64) -> Result<()> {
        self.connection
            .execute("DELETE FROM Offers WHERE id=?1", params![id])?;

        Ok(())
    }

    pub fn edit_offer(&self, item: NewOffer, id: i64) -> Result<()> {
        self.connection.execute(
            "UPDATE Offers
            SET application_id=?1, base_salary=?2, bonus=?3, equity_details=?4, expiration_date=?5, is_accepted=?6
            WHERE id=?7
            ",
            params![
                item.application_id,
                item.base_salary,
                item.bonus,
                item.equity_details,
                item.expiration_date,
                item.is_accepted,
                id
            ],
        )?;

        Ok(())
    }

    pub fn add_resume(&self, resume: NewResume) -> Result<()> {
        resume.add_row(&self.connection)?;

        Ok(())
    }

    pub fn get_file(&self, resume: &Resume) -> Result<PathBuf> {
        let mut file_path = self.resume_dir.join(&resume.hash);
        file_path.set_extension("pdf");

        Ok(file_path)
    }

    pub fn get_resumes(&self) -> Result<Vec<Resume>> {
        let mut stmt = self.connection.prepare("SELECT * FROM Resumes")?;

        let tmp = stmt.query_map([], |row| Ok(Resume::from_row(row)?))?;

        Ok(tmp.map(|q| q.unwrap()).collect())
    }
}
