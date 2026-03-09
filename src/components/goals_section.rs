use dioxus::prelude::*;
use crate::Route;

#[derive(Clone, PartialEq, Props)]
pub struct GoalRow {
    pub name: String,
    pub target: f32,
    pub current: f32,
    pub pct: f32,
}

#[component]
pub fn GoalsSection(goals: Vec<GoalRow>) -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "home-section",
            style: "animation-delay: 320ms",
            div {
                class: "home-section-header",
                span { class: "home-section-title", "GOALS" }
                button {
                    class: "home-section-link",
                    onclick: move |_| { nav.push(Route::Goals); },
                    "MANAGE"
                }
            }
            div {
                class: "goals-list",
                for g in goals.iter() {
                    div {
                        class: "goal-mini",
                        div {
                            class: "goal-mini-top",
                            span { class: "goal-mini-name", "{g.name}" }
                            span {
                                class: "goal-mini-pct",
                                { format!("{:.0}%", g.pct) }
                            }
                        }
                        div {
                            class: "goal-mini-bar",
                            div {
                                class: if g.pct >= 100.0 { "goal-mini-fill goal-mini-fill--done" }
                                       else { "goal-mini-fill" },
                                style: "width: {g.pct}%",
                            }
                        }
                        div {
                            class: "goal-mini-amounts",
                            span { { format!("${:.0}", g.current) } }
                            span { class: "goal-mini-target", { format!("/ ${:.0}", g.target) } }
                        }
                    }
                }
            }
        }
    }
}
