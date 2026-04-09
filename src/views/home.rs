use backend::database::{AppDB, Application};
use dioxus::prelude::*;

const STYLE: Asset = asset!("assets/styling/main.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let db = AppDB::new();

    rsx! {
        document::Stylesheet { href: STYLE }
        h1 { "Job Application Tracker" }
    }
}
