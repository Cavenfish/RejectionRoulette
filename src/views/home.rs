use backend::database::{AppDB, Application};
use dioxus::prelude::*;

use crate::components::AppsTable;

const STYLE: Asset = asset!("assets/styling/main.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let db = AppDB::new();

    let table: Signal<Vec<Application>> = use_signal(|| db.get_applications().unwrap());

    rsx! {
        document::Stylesheet { href: STYLE }
        h1 { "Job Application Tracker" }
        AppsTable { table }
    }
}
