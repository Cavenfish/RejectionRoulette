use dioxus::prelude::*;
use backend::{AppDB, Application, Database, plots::{stats_pie, stats_sankey}};

use crate::components::AppsTable;

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
    let stats = db.get_stats().unwrap();

    let table: Signal<Vec<Application>> = use_signal(|| {
        let mut apps = db.pull_all().unwrap();
        apps.reverse();
        apps
    });

    let sankey = stats_sankey(&stats).unwrap();

    rsx! {
        div {
            class: "stats-pie",
            dangerous_inner_html: sankey,
        }
        div {
            class: "recent-apps",
            AppsTable { table }
        }
    }
}