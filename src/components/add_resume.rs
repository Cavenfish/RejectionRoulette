use std::path::PathBuf;

use backend::database::{AppDB, NewResume, Resume};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AddResumeProps {
    resumes: Signal<Vec<Resume>>,
    on_close: EventHandler<()>,
}

#[component]
pub fn AddResume(mut props: AddResumeProps) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut file_path = use_signal(|| PathBuf::new());
    let mut file_selected = use_signal(|| false);

    rsx! {
        div { class: "new-entry-form",
            h3 { "Upload Resume" }

            form {
                div {
                    class: "form-group",
                    label { "Resume Name" }
                    input {
                        r#type: "text",
                        placeholder: "e.g. SWE v0.1.0",
                        oninput: move |evt| name.set(evt.value())
                    }
                }

                div {
                    class: "form-group",
                    label { "File Selection" }

                    div {
                        class: "file-input-container",
                        label {
                            r#for: "resume-upload",
                            class: "file-label",
                            span { class: "icon", "📁" }
                            if file_selected() {
                                {
                                    let binding = file_path();
                                    let file_name = binding.file_name().unwrap_or_default().to_str().unwrap_or("Couldn't read file name");
                                    rsx! { span { "{file_name}" } }
                                }
                            } else {
                                span { "Select Resume PDF" }
                            }
                        }
                        input {
                            r#type: "file",
                            id: "resume-upload",
                            class: "hidden-input",
                            accept: ".pdf",
                            onchange: move |evt| {
                                let files = evt.files();

                                if let Some(file) = files.first() {
                                    let path = file.path();
                                    file_path.set(path);
                                    file_selected.set(true);
                                }
                            }
                        }
                    }
                }

                div {
                    class: "form-actions",
                    div {
                        class: "submit-btn",
                        button {
                            disabled: !file_selected(),
                            onclick: move |_| {
                                let db = AppDB::new();

                                let resume = NewResume {
                                    file_path: file_path(),
                                    name: name()
                                };

                                db.add_resume(resume).unwrap();

                                let updated = db.get_resumes().unwrap();
                                props.resumes.set(updated);

                                props.on_close.call(());
                            },
                            "Add to Library"
                        }
                    }
                }
            }
        }
    }
}
