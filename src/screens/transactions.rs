use dioxus::prelude::*;
use chrono::{Duration, Local, Datelike, NaiveDate};
use std::collections::HashMap;
use crate::db::DbConnection;
use crate::models::model::{TransactionType, Transactions as TransactionModel};
use crate::repository::{transactions as tx_repo, categories as cat_repo};
use crate::Route;

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "JAN", 2 => "FEB", 3  => "MAR", 4  => "APR",
        5 => "MAY", 6 => "JUN", 7  => "JUL", 8  => "AUG",
        9 => "SEP", 10 => "OCT", 11 => "NOV", _  => "DEC",
    }
}

fn prev_month(m: u32, y: i32) -> (u32, i32) {
    if m == 1 { (12, y - 1) } else { (m - 1, y) }
}
fn next_month(m: u32, y: i32) -> (u32, i32) {
    if m == 12 { (1, y + 1) } else { (m + 1, y) }
}

#[derive(Clone, PartialEq)]
struct TxRow {
    id:        i32,
    title:     String,
    amount:    f32,
    icon:      String,
    is_income: bool,
}

#[derive(Clone, PartialEq)]
struct DayGroup {
    date_label:  String,
    daily_total: f32,
    items:       Vec<TxRow>,
}

fn expand_recurring(tx: &TransactionModel, year: i32, month: u32) -> Vec<TransactionModel> {
    let freq = match tx.frequency.as_deref() {
        Some(f) => f,
        None => return vec![],
    };

    let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let month_end = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    let till = tx.recurring_till.as_deref()
        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());

    // Effective end: earlier of month boundary or recurring_till+1
    let effective_end = match till {
        Some(t) if t < month_end => t + Duration::days(1),
        _ => month_end,
    };

    // Nothing to generate if start is after effective end
    if tx.tx_date >= effective_end {
        return vec![];
    }

    let mut result = vec![];

    match freq {
        "DAILY" => {
            let mut d = tx.tx_date.max(month_start);
            while d < effective_end {
                let mut occ = tx.clone();
                occ.tx_date = d;
                result.push(occ);
                d += Duration::days(1);
            }
        }
        "WEEKLY" => {
            let mut d = tx.tx_date;
            if d < month_start {
                let diff  = (month_start - d).num_days();
                let weeks = (diff + 6) / 7; // ceil to next week boundary
                d += Duration::weeks(weeks);
            }
            while d < effective_end {
                let mut occ = tx.clone();
                occ.tx_date = d;
                result.push(occ);
                d += Duration::weeks(1);
            }
        }
        "MONTHLY" => {
            if let Some(d) = NaiveDate::from_ymd_opt(year, month, tx.tx_date.day()) {
                if d >= tx.tx_date && d < effective_end {
                    let mut occ = tx.clone();
                    occ.tx_date = d;
                    result.push(occ);
                }
            }
        }
        "YEARLY" => {
            if tx.tx_date.month() == month {
                if let Some(d) = NaiveDate::from_ymd_opt(year, month, tx.tx_date.day()) {
                    if d >= tx.tx_date && d < effective_end {
                        let mut occ = tx.clone();
                        occ.tx_date = d;
                        result.push(occ);
                    }
                }
            }
        }
        _ => {}
    }

    result
}

fn build_groups(month: u32, year: i32, db: &DbConnection) -> Vec<DayGroup> {
    // Build category id → icon map
    let cat_map: HashMap<i16, String> = cat_repo::get_all(db)
        .unwrap_or_default()
        .into_iter()
        .map(|c| (c.id as i16, c.icon))
        .collect();

    // Non-recurring transactions for this month
    let mut all_txs = tx_repo::get_by_month(db, month, year).unwrap_or_default();

    // Expand recurring transactions into this month's occurrences
    let recurring = tx_repo::get_all_recurring(db).unwrap_or_default();
    for tx in &recurring {
        all_txs.extend(expand_recurring(tx, year, month));
    }

    // Sort by date DESC, time DESC before grouping
    all_txs.sort_by(|a, b| b.tx_date.cmp(&a.tx_date).then(b.tx_time.cmp(&a.tx_time)));

    // Group by date
    let mut groups: Vec<DayGroup> = Vec::new();
    for tx in all_txs {
        let label = tx.tx_date.format("%b %-d, %Y").to_string().to_uppercase();
        let is_income = tx.tx_type == TransactionType::Income;
        let signed = if is_income { tx.amount } else { -tx.amount };
        let row = TxRow {
            id:        tx.id,
            title:     if tx.title.is_empty() { tx.description.clone() } else { tx.title.clone() },
            amount:    signed,
            icon:      cat_map.get(&tx.category).cloned().unwrap_or_else(|| "circle".to_string()),
            is_income,
        };
        if let Some(g) = groups.last_mut().filter(|g| g.date_label == label) {
            g.daily_total += signed;
            g.items.push(row);
        } else {
            groups.push(DayGroup { date_label: label, daily_total: signed, items: vec![row] });
        }
    }
    groups
}

