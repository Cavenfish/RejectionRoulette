use backend::database::{
    AppDB, Application, Interview, NewApplication, NewInterview, NewOffer, Offer,
};
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
    let mut resume = use_signal(|| props.item.resume_id.clone());
    let mut company = use_signal(|| props.item.company.clone());
    let mut role = use_signal(|| props.item.role.clone());
    let mut location = use_signal(|| props.item.location.clone());
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
                    label { "Location" }
                    input {
                        r#type: "text",
                        value: "{location}",
                        oninput: move |e| location.set(e.value())
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
                                    resume_id: resume(),
                                    company: company(),
                                    role: role(),
                                    location: location(),
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

#[derive(Props, Clone, PartialEq)]
pub struct EditOfferProps {
    item: Offer,
    table: Signal<Vec<Offer>>,
    on_close: EventHandler<()>,
}

#[component]
pub fn EditOffer(mut props: EditOfferProps) -> Element {
    let mut application_id = use_signal(|| props.item.application_id.clone());
    let mut base_salary = use_signal(|| props.item.base_salary.clone());
    let mut bonus = use_signal(|| props.item.bonus.clone());
    let mut equity_details = use_signal(|| props.item.equity_details.clone());
    let mut expiration_date = use_signal(|| props.item.expiration_date.clone());
    let mut is_accepted = use_signal(|| props.item.is_accepted.clone());

    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Edit Offer" }

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
                    label { "Base Salary" }
                    input {
                        r#type: "text",
                        value: "{base_salary}",
                        oninput: move |e| base_salary.set(e.value().parse().unwrap_or(0))
                    }
                }

                div {
                    class: "form-group",
                    label { "Bonus" }
                    input {
                        r#type: "text",
                        value: "{bonus}",
                        oninput: move |e| bonus.set(e.value().parse().unwrap_or(0))
                    }
                }

                div {
                    class: "form-group",
                    label { "Equity Details" }
                    textarea {
                        name: "equity_details",
                        wrap: "soft",
                        value: "{equity_details}",
                        oninput: move |e| equity_details.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Expiration Date" }
                    input {
                        r#type: "text",
                        value: "{expiration_date}",
                        oninput: move |e| expiration_date.set(e.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "Is Accepted" }
                    input {
                        r#type: "checkbox",
                        checked: "{is_accepted}",
                        onchange: move |e| is_accepted.set(e.checked())
                    }
                }

                div {
                    class: "form-actions",

                    div {
                        class: "delete-btn",
                        button {
                            onclick: move |_| {
                                let db = AppDB::new();
                                db.delete_offer(props.item.id).unwrap();
                                let updated_table = db.get_offers().unwrap();
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

                                // TODO: Check application_id and dates are valid

                                let updated = NewOffer {
                                    application_id: application_id(),
                                    base_salary: base_salary(),
                                    bonus: bonus(),
                                    equity_details: equity_details(),
                                    expiration_date: expiration_date(),
                                    is_accepted: is_accepted(),
                                };

                                db.edit_offer(updated, props.item.id).unwrap();
                                let updated_table = db.get_offers().unwrap();
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
