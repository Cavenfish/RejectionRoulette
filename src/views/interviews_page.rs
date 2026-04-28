use backend::{database::AppDB, export::export_table, filter::FilterCriteria};
use dioxus::prelude::*;

use crate::components::{AddInterviewForm, InterviewsTable, ModalOverlay};

#[component]
pub fn InterviewsPage() -> Element {
    let db = AppDB::new();

    let mut new_entry_flag = use_signal(|| false);
    let mut criteria = use_signal(|| FilterCriteria::default());
    let table = use_signal(|| db.get_interviews().unwrap());

    rsx! {
        div {
            class: "table-header",
            h2 { "Interviews" }

            input {
                r#type: "text",
                class: "filter-input",
                placeholder: "Filter table (e.g. company: Google role: 'Software Engineer')",
                oninput: move |evt| {
                    criteria.set(FilterCriteria::parse(&evt.value()));
                }
            }

            div {
                class: "table-btns",
                div {
                    class: "new-item",
                    button {
                        onclick: move |_| {
                            let path = rfd::FileDialog::new()
                                .add_filter("CSV File", &["csv"])
                                .set_file_name("interviews.csv")
                                .save_file();

                            if let Some(path) = path {
                                let table = db.get_interviews().unwrap();

                                export_table(table, &path).unwrap();
                            }
                        },
                        span { "Export to CSV" }
                    }
                }

                div {
                    class: "new-item",
                    button {
                        onclick: move |_| new_entry_flag.set(true),
                        "New Entry",
                    }
                }
            }
        }
        if new_entry_flag() {
            ModalOverlay {
                on_close: move |_| new_entry_flag.set(false),
                inner: rsx!{AddInterviewForm {table, on_submit: move |_| new_entry_flag.set(false)}}
            }
        }
        InterviewsTable { criteria, table }
    }
}
