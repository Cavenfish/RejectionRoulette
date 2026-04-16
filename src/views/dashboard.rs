use backend::{database::AppDB, plots::stats_sankey};
use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let db = AppDB::new();
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
                }

                div {
                    class: "card alert-card",
                    h4 { "Ghost Alert" }
                    div {
                        class: "alert-item",
                        span { class: "company", "Evil Corp" }
                        span { class: "days", "21 Days No Contact" }
                    }
                }

                div {
                    class: "card offer-card",
                    h4 { "Open Offers" }
                    div { class: "salary", "$140k" }
                    span { "Base + Equity (Stripe)" }
                    div { class: "salary", "$120k" }
                    span { "Base + Equity (PayPal)" }
                }

                div {
                    class: "card summary-card",
                    h4 { "Quick Stats" }
                    div { "Total Apps: 42" }
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
