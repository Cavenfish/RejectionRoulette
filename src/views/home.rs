use dioxus::prelude::*;

use crate::{backend::db::load_db, components::{AppsTable, EntryForm, tables::Application}};

const STYLE: Asset = asset!("assets/styling/main.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let db = load_db()?;
    let mut stmt = db.prepare("SELECT * FROM applications")?;

    let tmp = stmt.query_map([], |row| {
        Ok(Application {
            id: row.get(0)?,
            company: row.get(1)?,
            role: row.get(2)?,
            date: row.get(3)?
        })
    })?;

    let table = use_signal(|| tmp.map(|q| q.unwrap()).collect::<Vec<Application>>());

    rsx! {
        document::Stylesheet { href: STYLE }
        h1 { "Job Application Tracker" }
        AppsTable { table }
    }
}
