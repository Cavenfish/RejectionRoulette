use backend::{
    database::AppDB,
    plots::{activity_calendar, resume_pie_chart, stats_sankey},
    settings::AppSettings,
};
use dioxus::{document::eval, prelude::*};

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
    let settings = AppSettings::load();
    let ghost_alerts = db.get_ghost_alerts(settings.ghost_time).unwrap();
    let upcoming = db.get_upcoming_interviews().unwrap();
    let offers = db.get_offers().unwrap();
    let stats = db.get_stats().unwrap();

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

                if stats.sankey.total() > 0 {
                    {
                        let sankey = stats_sankey(&stats.sankey).unwrap();
                        let date_range = db.get_date_range().unwrap();
                        let heatmap = activity_calendar(date_range, stats.dates).unwrap();
                        let resume_pie = resume_pie_chart(stats.resumes).unwrap();

                        rsx! {
                            div {
                                class: "card plot-card-sm",
                                h4 { "Interviews by Resume" }

                                div {
                                    class: "chart-container",
                                    div {
                                        id: "pie",
                                        class: "pie",
                                        title: "Resume Pie",
                                        onmounted: move |_| {
                                            eval(&resume_pie);
                                        }
                                    }
                                }
                            }

                            div {
                                class: "card plot-card-lg",
                                h4 { "Sankey Plot" }

                                div {
                                    class: "chart-container",
                                    div {
                                        id: "sankey",
                                        class: "sankey",
                                        title: "Sankey",
                                        onmounted: move |_| {
                                            eval(&sankey);
                                        }
                                    }
                                }


                            }

                            div {
                                class: "card plot-card-lg",
                                h4 { "Application Heatmap" }

                                div {
                                    class: "chart-container",
                                    div {
                                        id: "calendar",
                                        class: "yearly-calendar",
                                        title: "Calendar Heatmap",
                                        onmounted: move |_| {
                                            eval(&heatmap);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
