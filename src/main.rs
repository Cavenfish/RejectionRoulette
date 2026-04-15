use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

mod components;
mod views;

use views::{ApplicationsPage, Dashboard, InterviewsPage, Navbar};

const MAIN_CSS: Asset = asset!("/assets/styling/main.scss");

/// The Route enum is used to define the structure of internal routes in our app. All route enums need to derive
/// the [`Routable`] trait, which provides the necessary methods for the router to work.
/// 
/// Each variant represents a different URL pattern that can be matched by the router. If that pattern is matched,
/// the components for that route will be rendered.
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // The layout attribute defines a wrapper for all routes under the layout. Layouts are great for wrapping
    // many routes with a common UI like a navbar.
    #[layout(Navbar)]
        // The route attribute defines the URL pattern that a specific route matches. If that pattern matches the URL,
        // the component for that route will be rendered. The component name that is rendered defaults to the variant name.
        #[route("/")]
        Dashboard {},

        #[route("/applications")]
        ApplicationsPage {},

        #[route("/interviews")]
        InterviewsPage {},
}

fn main() {
    // Define a custom window configuration
    let window = WindowBuilder::new()
        .with_title("Rejection Roulette")
        .with_resizable(true);

    // Create the configuration, disabling the default menu
    let cfg = Config::new().with_window(window).with_menu(None); // This hides the "File, Edit..." menu bar

    LaunchBuilder::new().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
