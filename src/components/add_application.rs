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
        h3 { "Add new job application" }
        form {
            onsubmit: move |evt: FormEvent| async move {
                // let values: JobApplication = evt.parsed_values().unwrap();
            },
            label { "Company" }
            input { r#type: "text", placeholder: "e.g. Google" }
            br {  }

            label { "Role" }
            input { r#type: "text", placeholder: "e.g. Software Engineer, Backend" }
            br {  }

            label { "Date" }
            input { r#type: "text", placeholder: "e.g. 2026/04/10" }
            br {  }

            button { "Submit" }
        }
    }
}
