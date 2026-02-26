use dioxus::prelude::*;

use crate::{backend::db::load_db, components::{AppsTable, EntryForm, tables::Application}};

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

    let table = tmp.map(|q| q.unwrap()).collect::<Vec<Application>>();

    use_context_provider(|| table);

    rsx! {
        h1 { "Job Application Tracker" }
        EntryForm {}
        AppsTable { }
    }
}
