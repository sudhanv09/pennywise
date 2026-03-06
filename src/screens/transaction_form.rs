use dioxus::prelude::*;
use crate::models::model::TransactionType;

#[component]
pub fn AddTransaction() -> Element {
    rsx! { TransactionForm { id: None } }
}

#[component]
pub fn EditTransaction(id: i32) -> Element {
    rsx! { TransactionForm { id: Some(id) } }
}

const ACCOUNTS: &[&str]           = &["AMEX *1002", "BANK ACCOUNT", "CASH"];
const CATEGORIES: &[(&str, &str)] = &[
    ("🍔", "FOOD"),  ("🚌", "TRANSPORT"), ("🛍", "SHOPPING"),
    ("💊", "HEALTH"), ("🎬", "ENTERTAIN"), ("🏠", "HOUSING"),
    ("✈️", "TRAVEL"), ("📚", "EDUCATION"),  ("🎁", "GIFTS"),
];
const TAGS: &[&str] = &["LOAN", "GOAL", "SAVINGS", "RECURRING"];

#[component]
fn TransactionForm(id: Option<i32>) -> Element {
    let mut amount        = use_signal(|| String::from("0.00"));
    let mut tx_type       = use_signal(|| TransactionType::Expense);
    let mut selected_acct = use_signal(|| 0usize);
    let mut selected_cat  = use_signal(|| 0usize);
    let mut selected_tags: Signal<Vec<usize>> = use_signal(Vec::new);
    let mut note          = use_signal(String::new);
    let mut cat_drawer    = use_signal(|| false);

    let is_edit = id.is_some();
    let nav     = use_navigator();

    let (cat_icon, cat_label) = CATEGORIES[*selected_cat.read()];

    rsx! {
        // ── Main form ──────────────────────────────────────
        div {
            class: "txform",

            // Header
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

            // Amount hero — category pill lives here
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

                // Meta row: USD info + category pill side by side
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

            // Account
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

            // Tags
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

            // Note
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

            // Submit
            div {
                class: "txform-footer",
                button {
                    class: "txform-submit",
                    onclick: move |_| { nav.go_back(); },
                    "Save"
                }
            }
        }

        // ── Category drawer ────────────────────────────────
        div {
            class: if *cat_drawer.read() { "drawer-backdrop visible" } else { "drawer-backdrop" },
            onclick: move |_| cat_drawer.set(false),
        }

        div {
            class: if *cat_drawer.read() { "cat-drawer open" } else { "cat-drawer" },

            // Handle
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
