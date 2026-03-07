use chrono::Local;
use dioxus::prelude::*;
use crate::db::DbConnection;
use crate::models::model::{Account, BillingCycle, Category, Subscription};
use crate::repository::{
    accounts as acct_repo, categories as cat_repo,
    subscriptions as repo, transactions as tx_repo,
};

#[component]
pub fn Subscriptions() -> Element {
    let db  = use_context::<DbConnection>();
    let nav = use_navigator();

    let mut items:      Signal<Vec<Subscription>> = use_signal(Vec::new);
    let mut categories: Signal<Vec<Category>>     = use_signal(Vec::new);
    let mut accounts:   Signal<Vec<Account>>      = use_signal(Vec::new);
    let mut editing_id: Signal<i32>               = use_signal(|| -1i32);
    let mut drawer_open                           = use_signal(|| false);

    let mut f_name       = use_signal(String::new);
    let mut f_amount     = use_signal(String::new);
    let mut f_cycle      = use_signal(|| BillingCycle::Monthly);
    let mut f_start      = use_signal(String::new);
    let mut f_end        = use_signal(String::new);
    let mut f_cat_id:  Signal<i32> = use_signal(|| 0);
    let mut f_acct_id: Signal<i32> = use_signal(|| 0);

    {
        let db = db.clone();
        use_effect(move || {
            items.set(repo::get_all(&db).unwrap_or_default());
            let cats = cat_repo::get_all(&db).unwrap_or_default();
            let accts = acct_repo::get_all(&db).unwrap_or_default();
            if !cats.is_empty() && *f_cat_id.read() == 0 {
                f_cat_id.set(cats[0].id);
            }
            if !accts.is_empty() && *f_acct_id.read() == 0 {
                f_acct_id.set(accts[0].id);
            }
            categories.set(cats);
            accounts.set(accts);
        });
    }

    let is_new = *editing_id.read() == -1;

    let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
    let open_new = move |_| {
        f_name.set(String::new());
        f_amount.set(String::new());
        f_cycle.set(BillingCycle::Monthly);
        f_start.set(today.clone());
        f_end.set(String::new());
        if let Some(c) = categories.read().first() { f_cat_id.set(c.id); }
        if let Some(a) = accounts.read().first() { f_acct_id.set(a.id); }
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
                span { class: "txform-title", "SUBSCRIPTIONS" }
                button {
                    class: "subscreen-add-btn",
                    onclick: open_new,
                    "+"
                }
            }

            div {
                class: "subscreen-list",
                {items.read().iter().map(|s| {
                    let id   = s.id;
                    let name = s.name.clone();
                    let sub  = format!("${:.2} • {}", s.amount, s.billing_cycle.as_str());
                    let s    = s.clone();
                    rsx! {
                        button {
                            key: "{id}",
                            class: "settings-row",
                            onclick: move |_| {
                                f_name.set(s.name.clone());
                                f_amount.set(format!("{:.2}", s.amount));
                                f_cycle.set(s.billing_cycle.clone());
                                f_start.set(s.start_date.clone());
                                f_end.set(s.end_date.clone());
                                f_cat_id.set(s.category_id);
                                f_acct_id.set(s.account_id);
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
            p { class: "drawer-title", { if is_new { "NEW SUBSCRIPTION" } else { "EDIT SUBSCRIPTION" } } }
            div { class: "drawer-fields",
                div { class: "drawer-field",
                    label { "NAME" }
                    input {
                        class: "drawer-input", r#type: "text", placeholder: "Subscription name",
                        value: "{f_name}",
                        oninput: move |e| f_name.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "AMOUNT" }
                    input {
                        class: "drawer-input", r#type: "number", step: "0.01", placeholder: "0.00",
                        value: "{f_amount}",
                        oninput: move |e| f_amount.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "BILLING CYCLE" }
                    div { class: "chip-row",
                        {BillingCycle::all().iter().map(|cycle| {
                            let label = cycle.as_str();
                            let cycle = cycle.clone();
                            let selected = *f_cycle.read() == cycle;
                            rsx! {
                                button {
                                    class: if selected { "chip chip--on" } else { "chip" },
                                    onclick: move |_| f_cycle.set(cycle.clone()),
                                    "{label}"
                                }
                            }
                        })}
                    }
                }
                div { class: "drawer-field",
                    label { "START DATE" }
                    input {
                        class: "drawer-input", r#type: "date",
                        value: "{f_start}",
                        oninput: move |e| f_start.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "END DATE" }
                    input {
                        class: "drawer-input", r#type: "date",
                        value: "{f_end}",
                        oninput: move |e| f_end.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "CATEGORY" }
                    div { class: "chip-row",
                        {categories.read().iter().map(|cat| {
                            let cid  = cat.id;
                            let name = cat.name.to_uppercase();
                            let selected = *f_cat_id.read() == cid;
                            rsx! {
                                button {
                                    key: "{cid}",
                                    class: if selected { "chip chip--on" } else { "chip" },
                                    onclick: move |_| f_cat_id.set(cid),
                                    "{name}"
                                }
                            }
                        })}
                    }
                }
                div { class: "drawer-field",
                    label { "ACCOUNT" }
                    div { class: "chip-row",
                        {accounts.read().iter().map(|acct| {
                            let aid  = acct.id;
                            let name = acct.name.to_uppercase();
                            let selected = *f_acct_id.read() == aid;
                            rsx! {
                                button {
                                    key: "{aid}",
                                    class: if selected { "chip chip--on" } else { "chip" },
                                    onclick: move |_| f_acct_id.set(aid),
                                    "{name}"
                                }
                            }
                        })}
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
                                let id = *editing_id.read();
                                let _ = tx_repo::delete_by_subscription(&db, id);
                                let _ = repo::delete(&db, id);
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
                            let item = Subscription {
                                id,
                                name:          f_name.read().clone(),
                                amount:        f_amount.read().parse().unwrap_or(0.0),
                                billing_cycle: f_cycle.read().clone(),
                                start_date:    f_start.read().clone(),
                                end_date:      f_end.read().clone(),
                                category_id:   *f_cat_id.read(),
                                account_id:    *f_acct_id.read(),
                            };
                            if id == -1 {
                                if let Ok(new_id) = repo::insert(&db, &item) {
                                    repo::generate_transactions(&db, new_id as i32, &item);
                                }
                            } else {
                                let _ = repo::update(&db, &item);
                                let _ = tx_repo::delete_by_subscription(&db, id);
                                repo::generate_transactions(&db, id, &item);
                            }
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
