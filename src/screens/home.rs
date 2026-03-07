use crate::db::DbConnection;
use crate::models::model::{TransactionType, Transactions};
use crate::repository::{accounts as acct_repo, transactions as tx_repo};
use crate::Route;
use chrono::{Datelike, Duration, Local, NaiveDate};
use dioxus::prelude::*;

fn month_expense_from_txs(all_txs: &[Transactions], month: u32, year: i32) -> f32 {
    let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let month_end = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    all_txs.iter().filter_map(|tx| {
        if tx.tx_type != TransactionType::Expense { return None; }

        match tx.frequency.as_deref() {
            None => {
                // Non-recurring: check if it falls in this month
                if tx.tx_date >= month_start && tx.tx_date < month_end {
                    Some(tx.amount)
                } else {
                    None
                }
            }
            Some(freq) => {
                // Recurring: check if there's an occurrence this month
                let till = tx.recurring_till.as_deref()
                    .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
                let effective_end = match till {
                    Some(t) if t < month_end => t + Duration::days(1),
                    _ => month_end,
                };
                if tx.tx_date >= effective_end { return None; }

                let has_occurrence = match freq {
                    "DAILY" => tx.tx_date.max(month_start) < effective_end,
                    "WEEKLY" => {
                        let mut d = tx.tx_date;
                        if d < month_start {
                            let diff = (month_start - d).num_days();
                            let weeks = (diff + 6) / 7;
                            d += Duration::weeks(weeks);
                        }
                        d < effective_end
                    }
                    "MONTHLY" => NaiveDate::from_ymd_opt(year, month, tx.tx_date.day())
                        .map(|d| d >= tx.tx_date && d < effective_end)
                        .unwrap_or(false),
                    "YEARLY" => tx.tx_date.month() == month
                        && NaiveDate::from_ymd_opt(year, month, tx.tx_date.day())
                            .map(|d| d >= tx.tx_date && d < effective_end)
                            .unwrap_or(false),
                    _ => false,
                };
                if has_occurrence { Some(tx.amount) } else { None }
            }
        }
    }).sum()
}

#[component]
pub fn Home() -> Element {
    let db = use_context::<DbConnection>();
    let nav = use_navigator();
    let now = Local::now();

    let mut balance = use_signal(|| 0.0f32);
    let mut burn = use_signal(|| 0.0f32);
    let mut initialized = use_signal(|| false);

    {
        let db = db.clone();
        use_effect(move || {
            if *initialized.read() { return; }
            initialized.set(true);

            let accts = acct_repo::get_all(&db).unwrap_or_default();
            let all_txs = tx_repo::get_all(&db).unwrap_or_default();

            let base: f32 = accts.iter().map(|a| a.starting_balance).sum();
            let net: f32 = all_txs.iter().map(|tx| match tx.tx_type {
                TransactionType::Income   =>  tx.amount,
                TransactionType::Expense  => -tx.amount,
                TransactionType::Transfer =>  0.0,
            }).sum();
            balance.set(base + net);

            let month_burn = month_expense_from_txs(&all_txs, now.month(), now.year());
            burn.set(month_burn);
        });
    }

    let bal = *balance.read();
    let bal_whole = bal.abs().trunc() as i64;
    let bal_cents = ((bal.abs() - bal.abs().trunc()) * 100.0).round() as u32;
    let bal_neg = bal < 0.0;

    let burn_val = *burn.read();

    rsx! {
        div {
            class: "home-screen",

            div {
                class: "home-header",
                div {
                    class: "home-wordmark",
                    span { class: "wordmark-penny", "PENNY" }
                    span { class: "wordmark-wise", "WISE" }
                }
            }

            div {
                class: "balance-hero",
                p { class: "balance-label", "TOTAL BALANCE" }
                div {
                    class: "balance-display",
                    if bal_neg { span { class: "balance-currency", "−$" } }
                    else { span { class: "balance-currency", "$" } }
                    span { class: "balance-whole", "{bal_whole}" }
                    span { class: "balance-cents", ".{bal_cents:02}" }
                }
                div {
                    class: "burn-pill",
                    span { class: "burn-value", "−${burn_val:.2}" }
                    span { class: "burn-sep" }
                    span { class: "burn-tag", "MTH EXPENSE" }
                }
            }

            div {
                class: "action-grid",

                button {
                    class: "action-card action-card--primary",
                    style: "animation-delay: 0ms",
                    onclick: move |_| { nav.push(Route::AddTransaction); },
                    div { class: "card-glyph", "+" }
                    span { class: "card-label", "ADD" }
                }
                button {
                    class: "action-card",
                    style: "animation-delay: 60ms",
                    onclick: move |_| { nav.push(Route::AddTransaction); },
                    div { class: "card-glyph", "⇄" }
                    span { class: "card-label", "TRANSFER" }
                }
                button {
                    class: "action-card",
                    style: "animation-delay: 120ms",
                    onclick: move |_| { nav.push(Route::Transactions); },
                    div { class: "card-glyph", "≡" }
                    span { class: "card-label", "LEDGER" }
                }
                button {
                    class: "action-card",
                    style: "animation-delay: 180ms",
                    onclick: move |_| { nav.push(Route::Settings); },
                    div { class: "card-glyph", "◎" }
                    span { class: "card-label", "SETTINGS" }
                }
            }
        }
    }
}
