use backend::database::AppDB;
use dioxus::prelude::*;

use crate::components::{ApplicationsTable, EntryForm, ModalOverlay};

#[component]
pub fn ApplicationsPage() -> Element {
    let db = AppDB::new();

    let mut new_entry_flag = use_signal(|| false);
    let mut table = use_signal(|| db.get_applications().unwrap());

    rsx! {
        div {
            class: "table-header",
            h2 { "Applications" }

            div {
                class: "new-item",
                button {
                    onclick: move |_| new_entry_flag.set(true),
                    "New Entry",
                }
            }
        }
        if new_entry_flag() {
            ModalOverlay {
                on_close: move |_| new_entry_flag.set(false),
                inner: rsx!{EntryForm {table, on_submit: move |_| new_entry_flag.set(false)}}
            }
        }
        ApplicationsTable { table }
    }
}
