use dioxus::{prelude::*};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub id: i64,
    pub company: String,
    pub role: String,
    pub date: String,
}

#[component]
pub fn AppsTable() -> Element {
    let table = use_context::<Vec<Application>>();

    rsx! {
        table { 
            thead { 
                tr { 
                    th { "Company" }
                    th { "Role" }
                    th { "Date" }
                 }
             }
             tbody { 
                for item in table.iter() {
                    tr {
                        td { "{item.company}" }
                        td { "{item.role}" }
                        td { "{item.date}" }
                    }
                }
              }
         }
    }
}