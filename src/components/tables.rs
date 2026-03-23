use dioxus::{prelude::*};
use serde::{Serialize, Deserialize};
use backend::Application;

use crate::components::EntryForm;


#[component]
pub fn AppsTable(table: WriteSignal<Vec<Application>>) -> Element {
    rsx! {
        div { 
            class: "table-title",
            h3 { "Applications" }
        }
        div {
            class: "apps-table",
            table { 
                thead { 
                    tr {
                        th { "ID" }
                        th { "Company" }
                        th { "Role" }
                        th { "Date" }
                        th { "Status" }
                    }
                }
                tbody { 
                    for item in table.iter() {
                        tr {
                            if let Some(id) = item.id {
                                td { "{id}" }
                            } else {
                                td { "--" }
                            }
                            td { "{item.company}" }
                            td { "{item.role}" }
                            td { "{item.date}" }
                            td { "{item.status}" }
                        }
                    }
                }
            }
        }
    }
}