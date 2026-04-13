use backend::database::{Application, Interview};
use dioxus::prelude::*;

use super::{EditApplication, ModalOverlay};

fn get_status_color(status: &str) -> &'static str {
    match status {
        "Ghost" => "grey",
        "Rejected" => "red",
        "Interview" => "green",
        "Pending" => "yellow",
        _ => "white",
    }
}

#[component]
pub fn ApplicationsTable(table: Signal<Vec<Application>>) -> Element {
    let mut selected_item = use_signal(|| None::<Application>);

    rsx! {
        div {
            class: "apps-table",
            table {
                thead {
                    tr {
                        th { width: "4ch", "ID" }
                        th { width: "25%", "Company" }
                        th { width: "75%", "Role" }
                        th { width: "12ch", "Date" }
                        th { width: "10ch", "Status" }
                    }
                }
                tbody {
                    for item in table.read().iter().rev() {
                        {
                            let current_item = item.clone();
                            rsx! {
                                tr {
                                    style: "cursor: pointer;",
                                    class: "hover-highlight",
                                    onclick: move |_| {
                                        selected_item.set(Some(current_item.clone()));
                                    },
                                    td { "{item.id}" }
                                    td { "{item.company}" }
                                    td { "{item.role}" }
                                    td { "{item.submit_date}" }
                                    td {
                                        span {
                                            color: "{get_status_color(&item.status)}",
                                            font_weight: "bold",
                                            "{item.status}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(item) = selected_item() {
            ModalOverlay {
                on_close: move |_| selected_item.set(None),
                inner: rsx! {EditApplication {item, table, on_close: move |_| selected_item.set(None)}}
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

                        th { width: "4ch", "ID" }
                        th { width: "25%", "Company" }
                        th { width: "75%", "Role" }
                        th { width: "25ch", "Type" }
                        th { width: "12ch", "Date" }
                    }
                }
                tbody {
                    for item in table.read().iter().rev() {
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
