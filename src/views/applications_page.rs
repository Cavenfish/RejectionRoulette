use backend::database::AppDB;
use dioxus::prelude::*;

use crate::components::ApplicationsTable;

#[component]
pub fn ApplicationsPage() -> Element {
    let db = AppDB::new();

    let table = use_signal(|| db.get_applications().unwrap());

    rsx! {
        h1 { "Job Application Tracker" }
        ApplicationsTable { table }
    }
}
