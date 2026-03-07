use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use crate::db::DbConnection;
use crate::models::model::Loans as LoanModel;
use crate::repository::loans as repo;

fn days_to_date_str(days: f32) -> String {
    NaiveDate::from_num_days_from_ce_opt(days as i32)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn date_str_to_days(s: &str) -> f32 {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map(|d| d.num_days_from_ce() as f32)
        .unwrap_or(0.0)
}

#[component]
pub fn Loans() -> Element {
    let db  = use_context::<DbConnection>();
    let nav = use_navigator();

    let mut items:      Signal<Vec<LoanModel>> = use_signal(Vec::new);
    let mut editing_id: Signal<i32>        = use_signal(|| -1i32);
    let mut drawer_open                    = use_signal(|| false);

    let mut f_name   = use_signal(String::new);
    let mut f_total  = use_signal(String::new);
    let mut f_paid   = use_signal(String::new);
    let mut f_due    = use_signal(String::new);
    let mut f_lender = use_signal(|| false);

    {
        let db = db.clone();
        use_effect(move || {
            items.set(repo::get_all(&db).unwrap_or_default());
        });
    }

    let is_new = *editing_id.read() == -1;

    let open_new = move |_| {
        f_name.set(String::new());
        f_total.set(String::new());
        f_paid.set(String::new());
        f_due.set(String::new());
        f_lender.set(false);
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
                span { class: "txform-title", "LOANS" }
                button {
                    class: "subscreen-add-btn",
                    onclick: open_new,
                    "+"
                }
            }

            div {
                class: "subscreen-list",
                {items.read().iter().map(|l| {
                    let id   = l.id;
                    let name = l.name.clone();
                    let kind = if l.is_lender { "LENDER" } else { "BORROWER" };
                    let rem  = l.total_amount - l.paid_amount;
                    let sub  = format!("{} • ${:.0} remaining", kind, rem);
                    let l    = l.clone();
                    rsx! {
                        button {
                            key: "{id}",
                            class: "settings-row",
                            onclick: move |_| {
                                f_name.set(l.name.clone());
                                f_total.set(format!("{:.2}", l.total_amount));
                                f_paid.set(format!("{:.2}", l.paid_amount));
                                f_due.set(days_to_date_str(l.due));
                                f_lender.set(l.is_lender);
                                editing_id.set(id);
                                drawer_open.set(true);
                            },
                            div { class: "settings-row-main",
                                span { class: "settings-row-label", "{name}" }
                                span { class: "settings-row-sub", "{sub}" }
                            }
                            span { class: "settings-row-chevron", "›" }
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
            p { class: "drawer-title", { if is_new { "NEW LOAN" } else { "EDIT LOAN" } } }
            div { class: "drawer-fields",
                div { class: "drawer-field",
                    label { "NAME" }
                    input {
                        class: "drawer-input", r#type: "text", placeholder: "Person or lender name",
                        value: "{f_name}",
                        oninput: move |e| f_name.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "TOTAL AMOUNT" }
                    input {
                        class: "drawer-input", r#type: "number", step: "0.01", placeholder: "0.00",
                        value: "{f_total}",
                        oninput: move |e| f_total.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "PAID AMOUNT" }
                    input {
                        class: "drawer-input", r#type: "number", step: "0.01", placeholder: "0.00",
                        value: "{f_paid}",
                        oninput: move |e| f_paid.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "DUE DATE" }
                    input {
                        class: "drawer-input", r#type: "date",
                        value: "{f_due}",
                        oninput: move |e| f_due.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "ROLE" }
                    div { class: "drawer-toggle-row",
                        button {
                            class: if !*f_lender.read() { "chip chip--on" } else { "chip" },
                            onclick: move |_| f_lender.set(false),
                            "BORROWER"
                        }
                        button {
                            class: if *f_lender.read() { "chip chip--on" } else { "chip" },
                            onclick: move |_| f_lender.set(true),
                            "LENDER"
                        }
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
                            let item = LoanModel {
                                id,
                                name:         f_name.read().clone(),
                                total_amount: f_total.read().parse().unwrap_or(0.0),
                                paid_amount:  f_paid.read().parse().unwrap_or(0.0),
                                due:          date_str_to_days(&f_due.read()),
                                is_lender:    *f_lender.read(),
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
