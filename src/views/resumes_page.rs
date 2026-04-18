use backend::database::{AppDB, Resume};
use dioxus::prelude::*;
use open;

use crate::components::{AddResume, ModalOverlay};

#[component]
pub fn ResumesPage() -> Element {
    let db = AppDB::new();
    let mut new_add = use_signal(|| false);
    let resumes = use_signal(|| db.get_resumes().unwrap());

    let mut selected_resume: Signal<Option<Resume>> = use_signal(|| None);

    rsx! {
        div {
            class: "resume-page",

            div {
                class: "resume-library",

                div {
                    class: "library-header",
                    h2 { "Resumes" }

                    div {
                        class: "new-item",
                        button {
                            class: "btn-submit",
                            onclick: move |_| new_add.set(true),
                            "Add New"
                        }
                    }
                }

                div { class: "resume-grid",
                    for r in resumes.read().iter() {
                        {
                            let current_item = r.clone();
                            let file_path = db.get_file(&r).unwrap();

                            rsx! {
                                div {
                                    class: if selected_resume.read().as_ref() == Some(&current_item) { "resume-card active" } else { "resume-card" },
                                    onclick: move |_| {
                                        open::that(&file_path).unwrap();
                                        selected_resume.set(Some(current_item.clone()))
                                    },

                                    // Resume SVG (Document icon)
                                    svg { view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                                        path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                        polyline { points: "14 2 14 8 20 8" }
                                    }
                                    span { "{r.name}" }
                                }
                            }
                        }
                    }
                }
            }

            if new_add() {
                ModalOverlay {
                    on_close: move |_| new_add.set(false),
                    inner: rsx!{ AddResume {resumes, on_close: move |_| new_add.set(false)} }
                }
            }
        }
    }
}
