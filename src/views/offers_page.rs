use backend::database::AppDB;
use dioxus::prelude::*;

use crate::components::{AddOfferForm, ModalOverlay, OffersTable};

#[component]
pub fn OffersPage() -> Element {
    let db = AppDB::new();

    let mut new_entry_flag = use_signal(|| false);
    let table = use_signal(|| db.get_offers().unwrap());

    rsx! {
        div {
            class: "table-header",
            h2 { "Offers" }

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
                inner: rsx!{AddOfferForm {table, on_submit: move |_| new_entry_flag.set(false)}}
            }
        }
        OffersTable { table }
    }
}
