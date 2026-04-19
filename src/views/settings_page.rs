use backend::settings::AppSettings;
use dioxus::{document::eval, prelude::*};

#[component]
pub fn SettingsPage() -> Element {
    let mut settings = AppSettings::load();
    let mut theme_state = use_signal(|| "dark".to_string());
    let mut auto_save = use_signal(|| true);

    rsx! {
        div { class: "settings-container",
            h1 { "Settings" }

            // Appearance Section
            div { class: "settings-card",
                h3 { "Appearance" }
                div { class: "settings-group",
                    div { class: "setting-item",
                        div { class: "info",
                            label { "Desktop Theme" }
                            span { "Choose your visual interface style." }
                        }
                        select {
                            value: "{theme_state}",
                            onchange: move |evt| {
                                let theme = evt.value();
                                settings.theme = theme;
                                let eval_stmt = settings.get_eval_stmt().unwrap();
                                eval(&eval_stmt);
                                settings.save().unwrap();
                            },
                            option { value: "dark", "Dark (default)" }
                            option { value: "light", "Light" }
                        }
                    }
                }
            }

            // Data & Security Section
            div { class: "settings-card",
                h3 { "Data Management" }
                div { class: "settings-group",
                    div { class: "setting-item",
                        div { class: "info",
                            label { "Auto-save Database" }
                            span { "Automatically commit changes to SQLite on input." }
                        }
                        input {
                            r#type: "checkbox",
                            checked: "{auto_save}",
                            onchange: move |_| auto_save.toggle()
                        }
                    }
                    div { class: "setting-item",
                        div { class: "info",
                            label { "Export Data" }
                            span { "Download your application history as a CSV." }
                        }
                        button {
                            class: "btn-submit", // Reusing your form button style
                            "Export CSV"
                        }
                    }
                }
            }

            // App Info Section
            div { class: "settings-card",
                h3 { "About" }
                div { class: "settings-group",
                    div { class: "setting-item",
                        div { class: "info",
                            label { "Version" }
                            span { "Rejection Roulette v1.0.4-beta" }
                        }
                    }
                }
            }
        }
    }
}
