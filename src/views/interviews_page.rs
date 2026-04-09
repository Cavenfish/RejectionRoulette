use backend::database::AppDB;
use dioxus::prelude::*;

use crate::components::InterviewsTable;

#[component]
pub fn InterviewsPage() -> Element {
    let db = AppDB::new();

    let table = use_signal(|| db.get_interviews().unwrap());

    rsx! {
        h1 { "Job Application Tracker" }
        InterviewsTable { table }
    }
}
