use backend::database::{
    AppDB, Application, Interview, NewApplication, NewInterview, NewOffer, Offer,
};
use chrono::Local;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobData {
    pub resume: String,
    pub company: String,
    pub role: String,
    pub location: String,
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
    let db = AppDB::new();
    let resumes = db.get_resumes().unwrap();
    let date = Local::now().format("%Y/%m/%d").to_string();

    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Add new job application" }
            form {
                onsubmit: move |evt: FormEvent| async move {
                    let db = AppDB::new();
                    let data: JobData = evt.parsed_values().unwrap();

                    // TODO: Check date is valid

                    let new_app = NewApplication {
                        resume_id: data.resume.parse().ok(),
                        company: data.company,
                        role: data.role,
                        location: data.location,
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
                    label { "Location" }
                    input { r#type: "text", name: "location", placeholder: "e.g. Los Angeles, CA" }
                }

                div {
                    class: "form-group",
                    label { "Date" }
                    input { r#type: "text", name: "date", value: "{date}" }
                }

                div {
                    class: "form-group",
                    label { "Resume" }
                    select {
                        name: "resume",
                        option { value: None::<i64>, label: "--"}
                        for r in resumes {
                            option { value: r.id, label: "{r.name}" }
                        }
                    }
                }

                div {
                    class: "form-grop",
                    label { "Status" }
                    select {
                        name: "status",
                        value: "Pending",
                        option { "Pending" }
                        option { "Ghost" }
                        option { "Rejected" }
                        option { "Interview" }
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
    pub app_id: String,
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
                        application_id: data.app_id.parse().unwrap_or(1),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfferData {
    pub app_id: String,
    pub base_salary: String,
    pub bonus: String,
    pub equity_details: String,
    pub expiration_date: String,
    pub is_accepted: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct AddOfferFormProps {
    table: Signal<Vec<Offer>>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn AddOfferForm(mut props: AddOfferFormProps) -> Element {
    let today = Local::now().format("%Y/%m/%d").to_string();

    rsx! {
        div {
            class: "new-entry-form",
            h3 { "Add new offer" }

            form {
                onsubmit: move |evt: FormEvent| async move {
                    let db = AppDB::new();
                    let data: OfferData = evt.parsed_values().unwrap();

                    // TODO: Check app_id, salary, bonus, and date are valid

                    let new_offer = NewOffer {
                        application_id: data.app_id.parse().unwrap_or(1),
                        base_salary: data.base_salary.parse().unwrap_or(0),
                        bonus: data.bonus.parse().unwrap_or(0),
                        equity_details: data.equity_details,
                        expiration_date: data.expiration_date,
                        is_accepted: data.is_accepted == "true",
                    };

                    db.add_offer(new_offer).unwrap();

                    let updated_table = db.get_offers().unwrap();
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
                    label { "Base Salary" }
                    input {
                        r#type: "text",
                        name: "base_salary",
                        placeholder: "e.g. 100000",
                    }
                }

                div {
                    class: "form-group",
                    label { "Bonus" }
                    input {
                        r#type: "text",
                        name: "bonus",
                        placeholder: "e.g. 10000",
                    }
                }

                div {
                    class: "form-group",
                    label { "Equity Details" }
                    textarea {
                        name: "equity_details",
                        wrap: "soft",
                        placeholder: "e.g. 0.5% over 4 years",
                    }
                }

                div {
                    class: "form-group",
                    label { "Expiration Date" }
                    input {
                        r#type: "text",
                        name: "expiration_date",
                        placeholder: "e.g. {today}",
                    }
                }

                div {
                    class: "form-group",
                    label { "Offer Accepted" }
                    select {
                        name: "is_accepted",
                        value: "false",
                        option { "false" }
                        option { "true" }
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
