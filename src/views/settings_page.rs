use backend::settings::AppSettings;
use dioxus::{document::eval, prelude::*};

#[component]
pub fn SettingsPage() -> Element {
    let version = env!("CARGO_PKG_VERSION");
    let mut settings = use_signal(|| AppSettings::load());

    rsx! {
        div { class: "settings-container",
            h1 { "Settings" }

            // Appearance Section
            div {
                class: "settings-card",
                h3 { "Appearance" }

                div {
                    class: "settings-group",
                    div {
                        class: "setting-item",
                        div {
                            class: "info",
                            label { "Desktop Theme" }
                            span { "Choose your visual interface style." }
                        }
                        select {
                            value: "{settings().theme}",
                            onchange: move |evt| {
                                let mut current = settings();
                                let theme = evt.value();
                                current.theme = theme;
                                let eval_stmt = current.get_eval_stmt().unwrap();
                                eval(&eval_stmt);
                                settings.set(current);
                            },
                            option { value: "dark", "Dark" }
                            option { value: "light", "Light" }
                            option { value: "casino", "Midnight Casino" }
                            option { value: "nord", "Nordic Calm" }
                            option { value: "terminal", "Vintage Terminal" }
                            option { value: "sepia", "Sepia Library" }
                            option { value: "cyberpunk", "Cyberpunk Neon" }
                            option { value: "coffee", "Coffee Shop" }
                            option { value: "ocean", "Deep Sea" }
                            option { value: "dracula", "Dracula Pro" }
                            option { value: "peach", "Peach Fuzz" }
                        }
                    }

                    button {
                        class: "save-btn",
                        onclick: move |_| settings().save().unwrap(),
                        "Save Settings"
                    }
                }
            }

            // Data Management
            div {
                class: "settings-card",
                h3 { "Data Management" }

                div {
                    class: "settings-group",

                    div {
                        class: "setting-item",
                        div {
                            class: "info",
                            label { "Export Data" }
                            span { "Download your application history as a CSV." }
                        }
                        button {
                            class: "basic-btn",
                            "Export CSV"
                        }
                    }
                }
            }

            // App Info Section
            div {
                class: "settings-card",
                h3 { "About" }

                div {
                    class: "settings-group",
                    div {
                        class: "setting-item",

                        div {
                            class: "info",
                            label { "GitHub Repo" }
                            span { "https://github.com/Cavenfish/RejectionRoulette" }
                        }

                        div {
                            class: "info",
                            label { "Version" }
                            span { "v{version}" }
                        }
                    }
                }
            }
        }
    }
}
