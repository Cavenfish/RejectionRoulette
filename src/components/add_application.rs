use backend::database::Application;
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
pub fn EntryForm(mut table: WriteSignal<Vec<Application>>) -> Element {
    rsx! {
        h3 { "Add new job application" }
        form {
            onsubmit: move |evt: FormEvent| async move {

                let values: JobApplication = evt.parsed_values().unwrap();

                table.push(
                    Application {
                        id: None,
                        company: values.company,
                        role: values.role,
                        status: values.status,
                        submit_date: values.date,
                    }
                );
            },
            label { "Company" }
            input { r#type: "text", id: "company", name: "company" }
            br {  }

            label { "Role" }
            input { r#type: "text", id: "role", name: "role" }
            br {  }

            label { "Date" }
            input { r#type: "text", id: "date", name: "date" }
            br {  }

            button { "Submit" }
         }
    }
}
