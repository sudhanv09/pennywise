use dioxus::prelude::*;
use crate::Route;

#[derive(Clone, PartialEq, Props)]
pub struct RecentTx {
    pub id: i32,
    pub title: String,
    pub amount: f32,
    pub icon: String,
    pub is_income: bool,
    pub date_label: String,
}

#[component]
pub fn RecentTransactions(txs: Vec<RecentTx>) -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "home-section",
            style: "animation-delay: 260ms",
            div {
                class: "home-section-header",
                span { class: "home-section-title", "RECENT" }
                button {
                    class: "home-section-link",
                    onclick: move |_| { nav.push(Route::Transactions); },
                    "VIEW ALL"
                }
            }
            div {
                class: "recent-list",
                for tx in txs.iter() {
                    div {
                        class: "recent-row",
                        onclick: {
                            let id = tx.id;
                            move |_| { nav.push(Route::EditTransaction { id }); }
                        },
                        div {
                            class: "recent-icon",
                            if !tx.icon.is_empty() {
                                i { class: "icon-{tx.icon}" }
                            }
                        }
                        div {
                            class: "recent-info",
                            span { class: "recent-title", "{tx.title}" }
                            span { class: "recent-date", "{tx.date_label}" }
                        }
                        span {
                            class: if tx.is_income { "recent-amount recent-amount--pos" } else { "recent-amount" },
                            { if tx.is_income { format!("+${:.2}", tx.amount) }
                              else { format!("-${:.2}", tx.amount) } }
                        }
                    }
                }
            }
        }
    }
}
