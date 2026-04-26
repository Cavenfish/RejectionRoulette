use backend::{database::AppDB, export::export_table};
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
                class: "table-btns",
                div {
                    class: "new-item",
                    button {
                        onclick: move |_| {
                            let path = rfd::FileDialog::new()
                                .add_filter("CSV File", &["csv"])
                                .set_file_name("offers.csv")
                                .save_file();

                            if let Some(path) = path {
                                let table = db.get_offers().unwrap();

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
                inner: rsx!{AddOfferForm {table, on_submit: move |_| new_entry_flag.set(false)}}
            }
        }
        OffersTable { table }
    }
}
