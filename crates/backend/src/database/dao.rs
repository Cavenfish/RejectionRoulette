use std::{collections::BTreeMap, fs};

use anyhow::Result;
use chrono::{Local, NaiveDate};
use dirs::data_dir;
use rusqlite::{Connection, params};

use crate::database::schema::{NewApplication, NewInterview, NewOffer};

use super::schema::{Application, Interview, Offer, RowInsert, RowRead, init_db};

//TODO
// make funcs:
//  - search apps
//  - search interviews
//

#[derive(Debug)]
pub struct AppDB {
    connection: Connection,
}

impl AppDB {
    pub fn new() -> Self {
        let filename = data_dir().unwrap().join("RejectionRoulette/main.db");
        let save_dir = filename.parent().unwrap();

        if !save_dir.exists() {
            fs::create_dir_all(&save_dir).expect("Failed to create data dir");
        }

        let connection = Connection::open(&filename).unwrap();

        init_db(&connection).expect("Failed to init db");

        Self { connection }
    }

    pub fn scan_for_ghosts(&self) -> Result<()> {
        let mut applications = self.get_applications()?;

        for app in applications.iter_mut() {
            let today = Local::now().date_naive();
            let sent = NaiveDate::parse_from_str(&app.submit_date, "%Y/%m/%d")?;

            if today.signed_duration_since(sent).num_weeks() >= 8
                && app.status.as_str() == "Pending"
            {
                self.update_application(app.id, "Ghost".to_string())?;
            };
        }

        Ok(())
    }

    pub fn get_stats(&self) -> Result<BTreeMap<String, u32>> {
        let applications = self.get_applications()?;
        let mut stats: BTreeMap<String, u32> = BTreeMap::new();

        for app in applications.iter() {
            *stats.entry(app.status.clone()).or_insert(0) += 1;
        }

        Ok(stats)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        // Deletes cascading
        self.connection
            .execute("DELETE FROM Applications WHERE id=?1", params![id])?;

        Ok(())
    }

    pub fn get_applications(&self) -> Result<Vec<Application>> {
        let mut stmt = self.connection.prepare("SELECT * FROM Applications")?;

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
            SET company=?1, role=?2, status=?3, submit_date=?4
            WHERE id=?5
            ",
            params![item.company, item.role, item.status, item.submit_date, id],
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
}
