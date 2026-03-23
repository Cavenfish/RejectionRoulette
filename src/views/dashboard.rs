use dioxus::prelude::*;
use backend::{AppDB, Database, Application, plots::stats_pie};

use crate::components::AppsTable;

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
    let stats = db.get_stats().unwrap();

    let table: Signal<Vec<Application>> = use_signal(|| db.pull_all().unwrap());

    let pie= stats_pie(&stats).unwrap();

    rsx! {
        div {
            class: "stats-pie",
            dangerous_inner_html: pie,
        }
        div {
            class: "recent-apps",
            AppsTable { table }
        }
    }
}