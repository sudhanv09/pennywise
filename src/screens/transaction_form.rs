use dioxus::prelude::*;
use chrono::Local;
use crate::db::DbConnection;
use crate::models::model::{TransactionType, Transactions};
use crate::repository::transactions as tx_repo;

#[component]
pub fn AddTransaction() -> Element {
    rsx! { TransactionForm { id: None } }
}

#[component]
pub fn EditTransaction(id: i32) -> Element {
    rsx! { TransactionForm { id: Some(id) } }
}

pub const CATEGORIES: &[(&str, &str)] = &[
    ("🍔", "FOOD"),  ("🚌", "TRANSPORT"), ("🛍", "SHOPPING"),
    ("💊", "HEALTH"), ("🎬", "ENTERTAIN"), ("🏠", "HOUSING"),
    ("✈️", "TRAVEL"), ("📚", "EDUCATION"),  ("🎁", "GIFTS"),
];
const TAGS: &[&str] = &["LOAN", "GOAL", "SAVINGS", "RECURRING"];

// Placeholder accounts until accounts screen is built
const ACCOUNTS: &[&str] = &["ACCOUNT 1", "ACCOUNT 2", "CASH"];

#[component]
fn TransactionForm(id: Option<i32>) -> Element {
    let db = use_context::<DbConnection>();

    let mut amount        = use_signal(|| String::from("0.00"));
    let mut tx_type       = use_signal(|| TransactionType::Expense);
    let mut selected_acct = use_signal(|| 0usize);
    let mut selected_cat  = use_signal(|| 0usize);
    let mut selected_tags: Signal<Vec<usize>> = use_signal(Vec::new);
    let mut note          = use_signal(String::new);
    let mut cat_drawer    = use_signal(|| false);

    let is_edit = id.is_some();
    let nav     = use_navigator();

    // Pre-populate form when editing
    {
        let db = db.clone();
        use_effect(move || {
            if let Some(edit_id) = id {
                if let Ok(tx) = tx_repo::get_by_id(&db, edit_id) {
                    amount.set(format!("{:.2}", tx.amount.abs()));
                    tx_type.set(tx.tx_type);
                    selected_cat.set(tx.category as usize);
                    selected_acct.set(tx.account as usize);
                    note.set(if tx.title.is_empty() { tx.description } else { tx.title });
                }
            }
        });
    }

    let (cat_icon, cat_label) = CATEGORIES[*selected_cat.read()];

    rsx! {
        div {
            class: "txform",

            div {
                class: "txform-header",
                button {
                    class: "txform-close",
                    onclick: move |_| { nav.go_back(); },
                    "×"
                }
                span { class: "txform-title",
                    { if is_edit { "EDIT ENTRY" } else { "NEW ENTRY" } }
                }
                button { class: "txform-date-pill", "📅  TODAY  ▾" }
            }

            div {
                class: "txform-hero",

                div {
                    class: "txform-type-row",
                    button {
                        class: if *tx_type.read() == TransactionType::Income   { "type-chip type-chip--income active" } else { "type-chip type-chip--income" },
                        onclick: move |_| tx_type.set(TransactionType::Income),
                        "INCOME"
                    }
                    button {
                        class: if *tx_type.read() == TransactionType::Expense  { "type-chip type-chip--expense active" } else { "type-chip type-chip--expense" },
                        onclick: move |_| tx_type.set(TransactionType::Expense),
                        "EXPENSE"
                    }
                    button {
                        class: if *tx_type.read() == TransactionType::Transfer { "type-chip type-chip--transfer active" } else { "type-chip type-chip--transfer" },
                        onclick: move |_| tx_type.set(TransactionType::Transfer),
                        "TRANSFER"
                    }
                }

                div {
                    class: "txform-amount-wrap",
                    span { class: "txform-currency", "$" }
                    input {
                        class: "txform-amount-input",
                        r#type: "number",
                        min: "0",
                        step: "0.01",
                        placeholder: "0.00",
                        value: "{amount}",
                        oninput: move |e| amount.set(e.value()),
                    }
                }

                div {
                    class: "txform-meta-row",
                    span { class: "txform-meta", "USD  •  PERSONAL" }
                    button {
                        class: "cat-pill",
                        onclick: move |_| cat_drawer.set(true),
                        span { "{cat_icon}" }
                        span { "{cat_label}" }
                        span { class: "cat-pill-arrow", "›" }
                    }
                }
            }

            div {
                class: "txform-section",
                p { class: "txform-section-label", "ACCOUNT" }
                div {
                    class: "chip-row",
                    for (i, name) in ACCOUNTS.iter().enumerate() {
                        button {
                            class: if *selected_acct.read() == i { "chip chip--on" } else { "chip" },
                            onclick: move |_| selected_acct.set(i),
                            "{name}"
                        }
                    }
                }
            }

            div {
                class: "txform-section",
                p { class: "txform-section-label", "TAGS" }
                div {
                    class: "chip-row",
                    for (i, tag) in TAGS.iter().enumerate() {
                        button {
                            class: if selected_tags.read().contains(&i) { "chip chip--on" } else { "chip" },
                            onclick: move |_| {
                                let mut t = selected_tags.write();
                                if t.contains(&i) { t.retain(|&x| x != i); }
                                else { t.push(i); }
                            },
                            "{tag}"
                        }
                    }
                }
            }

            div {
                class: "txform-section",
                input {
                    class: "txform-note",
                    r#type: "text",
                    placeholder: "Add a note…",
                    value: "{note}",
                    oninput: move |e| note.set(e.value()),
                }
            }

            div {
                class: "txform-footer",
                button {
                    class: "txform-submit",
                    onclick: {
                        let db = db.clone();
                        move |_| {
                            let now        = Local::now();
                            let amt: f32   = amount.read().parse().unwrap_or(0.0);
                            let title      = note.read().clone();
                            let tx = Transactions {
                                id:          id.unwrap_or(0),
                                title:       title.clone(),
                                amount:      amt,
                                tx_date:     now.date_naive(),
                                tx_time:     now.time(),
                                tx_type:     tx_type.read().clone(),
                                category:    *selected_cat.read() as i16,
                                account:     *selected_acct.read() as i16,
                                description: title,
                            };
                            if is_edit {
                                let _ = tx_repo::update(&db, &tx);
                            } else {
                                let _ = tx_repo::insert(&db, &tx);
                            }
                            nav.go_back();
                        }
                    },
                    "Save"
                }
            }
        }

        div {
            class: if *cat_drawer.read() { "drawer-backdrop visible" } else { "drawer-backdrop" },
            onclick: move |_| cat_drawer.set(false),
        }

        div {
            class: if *cat_drawer.read() { "cat-drawer open" } else { "cat-drawer" },
            div { class: "drawer-handle" }
            p { class: "drawer-title", "CATEGORY" }
            div {
                class: "cat-grid",
                for (i, (icon, label)) in CATEGORIES.iter().enumerate() {
                    button {
                        class: if *selected_cat.read() == i { "cat-grid-card cat-grid-card--on" } else { "cat-grid-card" },
                        onclick: move |_| {
                            selected_cat.set(i);
                            cat_drawer.set(false);
                        },
                        span { class: "cat-icon", "{icon}" }
                        span { class: "cat-label", "{label}" }
                    }
                }
            }
        }
    }
}
