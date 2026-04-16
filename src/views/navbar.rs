use crate::Route;
use dioxus::prelude::*;

const DASHBOARD: &str = include_str!("../../assets/icons/dashboard.svg");
const APPLICATIONS: &str = include_str!("../../assets/icons/applications.svg");
const INTERVIEWS: &str = include_str!("../../assets/icons/interviews.svg");
const OFFERS: &str = include_str!("../../assets/icons/offers.svg");
const CV: &str = include_str!("../../assets/icons/cv.svg");
const SETTINGS: &str = include_str!("../../assets/icons/settings.svg");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "app-container",

            div {
                class: "sidebar",

                div {
                    class: "nav-top",

                    Link {
                        to: Route::Dashboard {},
                        div {
                            class: "nav-item",
                            title: "Dashboard",
                            dangerous_inner_html: DASHBOARD
                        }
                    }

                    Link {
                        to: Route::ApplicationsPage {},
                        div {
                            class: "nav-item",
                            title: "Applications",
                            dangerous_inner_html: APPLICATIONS
                        }
                    }

                    Link {
                        to: Route::InterviewsPage {},
                        div {
                            class: "nav-item",
                            title: "Interviews",
                            dangerous_inner_html: INTERVIEWS
                        }
                    }

                    Link {
                        to: Route::OffersPage {},
                        div {
                            class: "nav-item",
                            title: "Offers",
                            dangerous_inner_html: OFFERS
                        }
                    }

                    Link {
                        to: Route::ResumesPage {},
                        div {
                            class: "nav-item",
                            title: "Resumes",
                            dangerous_inner_html: CV
                        }
                    }
                }

                div {
                    class: "nav-bottom",

                    Link {
                        to: Route::SettingsPage {},
                        div {
                            class: "nav-item",
                            title: "Settings",
                            dangerous_inner_html: SETTINGS
                        }
                    }
                }
            }

            div {
                class: "main-content",
                Outlet::<Route> {}
            }
        }
    }
}
