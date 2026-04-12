use backend::database::{Application, NewApplication};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobApplication {
    pub company: String,
    pub role: String,
    pub date: String,
    pub status: String,
}

#[component]
pub fn EntryForm() -> Element {
    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Add new job application" }
            form {
                onsubmit: move |evt: FormEvent| async move {
                    // let values: JobApplication = evt.parsed_values().unwrap();
                },

                div {
                    class: "form-group",
                    label { "Company" }
                    input { r#type: "text", placeholder: "e.g. Google" }
                }

                div {
                    class: "form-group",
                    label { "Role" }
                    input { r#type: "text", placeholder: "e.g. Software Engineer, Backend" }
                }

                div {
                    class: "form-group",
                    label { "Date" }
                    input { r#type: "text", placeholder: "e.g. 2026/04/10" }
                }

                div {
                    class: "form-actions",
                    button { "Submit" }
                }
            }
        }
    }
}
