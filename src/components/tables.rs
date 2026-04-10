use backend::database::{Application, Interview};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::EntryForm;

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
pub fn ApplicationsTable(table: WriteSignal<Vec<Application>>) -> Element {
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
            EditPopup {
                item: item,
                on_close: move |_| selected_item.set(None),
                // You'll pass your DB update/delete handlers here
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

#[derive(Props, Clone, PartialEq)]
pub struct EditProps {
    item: Application,
    on_close: EventHandler<()>,
}

#[component]
pub fn EditPopup(props: EditProps) -> Element {
    // Local state for the form inputs
    let mut company = use_signal(|| props.item.company.clone());
    let mut role = use_signal(|| props.item.role.clone());
    let mut status = use_signal(|| props.item.status.clone());

    rsx! {
        div {
            class: "modal-overlay",
            onclick: move |_| props.on_close.call(()), // Close if clicking background

            div {
                class: "modal-content",
                onclick: |evt| evt.stop_propagation(), // Prevent closing when clicking inside

                h2 { "Edit Application" }

                label { "Company" }
                input {
                    r#type: "text",
                    value: "{company}",
                    placeholder: "Hello Company",
                    oninput: move |e| company.set(e.value())
                }

                label { "Role" }
                input {
                    "value": "{role}",
                    oninput: move |e| role.set(e.value())
                }

                label { "Status" }
                select {
                    value: "{status}",
                    onchange: move |e| status.set(e.value()),
                    option { "Ghost" }
                    option { "Rejected" }
                    option { "Interview" }
                    option { "Pending" }
                }

                div {
                    style: "margin-top: 20px; display: flex; justify-content: space-between;",
                    button {
                        style: "background: #ff5555; color: white;",
                        onclick: move |_| {
                            // db.delete(props.item.id);
                            println!("Delete ID: {}", props.item.id);
                            props.on_close.call(());
                        },
                        "Delete"
                    }
                    div {
                        button {
                            style: "background: #44aa44; color: white; margin-left: 10px;",
                            onclick: move |_| {
                                // db.update(props.item.id, company(), role(), status());
                                println!("Update ID: {}", props.item.id);
                                props.on_close.call(());
                            },
                            "Update"
                        }
                    }
                }
            }
        }
    }
}
