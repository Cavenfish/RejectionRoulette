use dioxus::{prelude::*};
use serde::{Deserialize, Serialize};
use backend::Application;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobApplication {
    pub company: String,
    pub role: String,
    pub date: String,
    pub status: String
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
                        id: Some(100), 
                        company: values.company, 
                        role: values.role, 
                        date: values.date,
                        status: values.status
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