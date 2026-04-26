use backend::{database::AppDB, export::export_table};
use dioxus::prelude::*;

use crate::components::{AddApplicationForm, ApplicationsTable, ModalOverlay};

#[component]
pub fn ApplicationsPage() -> Element {
    let db = AppDB::new();

    let mut new_entry_flag = use_signal(|| false);
    let table = use_signal(|| db.get_applications().unwrap());

    rsx! {
        div {
            class: "table-header",
            h2 { "Applications" }

            div {
                class: "table-btns",
                div {
                    class: "new-item",
                    button {
                        onclick: move |_| {
                            let path = rfd::FileDialog::new()
                                .add_filter("CSV File", &["csv"])
                                .set_file_name("applications.csv")
                                .save_file();

                            if let Some(path) = path {
                                let table = db.get_applications().unwrap();

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
                inner: rsx!{AddApplicationForm {table, on_submit: move |_| new_entry_flag.set(false)}}
            }
        }
        ApplicationsTable { table }
    }
}
