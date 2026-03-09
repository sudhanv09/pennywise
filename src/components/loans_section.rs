use dioxus::prelude::*;
use crate::Route;

#[derive(Clone, PartialEq, Props)]
pub struct LoanRow {
    pub name: String,
    pub total: f32,
    pub paid: f32,
    pub pct: f32,
    pub is_lender: bool,
}

#[component]
pub fn LoansSection(loans: Vec<LoanRow>) -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "home-section",
            style: "animation-delay: 380ms",
            div {
                class: "home-section-header",
                span { class: "home-section-title", "LOANS" }
                button {
                    class: "home-section-link",
                    onclick: move |_| { nav.push(Route::Loans); },
                    "MANAGE"
                }
            }
            div {
                class: "loans-list",
                for l in loans.iter() {
                    div {
                        class: "loan-mini",
                        div {
                            class: "loan-mini-top",
                            div {
                                class: "loan-mini-left",
                                span { class: "loan-mini-name", "{l.name}" }
                                span {
                                    class: if l.is_lender { "loan-mini-tag loan-mini-tag--lent" }
                                           else { "loan-mini-tag" },
                                    if l.is_lender { "LENT" } else { "OWED" }
                                }
                            }
                            span {
                                class: "loan-mini-remaining",
                                { format!("${:.0}", (l.total - l.paid).max(0.0)) }
                            }
                        }
                        div {
                            class: "loan-mini-bar",
                            div {
                                class: if l.pct >= 100.0 { "loan-mini-fill loan-mini-fill--done" }
                                       else { "loan-mini-fill" },
                                style: "width: {l.pct}%",
                            }
                        }
                    }
                }
            }
        }
    }
}
