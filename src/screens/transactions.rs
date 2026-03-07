use dioxus::prelude::*;
use chrono::{Local, Datelike};
use crate::db::DbConnection;
use crate::models::model::TransactionType;
use crate::repository::transactions as tx_repo;
use crate::Route;

// Category icons seeded in order; DB ids are 1-based so id-1 = array index.
const CATEGORY_ICONS: &[&str] = &[
    "🍔", "🚌", "🛍", "💊", "🎬", "🏠", "✈️", "📚", "🎁",
];

fn category_icon(id: i16) -> &'static str {
    let idx = (id as usize).saturating_sub(1);
    CATEGORY_ICONS.get(idx).copied().unwrap_or("💸")
}

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
    icon:      &'static str,
    is_income: bool,
}

#[derive(Clone, PartialEq)]
struct DayGroup {
    date_label:  String,
    daily_total: f32,
    items:       Vec<TxRow>,
}

fn build_groups(month: u32, year: i32, db: &DbConnection) -> Vec<DayGroup> {
    let txs = tx_repo::get_by_month(db, month, year).unwrap_or_default();

    // Group by date (txs are already ordered DESC by date+time)
    let mut groups: Vec<DayGroup> = Vec::new();
    for tx in txs {
        let label = tx.tx_date.format("%b %-d, %Y").to_string().to_uppercase();
        let row = TxRow {
            id:        tx.id,
            title:     if tx.title.is_empty() { tx.description.clone() } else { tx.title.clone() },
            amount:    tx.amount,
            icon:      category_icon(tx.category),
            is_income: tx.tx_type == TransactionType::Income,
        };
        if let Some(g) = groups.last_mut().filter(|g| g.date_label == label) {
            g.daily_total += tx.amount;
            g.items.push(row);
        } else {
            groups.push(DayGroup { date_label: label, daily_total: tx.amount, items: vec![row] });
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
    let mtd_sign  = if mtd_total >= 0.0 { "+" } else { "" };
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
                span { class: "{mtd_class}", "{mtd_sign}${mtd_total:.2}" }
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
                            div { class: "tx-icon", "{tx.icon}" }
                            span { class: "tx-title", "{tx.title}" }
                            span { class: "tx-dots" }
                            span {
                                class: if tx.is_income { "tx-amount tx-amount--pos" } else { "tx-amount" },
                                { if tx.is_income {
                                    format!("+{:.2}", tx.amount)
                                } else {
                                    format!("({:.2})", tx.amount.abs())
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
