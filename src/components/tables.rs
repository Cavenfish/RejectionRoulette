use backend::database::{Application, Interview};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::EntryForm;

#[component]
pub fn ApplicationsTable(table: WriteSignal<Vec<Application>>) -> Element {
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
                            td { "{item.id}" }
                            td { "{item.company}" }
                            td { "{item.role}" }
                            td { "{item.submit_date}" }
                            td { "{item.status}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn InterviewsTable(table: WriteSignal<Vec<Interview>>) -> Element {
    rsx! {
        div {
            class: "table-title",
            h3 { "Interviews" }
        }
        div {
            class: "apps-table",
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Company" }
                        th { "Role" }
                        th { "Type" }
                        th { "Date" }
                    }
                }
                tbody {
                    for item in table.iter() {
                        tr {
                            td { "{item.id}" }
                            td { "{item.company}" }
                            td { "{item.role}" }
                            td { "{item.interview_type}" }
                            td { "{item.interview_date}" }
                        }
                    }
                }
            }
        }
    }
}
