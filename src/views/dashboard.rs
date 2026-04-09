use backend::{
    database::{AppDB, Application},
    plots::{stats_pie, stats_sankey},
};
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
    let stats = db.get_stats().unwrap();

    let sankey = stats_sankey(&stats).unwrap();

    rsx! {
        div {
            class: "stats-pie",
            dangerous_inner_html: sankey,
        }
    }
}
