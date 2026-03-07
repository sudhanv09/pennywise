use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use pennywise::db::DbConnection;
use pennywise::models::model::Goals as GoalModel;
use pennywise::repository::{goals as repo, transactions as tx_repo};

fn days_to_date_str(days: f32) -> String {
    NaiveDate::from_num_days_from_ce_opt(days as i32)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn date_str_to_days(s: &str) -> Option<f32> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .map(|d| d.num_days_from_ce() as f32)
}

#[component]
pub fn Goals() -> Element {
    let db  = use_context::<DbConnection>();
    let nav = use_navigator();

    let mut items:      Signal<Vec<GoalModel>> = use_signal(Vec::new);
    let mut editing_id: Signal<i32>        = use_signal(|| -1i32);
    let mut drawer_open                    = use_signal(|| false);

    let mut f_name     = use_signal(String::new);
    let mut f_target   = use_signal(String::new);
    let mut f_deadline = use_signal(String::new);

    {
        let db = db.clone();
        use_effect(move || {
            items.set(repo::get_all(&db).unwrap_or_default());
        });
    }

    let is_new = *editing_id.read() == -1;

    let open_new = move |_| {
        f_name.set(String::new());
        f_target.set(String::new());
        f_deadline.set(String::new());
        editing_id.set(-1);
        drawer_open.set(true);
    };

    rsx! {
        div {
            class: "subscreen",

            div {
                class: "subscreen-header",
                button {
                    class: "txform-close",
                    onclick: move |_| nav.go_back(),
                    "×"
                }
                span { class: "txform-title", "GOALS" }
                button {
                    class: "subscreen-add-btn",
                    onclick: open_new,
                    "+"
                }
            }

            div {
                class: "subscreen-list",
                {items.read().iter().map(|g| {
                    let id   = g.id;
                    let name = g.name.clone();
                    let current = tx_repo::sum_for_goal(&db, id).unwrap_or(0.0);
                    let pct  = if g.target > 0.0 { (current / g.target * 100.0).min(100.0) } else { 0.0 };
                    let sub  = format!("${:.0} / ${:.0}", current, g.target);
                    let pct_str   = format!("{:.0}%", pct);
                    let pct_width = format!("{}%", pct as u32);
                    let bar_class = if pct >= 100.0 { "loan-bar-fill loan-bar-fill--done" } else { "loan-bar-fill" };
                    let status = if pct >= 100.0 { "REACHED" } else { "" };
                    let status_class = if pct >= 100.0 { "loan-status loan-status--done" } else { "loan-status" };
                    let g = g.clone();
                    rsx! {
                        button {
                            key: "{id}",
                            class: "loan-card",
                            onclick: move |_| {
                                f_name.set(g.name.clone());
                                f_target.set(format!("{:.2}", g.target));
                                f_deadline.set(g.deadline.map(|d| days_to_date_str(d)).unwrap_or_default());
                                editing_id.set(id);
                                drawer_open.set(true);
                            },
                            div { class: "loan-card-top",
                                div { class: "settings-row-main",
                                    span { class: "settings-row-label", "{name}" }
                                    span { class: "settings-row-sub", "{sub}" }
                                }
                                span { class: "settings-row-chevron", "›" }
                            }
                            div { class: "loan-bar-row",
                                div { class: "loan-bar",
                                    div { class: "{bar_class}", style: "width: {pct_width}" }
                                }
                                span { class: "loan-pct", "{pct_str}" }
                            }
                            if !status.is_empty() {
                                div { class: "loan-insights",
                                    span { class: "{status_class}", "{status}" }
                                }
                            }
                        }
                    }
                })}
            }
        }

        div {
            class: if *drawer_open.read() { "drawer-backdrop visible" } else { "drawer-backdrop" },
            onclick: move |_| drawer_open.set(false),
        }

        div {
            class: if *drawer_open.read() { "settings-drawer open" } else { "settings-drawer" },
            div { class: "drawer-handle" }
            p { class: "drawer-title", { if is_new { "NEW GOAL" } else { "EDIT GOAL" } } }
            div { class: "drawer-fields",
                div { class: "drawer-field",
                    label { "NAME" }
                    input {
                        class: "drawer-input", r#type: "text", placeholder: "Goal name",
                        value: "{f_name}",
                        oninput: move |e| f_name.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "TARGET" }
                    input {
                        class: "drawer-input", r#type: "number", step: "0.01", placeholder: "0.00",
                        value: "{f_target}",
                        oninput: move |e| f_target.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "DEADLINE (optional)" }
                    input {
                        class: "drawer-input", r#type: "date",
                        value: "{f_deadline}",
                        oninput: move |e| f_deadline.set(e.value()),
                    }
                }
            }
            div { class: "drawer-actions",
                if !is_new {
                    button {
                        class: "drawer-btn drawer-btn--delete",
                        onclick: {
                            let db = db.clone();
                            move |_| {
                                let _ = repo::delete(&db, *editing_id.read());
                                items.set(repo::get_all(&db).unwrap_or_default());
                                drawer_open.set(false);
                            }
                        },
                        "DELETE"
                    }
                }
                button {
                    class: "drawer-btn drawer-btn--save",
                    onclick: {
                        let db = db.clone();
                        move |_| {
                            let id = *editing_id.read();
                            let item = GoalModel {
                                id,
                                name:     f_name.read().clone(),
                                target:   f_target.read().parse().unwrap_or(0.0),
                                current:  0.0,
                                deadline: date_str_to_days(&f_deadline.read()),
                            };
                            if id == -1 { let _ = repo::insert(&db, &item); }
                            else        { let _ = repo::update(&db, &item); }
                            items.set(repo::get_all(&db).unwrap_or_default());
                            drawer_open.set(false);
                        }
                    },
                    "SAVE"
                }
            }
        }
    }
}
