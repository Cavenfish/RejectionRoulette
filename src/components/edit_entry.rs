use backend::database::{AppDB, Application, NewApplication};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EditApplicationProps {
    item: Application,
    table: Signal<Vec<Application>>,
    on_close: EventHandler<()>,
}

#[component]
pub fn EditApplication(mut props: EditApplicationProps) -> Element {
    // Local state for the form inputs
    let mut company = use_signal(|| props.item.company.clone());
    let mut role = use_signal(|| props.item.role.clone());
    let mut status = use_signal(|| props.item.status.clone());
    let mut date = use_signal(|| props.item.submit_date.clone());

    rsx! {
        div {
            class: "new-entry-form",
            h2 { "Edit Application" }

            form {

                div {
                    class: "form-group",
                    label { "Company" }
                    input {
                        r#type: "text",
                        value: "{company}",
                        oninput: move |e| company.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Role" }
                    input {
                        r#type: "text",
                        value: "{role}",
                        oninput: move |e| role.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Date" }
                    input {
                        r#type: "text",
                        value: "{date}",
                        oninput: move |e| date.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Status" }
                    select {
                        value: "{status}",
                        onchange: move |e| status.set(e.value()),
                        option { "Ghost" }
                        option { "Rejected" }
                        option { "Interview" }
                        option { "Pending" }
                    }
                }

                div {
                    class: "form-actions",

                    div {
                        class: "delete-btn",
                        button {
                            onclick: move |_| {
                                let db = AppDB::new();
                                db.delete(props.item.id).unwrap();
                                let updated_table = db.get_applications().unwrap();
                                props.table.set(updated_table);
                                props.on_close.call(());
                            },
                            "Delete"
                        }
                    }

                    div {
                        class: "submit-btn",
                        button {
                            onclick: move |_| {
                                let db = AppDB::new();

                                let updated = NewApplication {
                                    company: company(),
                                    role: role(),
                                    status: status(),
                                    submit_date: date(),
                                };

                                db.edit_application(updated, props.item.id).unwrap();
                                let updated_table = db.get_applications().unwrap();
                                props.table.set(updated_table);
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
