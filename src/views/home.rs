use dioxus::prelude::*;

use crate::{components::{AppsTable, tables::Application}};

const STYLE: Asset = asset!("assets/styling/main.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {

    let table: Signal<Vec<Application>> = use_signal(|| Vec::new());

    rsx! {
        document::Stylesheet { href: STYLE }
        h1 { "Job Application Tracker" }
        AppsTable { table }
    }
}
