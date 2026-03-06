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

#[component]
fn TransactionForm(id: Option<i32>) -> Element {
    let mut title = use_signal(|| String::new());
    let mut amount = use_signal(|| String::new());
    let mut tx_date = use_signal(|| String::new());
    let mut tx_time = use_signal(|| String::new());
    let mut tx_type = use_signal(|| TransactionType::Expense);
    let mut description = use_signal(|| String::new());

    let is_edit = id.is_some();
    let nav = use_navigator();

    rsx! {
        div {
            class: "transaction-form",

            div {
                class: "form-header",
                button {
                    onclick: move |_| { nav.go_back(); },
                    "← Back"
                }
                h1 { { if is_edit { "Edit Transaction" } else { "Add Transaction" } } }
            }

            // Transaction type toggle
            div {
                class: "tx-type-selector",
                button {
                    class: if *tx_type.read() == TransactionType::Income { "active" } else { "" },
                    onclick: move |_| tx_type.set(TransactionType::Income),
                    "Income"
                }
                button {
                    class: if *tx_type.read() == TransactionType::Expense { "active" } else { "" },
                    onclick: move |_| tx_type.set(TransactionType::Expense),
                    "Expense"
                }
                button {
                    class: if *tx_type.read() == TransactionType::Transfer { "active" } else { "" },
                    onclick: move |_| tx_type.set(TransactionType::Transfer),
                    "Transfer"
                }
            }

            div {
                class: "form-field",
                label { "Title" }
                input {
                    r#type: "text",
                    placeholder: "e.g. Groceries",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                }
            }

            div {
                class: "form-field",
                label { "Amount" }
                input {
                    r#type: "number",
                    placeholder: "0.00",
                    min: "0",
                    step: "0.01",
                    value: "{amount}",
                    oninput: move |e| amount.set(e.value()),
                }
            }

            div {
                class: "form-field",
                label { "Date" }
                input {
                    r#type: "date",
                    value: "{tx_date}",
                    oninput: move |e| tx_date.set(e.value()),
                }
            }

            div {
                class: "form-field",
                label { "Time" }
                input {
                    r#type: "time",
                    value: "{tx_time}",
                    oninput: move |e| tx_time.set(e.value()),
                }
            }

            div {
                class: "form-field",
                label { "Description" }
                textarea {
                    placeholder: "Optional note",
                    value: "{description}",
                    oninput: move |e| description.set(e.value()),
                }
            }

            div {
                class: "form-actions",
                button {
                    class: "btn-cancel",
                    onclick: move |_| { nav.go_back(); },
                    "Cancel"
                }
                button {
                    class: "btn-submit",
                    onclick: move |_| {
                        // TODO: validate and save to db
                        nav.go_back();
                    },
                    { if is_edit { "Save" } else { "Add" } }
                }
            }
        }
    }
}
