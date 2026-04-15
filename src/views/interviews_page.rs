use backend::database::AppDB;
use dioxus::prelude::*;

use crate::components::{AddInterviewForm, InterviewsTable, ModalOverlay};

#[component]
pub fn InterviewsPage() -> Element {
    let db = AppDB::new();

    let mut new_entry_flag = use_signal(|| false);
    let table = use_signal(|| db.get_interviews().unwrap());

    rsx! {
        div {
            class: "table-header",
            h2 { "Interviews" }

            div {
                class: "new-item",
                button {
                    onclick: move |_| new_entry_flag.set(true),
                    "New Entry"
                }
            }
        }
        if new_entry_flag() {
            ModalOverlay {
                on_close: move |_| new_entry_flag.set(false),
                inner: rsx!{AddInterviewForm {table, on_submit: move |_| new_entry_flag.set(false)}}
            }
        }
        InterviewsTable { table }
    }
}
