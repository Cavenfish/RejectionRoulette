use backend::{database::AppDB, plots::stats_sankey};
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
    let ghost_alerts = db.get_ghost_alerts(8).unwrap();
    let upcoming = db.get_upcoming_interviews().unwrap();
    let offers = db.get_offers().unwrap();
    let stats = db.get_stats().unwrap();
    let sankey = stats_sankey(&stats).unwrap();

    rsx! {
        div {
            class: "dashboard-container",
            h2 { "Dashboard" }

            div {
                class: "dashboard-grid",

                div {
                    class: "card event-card",
                    h4 { "Upcoming Interviews" }

                    for i in upcoming.iter() {
                        div {
                            class: "card-row",
                            div {
                                class: "main-info",
                                span { class: "company-name", "{i.company}" }
                                span { class: "generic-info", "{i.interview_type}" }
                            }
                            span { class: "date-display", "{i.interview_date}" }
                        }
                    }
                }

                div {
                    class: "card alert-card",
                    h4 { "Ghost Alert" }
                    div {
                        class: "alert-item",

                        for alert in ghost_alerts.iter() {
                            div {
                                class: "card-row",
                                div {
                                    class: "main-info",
                                    span { class: "company-name", "{alert.company}"}
                                }
                                span { class: "date-display", "{alert.submit_date}" }
                            }
                        }
                    }
                }

                div {
                    class: "card offer-card",
                    h4 { "Open Offers" }

                    for off in offers.iter() {
                        div {
                            class: "card-row",
                            div {
                                class: "main-info",
                                span { class: "salary", "${off.base_salary / 1000}k" }
                                span { class: "company-name", "{off.company}" }
                            }
                            span { class: "date-display", "{off.expiration_date}" }
                        }
                    }
                }

                div {
                    class: "card summary-card",
                    h4 { "Quick Stats" }

                    for (key,value) in stats.iter() {
                        span {"{key}: {value}"}
                    }
                }

                div {
                    class: "card plot-card-lg",
                    h4 { "Sankey Plot" }

                    div {
                        class: "plot-placeholder",
                        title: "Sankey",
                        dangerous_inner_html: sankey
                    }

                }

            }
        }
    }
}
