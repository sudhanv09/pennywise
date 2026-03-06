use dioxus::prelude::*;
use chrono::Local;
use crate::db::DbConnection;
use crate::models::model::{Account, Category, TransactionType, Transactions};
use crate::repository::{accounts as acct_repo, categories as cat_repo, transactions as tx_repo};

#[component]
pub fn AddTransaction() -> Element {
    rsx! { TransactionForm { id: None } }
}

#[component]
pub fn EditTransaction(id: i32) -> Element {
    rsx! { TransactionForm { id: Some(id) } }
}

const TAGS: &[&str] = &["LOAN", "GOAL", "SAVINGS", "RECURRING"];

#[component]
fn TransactionForm(id: Option<i32>) -> Element {
    let db = use_context::<DbConnection>();

    let mut amount        = use_signal(|| String::from("0.00"));
    let mut tx_type       = use_signal(|| TransactionType::Expense);
    let mut selected_cat_id:  Signal<i32> = use_signal(|| 0);
    let mut selected_acct_id: Signal<i32> = use_signal(|| 0);
    let mut selected_tags: Signal<Vec<usize>> = use_signal(Vec::new);
    let mut note          = use_signal(String::new);
    let mut cat_drawer    = use_signal(|| false);

    let mut categories: Signal<Vec<Category>> = use_signal(Vec::new);
    let mut accounts:   Signal<Vec<Account>>  = use_signal(Vec::new);

    let is_edit = id.is_some();
    let nav     = use_navigator();

    {
        let db = db.clone();
        use_effect(move || {
            let loaded_cats  = cat_repo::get_all(&db).unwrap_or_default();
            let loaded_accts = acct_repo::get_all(&db).unwrap_or_default();

            // Set defaults from first items
            if !loaded_cats.is_empty() && *selected_cat_id.read() == 0 {
                selected_cat_id.set(loaded_cats[0].id);
            }
            if !loaded_accts.is_empty() && *selected_acct_id.read() == 0 {
                selected_acct_id.set(loaded_accts[0].id);
            }

            // Pre-populate for edit (overrides defaults)
            if let Some(edit_id) = id {
                if let Ok(tx) = tx_repo::get_by_id(&db, edit_id) {
                    amount.set(format!("{:.2}", tx.amount.abs()));
                    tx_type.set(tx.tx_type);
                    selected_cat_id.set(tx.category as i32);
                    selected_acct_id.set(tx.account as i32);
                    note.set(if tx.title.is_empty() { tx.description } else { tx.title });
                }
            }

            categories.set(loaded_cats);
            accounts.set(loaded_accts);
        });
    }

    let cat_icon = {
        let cats = categories.read();
        let id = *selected_cat_id.read();
        cats.iter().find(|c| c.id == id).map(|c| c.icon.clone()).unwrap_or_default()
    };
    let cat_label = {
        let cats = categories.read();
        let id = *selected_cat_id.read();
        cats.iter().find(|c| c.id == id).map(|c| c.name.to_uppercase()).unwrap_or_else(|| "SELECT".to_string())
    };

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
                    {accounts.read().iter().map(|acct| {
                        let acct_id   = acct.id;
                        let acct_name = acct.name.to_uppercase();
                        let selected  = *selected_acct_id.read() == acct_id;
                        rsx! {
                            button {
                                class: if selected { "chip chip--on" } else { "chip" },
                                onclick: move |_| selected_acct_id.set(acct_id),
                                "{acct_name}"
                            }
                        }
                    })}
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
                            let now      = Local::now();
                            let amt: f32 = amount.read().parse().unwrap_or(0.0);
                            let title    = note.read().clone();
                            let tx = Transactions {
                                id:          id.unwrap_or(0),
                                title:       title.clone(),
                                amount:      amt,
                                tx_date:     now.date_naive(),
                                tx_time:     now.time(),
                                tx_type:     tx_type.read().clone(),
                                category:    *selected_cat_id.read() as i16,
                                account:     *selected_acct_id.read() as i16,
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
                {categories.read().iter().map(|cat| {
                    let cat_id    = cat.id;
                    let cat_icon  = cat.icon.clone();
                    let cat_name  = cat.name.to_uppercase();
                    let selected  = *selected_cat_id.read() == cat_id;
                    rsx! {
                        button {
                            class: if selected { "cat-grid-card cat-grid-card--on" } else { "cat-grid-card" },
                            onclick: move |_| {
                                selected_cat_id.set(cat_id);
                                cat_drawer.set(false);
                            },
                            span { class: "cat-icon", "{cat_icon}" }
                            span { class: "cat-label", "{cat_name}" }
                        }
                    }
                })}
            }
        }
    }
}
