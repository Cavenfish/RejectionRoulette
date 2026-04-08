use backend::{
    database::{AppDB, Application},
    plots::{stats_pie, stats_sankey},
};
use dioxus::prelude::*;

use crate::components::AppsTable;

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
    let stats = db.get_stats().unwrap();

    let table: Signal<Vec<Application>> = use_signal(|| {
        let mut apps = db.get_applications().unwrap();
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
