use dioxus::{prelude::*};
use serde::{Serialize, Deserialize};
use backend::Application;

use crate::components::EntryForm;


#[component]
pub fn AppsTable(table: WriteSignal<Vec<Application>>) -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div { 
            class: "table-title",
            h3 { "Applications" }
            button {
                style: "width: 100px",
                onsubmit: move |_| visible.set(!visible()),
                "Add New"
            }
        }
        if visible() {
            div {
                class: "overlay",
                h1 {"HELLO WORLD"}
                // EntryForm { table }
            }
        } else {
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
}