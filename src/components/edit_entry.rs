use backend::database::{AppDB, Application, Interview, NewApplication, NewInterview};
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

                                // TODO: Check date is valid

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

#[derive(Props, Clone, PartialEq)]
pub struct EditInterviewProps {
    item: Interview,
    table: Signal<Vec<Interview>>,
    on_close: EventHandler<()>,
}

#[component]
pub fn EditInterview(mut props: EditInterviewProps) -> Element {
    let mut application_id = use_signal(|| props.item.application_id.clone());
    let mut interview_type = use_signal(|| props.item.interview_type.clone());
    let mut date = use_signal(|| props.item.interview_date.clone());
    let mut notes = use_signal(|| props.item.notes.clone());

    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Edit Interview" }

            form {
                div {
                    class: "form-group",
                    label { "Application ID" }
                    input {
                        r#type: "text",
                        value: "{application_id}",
                        oninput: move |e| application_id.set(e.value().parse().unwrap_or(0))
                    }
                }

                div {
                    class: "form-group",
                    label { "Interview Type" }
                    input {
                        r#type: "text",
                        value: "{interview_type}",
                        oninput: move |e| interview_type.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Interview Date" }
                    input {
                        r#type: "text",
                        value: "{date}",
                        oninput: move |e| date.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Notes" }
                    textarea {
                        name: "notes",
                        wrap: "soft",
                        value: "{notes}",
                        oninput: move |e| notes.set(e.value())
                    }
                }

                div {
                    class: "form-actions",

                    div {
                        class: "delete-btn",
                        button {
                            onclick: move |_| {
                                let db = AppDB::new();
                                db.delete_interview(props.item.id).unwrap();
                                let updated_table = db.get_interviews().unwrap();
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

                                // TODO: Check app_id and date are valid

                                let updated = NewInterview {
                                    application_id: application_id(),
                                    interview_type: interview_type(),
                                    interview_date: date(),
                                    notes: notes(),
                                };

                                db.edit_interview(updated, props.item.id).unwrap();
                                let updated_table = db.get_interviews().unwrap();
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
