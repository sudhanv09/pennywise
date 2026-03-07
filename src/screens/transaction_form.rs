use pennywise::db::DbConnection;
use pennywise::models::model::{Account, BillingCycle, Category, Goals, Loans, TransactionType, Transactions};
use pennywise::repository::{
    accounts as acct_repo, categories as cat_repo, goals as goal_repo, loans as loan_repo,
    transactions as tx_repo,
};
use chrono::{Local, NaiveDate, NaiveTime};
use dioxus::prelude::*;

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

    let mut amount = use_signal(String::new);
    let mut tx_type = use_signal(|| TransactionType::Expense);
    let mut selected_cat_id: Signal<i32> = use_signal(|| 0);
    let mut selected_acct_id: Signal<i32> = use_signal(|| 0);
    let mut selected_tags: Signal<Vec<usize>> = use_signal(Vec::new);
    let mut title = use_signal(String::new);
    let mut note = use_signal(String::new);
    let mut cat_drawer = use_signal(|| false);
    let mut tx_date = use_signal(|| Local::now().date_naive());
    let mut tx_time = use_signal(|| Local::now().time());

    let mut categories: Signal<Vec<Category>> = use_signal(Vec::new);
    let mut accounts: Signal<Vec<Account>> = use_signal(Vec::new);
    let mut goals: Signal<Vec<Goals>> = use_signal(Vec::new);
    let mut loans: Signal<Vec<Loans>> = use_signal(Vec::new);
    let mut selected_goal_id: Signal<Option<i32>> = use_signal(|| None);
    let mut selected_loan_id: Signal<Option<i32>> = use_signal(|| None);
    let mut selected_to_acct_id: Signal<Option<i32>> = use_signal(|| None);
    let mut frequency: Signal<Option<BillingCycle>> = use_signal(|| None);
    let mut recurring_till: Signal<String> = use_signal(String::new);

    let is_edit = id.is_some();
    let nav = use_navigator();
    let mut initialized = use_signal(|| false);

    {
        let db = db.clone();
        use_effect(move || {
            if *initialized.read() {
                return;
            }
            initialized.set(true);

            let loaded_cats = cat_repo::get_all(&db).unwrap_or_default();
            let loaded_accts = acct_repo::get_all(&db).unwrap_or_default();
            let loaded_goals = goal_repo::get_all(&db).unwrap_or_default();
            let loaded_loans = loan_repo::get_all(&db).unwrap_or_default();

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
                    title.set(tx.title);
                    note.set(tx.description);
                    tx_date.set(tx.tx_date);
                    tx_time.set(tx.tx_time);
                    selected_goal_id.set(tx.goal_id);
                    selected_loan_id.set(tx.loan_id);
                    selected_to_acct_id.set(tx.to_account.map(|a| a as i32));
                    frequency.set(tx.frequency.as_deref().and_then(BillingCycle::from_str));
                    recurring_till.set(tx.recurring_till.clone().unwrap_or_default());
                    if tx.frequency.is_some() {
                        let mut t = selected_tags.write();
                        if !t.contains(&3) {
                            t.push(3);
                        }
                    }
                }
            }

            categories.set(loaded_cats);
            accounts.set(loaded_accts);
            goals.set(loaded_goals);
            loans.set(loaded_loans);
        });
    }

    let tx_date_str = tx_date.read().format("%Y-%m-%d").to_string();
    let tx_time_str = tx_time.read().format("%H:%M").to_string();

    let cat_icon = {
        let cats = categories.read();
        let id = *selected_cat_id.read();
        cats.iter()
            .find(|c| c.id == id)
            .map(|c| c.icon.clone())
            .unwrap_or_default()
    };
    let cat_label = {
        let cats = categories.read();
        let id = *selected_cat_id.read();
        cats.iter()
            .find(|c| c.id == id)
            .map(|c| c.name.to_uppercase())
            .unwrap_or_else(|| "SELECT".to_string())
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
                div {
                    class: "txform-datetime-row",
                    input {
                        class: "txform-date-pill",
                        r#type: "date",
                        value: "{tx_date_str}",
                        oninput: move |e| {
                            if let Ok(d) = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d") {
                                tx_date.set(d);
                            }
                        },
                    }
                    input {
                        class: "txform-time-pill",
                        r#type: "time",
                        value: "{tx_time_str}",
                        oninput: move |e| {
                            if let Ok(t) = NaiveTime::parse_from_str(&e.value(), "%H:%M") {
                                tx_time.set(t);
                            }
                        },
                    }
                }
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
                    input {
                        class: "txform-amount-input",
                        r#type: "number",
                        min: "0",
                        step: "0.01",
                        placeholder: "$0.00",
                        value: "{amount}",
                        oninput: move |e| amount.set(e.value()),
                    }
                }

                div {
                    class: "txform-meta-row",
                    input {
                        class: "txform-title-input",
                        r#type: "text",
                        placeholder: "Title…",
                        value: "{title}",
                        oninput: move |e| title.set(e.value()),
                    }
                    button {
                        class: "cat-pill",
                        onclick: move |_| cat_drawer.set(true),
                        i { class: "icon-{cat_icon}" }
                        span { "{cat_label}" }
                        span { class: "cat-pill-arrow", "›" }
                    }
                }
            }

            div {
                class: "txform-section",
                p { class: "txform-section-label",
                    { if *tx_type.read() == TransactionType::Transfer { "FROM ACCOUNT" } else { "ACCOUNT" } }
                }
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

            if *tx_type.read() == TransactionType::Transfer {
                div {
                    class: "txform-section",
                    p { class: "txform-section-label", "TO ACCOUNT" }
                    div {
                        class: "chip-row",
                        {accounts.read().iter().map(|acct| {
                            let acct_id   = acct.id;
                            let acct_name = acct.name.to_uppercase();
                            let selected  = *selected_to_acct_id.read() == Some(acct_id);
                            rsx! {
                                button {
                                    key: "{acct_id}",
                                    class: if selected { "chip chip--on" } else { "chip" },
                                    onclick: move |_| selected_to_acct_id.set(Some(acct_id)),
                                    "{acct_name}"
                                }
                            }
                        })}
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
                                if t.contains(&i) {
                                    t.retain(|&x| x != i);
                                    if i == 3 {
                                        frequency.set(None);
                                        recurring_till.set(String::new());
                                    }
                                } else {
                                    t.push(i);
                                }
                            },
                            "{tag}"
                        }
                    }
                }
            }

            if selected_tags.read().contains(&3) {
                div {
                    class: "txform-section",
                    p { class: "txform-section-label", "FREQUENCY" }
                    div {
                        class: "chip-row",
                        {BillingCycle::all().iter().map(|cycle| {
                            let label = cycle.as_str();
                            let cycle = cycle.clone();
                            rsx! {
                                button {
                                    class: if *frequency.read() == Some(cycle.clone()) { "chip chip--on" } else { "chip" },
                                    onclick: move |_| frequency.set(Some(cycle.clone())),
                                    "{label}"
                                }
                            }
                        })}
                    }
                    div { class: "txform-till-row",
                        span { class: "txform-section-label", "TILL" }
                        input {
                            class: "txform-date-pill",
                            r#type: "date",
                            value: "{recurring_till}",
                            oninput: move |e| recurring_till.set(e.value()),
                        }
                    }
                }
            }

            if !goals.read().is_empty() {
                div {
                    class: "txform-section",
                    p { class: "txform-section-label", "GOAL" }
                    div {
                        class: "chip-row",
                        button {
                            class: if selected_goal_id.read().is_none() { "chip chip--on" } else { "chip" },
                            onclick: move |_| selected_goal_id.set(None),
                            "NONE"
                        }
                        {goals.read().iter().map(|g| {
                            let gid  = g.id;
                            let name = g.name.to_uppercase();
                            let selected = *selected_goal_id.read() == Some(gid);
                            rsx! {
                                button {
                                    key: "{gid}",
                                    class: if selected { "chip chip--on" } else { "chip" },
                                    onclick: move |_| selected_goal_id.set(Some(gid)),
                                    "{name}"
                                }
                            }
                        })}
                    }
                }
            }

            if !loans.read().is_empty() {
                div {
                    class: "txform-section",
                    p { class: "txform-section-label", "LOAN" }
                    div {
                        class: "chip-row",
                        button {
                            class: if selected_loan_id.read().is_none() { "chip chip--on" } else { "chip" },
                            onclick: move |_| selected_loan_id.set(None),
                            "NONE"
                        }
                        {loans.read().iter().map(|l| {
                            let lid  = l.id;
                            let name = l.name.to_uppercase();
                            let selected = *selected_loan_id.read() == Some(lid);
                            rsx! {
                                button {
                                    key: "{lid}",
                                    class: if selected { "chip chip--on" } else { "chip" },
                                    onclick: move |_| selected_loan_id.set(Some(lid)),
                                    "{name}"
                                }
                            }
                        })}
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
                if is_edit {
                    button {
                        class: "txform-delete",
                        onclick: {
                            let db = db.clone();
                            move |_| {
                                if let Some(edit_id) = id {
                                    let _ = tx_repo::delete(&db, edit_id);
                                }
                                nav.go_back();
                            }
                        },
                        "Delete"
                    }
                }
                button {
                    class: "txform-submit",
                    onclick: {
                        let db = db.clone();
                        move |_| {
                            let amt: f32 = amount.read().parse().unwrap_or(0.0);
                            let tx = Transactions {
                                id:          id.unwrap_or(0),
                                title:       title.read().clone(),
                                amount:      amt,
                                tx_date:     *tx_date.read(),
                                tx_time:     *tx_time.read(),
                                tx_type:     tx_type.read().clone(),
                                category:    *selected_cat_id.read() as i16,
                                account:     *selected_acct_id.read() as i16,
                                description: note.read().clone(),
                                goal_id:     *selected_goal_id.read(),
                                loan_id:     *selected_loan_id.read(),
                                frequency:      frequency.read().as_ref().map(|f| f.as_str().to_string()),
                                recurring_till: if recurring_till.read().is_empty() { None } else { Some(recurring_till.read().clone()) },
                                to_account:  if tx_type.read().clone() == TransactionType::Transfer {
                                                 (*selected_to_acct_id.read()).map(|a| a as i16)
                                             } else {
                                                 None
                                             },
                                subscription_id: None,
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
                            i { class: "cat-icon icon-{cat_icon}" }
                            span { class: "cat-label", "{cat_name}" }
                        }
                    }
                })}
            }
        }
    }
}
