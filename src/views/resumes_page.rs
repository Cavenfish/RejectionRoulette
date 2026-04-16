use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
struct ResumeEntry {
    id: i64,
    name: String,
    hash: String,
}

#[component]
pub fn ResumesPage() -> Element {
    // In a real app, pull these from your AppDB
    let mut resumes = use_signal(|| {
        vec![
            ResumeEntry {
                id: 1,
                name: "Software_Engineer_2026.pdf".into(),
                hash: "a1b2c3d4".into(),
            },
            ResumeEntry {
                id: 2,
                name: "Product_Manager_v1.pdf".into(),
                hash: "e5f6g7h8".into(),
            },
        ]
    });

    let mut selected_resume: Signal<Option<ResumeEntry>> = use_signal(|| None);

    rsx! {
        div { class: "resume-page",

            // Left Side: Grid
            div { class: "resume-library",
                div { class: "library-header",
                    h2 { "Resumes" }
                    button {
                        class: "btn-submit",
                        onclick: move |_| { /* Trigger File Picker Modal Next */ },
                        "Add New"
                    }
                }

                div { class: "resume-grid",
                    for r in resumes.read().iter() {
                        {
                            let current_item = r.clone();

                            rsx! {
                                div {
                                    class: if selected_resume.read().as_ref() == Some(&current_item) { "resume-card active" } else { "resume-card" },
                                    onclick: move |_| selected_resume.set(Some(current_item.clone())),

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

            // Right Side: Preview
            div { class: "resume-preview",
                if let Some(resume) = selected_resume.read().as_ref() {
                    div { class: "preview-header",
                        h3 { "{resume.name}" }
                        button { class: "btn-delete", "Delete" }
                    }
                    // Assuming your storage dir is served or accessible via local path
                    iframe {
                        class: "pdf-frame",
                        src: "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf", // Swap with: format!("./storage/resumes/{}", resume.hash)
                    }
                } else {
                    div { class: "no-selection", "Select a resume to preview contents" }
                }
            }
        }
    }
}
