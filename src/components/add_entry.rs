use backend::database::{AppDB, Application, Interview, NewApplication, NewInterview};
use chrono::Local;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobApplication {
    pub company: String,
    pub role: String,
    pub date: String,
    pub status: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct AddApplicationFormProps {
    table: Signal<Vec<Application>>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn AddApplicationForm(mut props: AddApplicationFormProps) -> Element {
    let date = Local::now().format("%Y/%m/%d").to_string();

    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Add new job application" }
            form {
                onsubmit: move |evt: FormEvent| async move {
                    let db = AppDB::new();
                    let data: JobApplication = evt.parsed_values().unwrap();

                    // TODO: Check date is valid

                    let new_app = NewApplication {
                        company: data.company,
                        role: data.role,
                        status: data.status,
                        submit_date: data.date
                    };

                    db.add_application(new_app).unwrap();

                    let updated_table = db.get_applications().unwrap();

                    props.table.set(updated_table);

                    props.on_submit.call(());
                },

                div {
                    class: "form-group",
                    label { "Company" }
                    input { r#type: "text", name: "company", placeholder: "e.g. Google" }
                }

                div {
                    class: "form-group",
                    label { "Role" }
                    input { r#type: "text", name: "role", placeholder: "e.g. Software Engineer, Backend" }
                }

                div {
                    class: "form-group",
                    label { "Date" }
                    input { r#type: "text", name: "date", value: "{date}" }
                }

                div {
                    class: "form-grop",
                    label { "Status" }
                    select {
                        name: "status",
                        value: "Pending",
                        option { "Ghost" }
                        option { "Rejected" }
                        option { "Interview" }
                        option { "Pending" }
                    }
                }

                div {
                    class: "form-actions",
                    div {
                        class: "submit-btn",
                        button { "Submit" }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterviewData {
    pub app_id: i64,
    pub interview_type: String,
    pub date: String,
    pub notes: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct AddInterviewFormProps {
    table: Signal<Vec<Interview>>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn AddInterviewForm(mut props: AddInterviewFormProps) -> Element {
    let today = Local::now().format("%Y/%m/%d").to_string();

    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Add new interview" }

            form {
                onsubmit: move |evt: FormEvent| async move {
                    let db = AppDB::new();
                    let data: InterviewData = evt.parsed_values().unwrap();

                    // TODO: Check app_id and date are valid

                    let new_interview = NewInterview {
                        application_id: data.app_id,
                        interview_type: data.interview_type,
                        interview_date: data.date,
                        notes: data.notes,
                    };

                    db.add_interview(new_interview).unwrap();

                    let updated_table = db.get_interviews().unwrap();

                    props.table.set(updated_table);
                    props.on_submit.call(());
                },

                div {
                    class: "form-group",
                    label { "Application ID" }
                    input {
                        r#type: "text",
                        name: "app_id",
                        placeholder: "e.g. 1",
                    }
                }

                div {
                    class: "form-group",
                    label { "Interview Type" }
                    input {
                        r#type: "text",
                        name: "interview_type",
                        placeholder: "e.g. Technical"
                    }
                }

                div {
                    class: "form-group",
                    label { "Interview Date" }
                    input {
                        r#type: "text",
                        name: "date",
                        placeholder: "e.g. {today}"
                    }
                }

                div {
                    class: "form-group",
                    label { "Notes" }
                    textarea {
                        name: "notes",
                        wrap: "soft",
                        placeholder: "What are your thoughts going into this interview? Update this later to save how you felt it went."
                    }
                }

                div {
                    class: "form-actions",
                    div {
                        class: "submit-btn",
                        button { "Submit" }
                    }
                }
            }
        }
    }
}