#[component]
pub fn Transactions() -> Element {
    let db  = use_context::<DbConnection>();
    let now = Local::now();
    let mut sel_month = use_signal(|| now.month());
    let mut sel_year  = use_signal(|| now.year());
    let nav = use_navigator();

    let groups = use_memo(move || build_groups(*sel_month.read(), *sel_year.read(), &db));

    let mtd_total: f32 = groups.read().iter()
        .map(|g| g.daily_total)
        .sum();

    let month_window = {
        let m = *sel_month.read();
        let y = *sel_year.read();
        let (pm, py) = prev_month(m, y);
        let (nm, ny) = next_month(m, y);
        vec![(pm, py), (m, y), (nm, ny)]
    };

    let mtd_class = if mtd_total >= 0.0 { "mtd-total mtd-total--pos" } else { "mtd-total mtd-total--neg" };
    let cur_mn    = month_name(*sel_month.read());
    let cur_yr    = *sel_year.read();

    rsx! {
        div {
            class: "ledger-screen",

            div {
                class: "ledger-header",
                h1 { class: "ledger-title", "THE LEDGER" }
            }

            div {
                class: "month-tabs",
                for (m, y) in month_window.iter().copied() {
                    button {
                        class: if m == *sel_month.read() && y == *sel_year.read()
                               { "month-tab month-tab--active" } else { "month-tab" },
                        onclick: move |_| { sel_month.set(m); sel_year.set(y); },
                        "[ {month_name(m)} ]"
                    }
                }
            }

            div {
                class: "mtd-row",
                span { class: "mtd-label", "{cur_mn} {cur_yr} MTD:" }
                span { class: "{mtd_class}",
                    { if mtd_total >= 0.0 {
                        format!("${:.2}", mtd_total.abs())
                    } else {
                        format!("${:.2}", mtd_total.abs())
                    }}
                }
            }

            div {
                class: "tx-list",
                if groups.read().is_empty() {
                    div {
                        class: "tx-empty",
                        p { "No transactions this month" }
                    }
                }
                for group in groups.read().iter() {
                    div {
                        class: "day-header",
                        span { class: "day-date", "{group.date_label}" }
                        span {
                            class: if group.daily_total >= 0.0 { "day-total day-total--pos" } else { "day-total day-total--neg" },
                            { if group.daily_total >= 0.0 {
                                format!("+${:.2}", group.daily_total)
                            } else {
                                format!("-${:.2}", group.daily_total.abs())
                            }}
                        }
                    }
                    for tx in group.items.iter() {
                        div {
                            class: "tx-row",
                            onclick: {
                                let id = tx.id;
                                move |_| { nav.push(Route::EditTransaction { id }); }
                            },
                            div { class: "tx-icon", i { class: "icon-{tx.icon}" } }
                            span { class: "tx-title", "{tx.title}" }
                            span { class: "tx-dots" }
                            span {
                                class: if tx.is_income { "tx-amount tx-amount--pos" } else { "tx-amount" },
                                { if tx.amount >= 0.0 {
                                    format!("+{:.2}", tx.amount)
                                } else {
                                    format!("{:.2}", tx.amount)
                                }}
                            }
                        }
                    }
                }
            }
        }

        button {
            class: "fab",
            onclick: move |_| { nav.push(Route::AddTransaction); },
            "+"
        }
    }
}
