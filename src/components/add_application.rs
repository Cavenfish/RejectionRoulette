use backend::database::{AppDB, Application, NewApplication};
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
pub struct EntryFormProps {
    table: Signal<Vec<Application>>,
    on_submit: EventHandler<()>,
}

#[component]
pub fn EntryForm(mut props: EntryFormProps) -> Element {
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
                        option { "Ghost" }
                        option { "Rejected" }
                        option { "Interview" }
                        option { "Pending" }
                    }
                }

                div {
                    class: "form-actions",
                    button { "Submit" }
                }
            }
        }
    }
}
