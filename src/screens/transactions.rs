use dioxus::prelude::*;
use chrono::{Local, Datelike};
use crate::Route;

#[derive(Clone, PartialEq)]
struct MockTx {
    title:  &'static str,
    amount: f32,
    icon:   &'static str,
}

#[derive(Clone, PartialEq)]
struct DayGroup {
    date_label:  String,
    daily_total: f32,
    items:       Vec<MockTx>,
}

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "JAN", 2 => "FEB", 3  => "MAR", 4  => "APR",
        5 => "MAY", 6 => "JUN", 7  => "JUL", 8  => "AUG",
        9 => "SEP", 10 => "OCT", 11 => "NOV", _  => "DEC",
    }
}

fn mock_groups(month: u32, year: i32) -> Vec<DayGroup> {
    let mn = month_name(month);
    vec![
        DayGroup {
            date_label: format!("{mn} 15, {year}"),
            daily_total: -42.50,
            items: vec![
                MockTx { title: "Uber Rides",         amount: -12.50, icon: "🚌" },
                MockTx { title: "Starbucks Coffee",   amount:  -4.50, icon: "☕" },
                MockTx { title: "Groceries",          amount: -25.50, icon: "🛍" },
            ],
        },
        DayGroup {
            date_label: format!("{mn} 12, {year}"),
            daily_total: 1200.0,
            items: vec![
                MockTx { title: "Freelance Inc.",     amount: 1200.0, icon: "💼" },
            ],
        },
        DayGroup {
            date_label: format!("{mn} 8, {year}"),
            daily_total: -189.99,
            items: vec![
                MockTx { title: "Netflix",            amount: -15.99, icon: "🎬" },
                MockTx { title: "Electric Bill",      amount: -74.00, icon: "⚡" },
                MockTx { title: "Spotify",            amount: -9.99,  icon: "🎵" },
                MockTx { title: "Phone Bill",         amount: -90.01, icon: "📱" },
            ],
        },
        DayGroup {
            date_label: format!("{mn} 1, {year}"),
            daily_total: -1200.0,
            items: vec![
                MockTx { title: "Rent",               amount: -1200.0, icon: "🏠" },
            ],
        },
    ]
}

fn prev_month(m: u32, y: i32) -> (u32, i32) {
    if m == 1 { (12, y - 1) } else { (m - 1, y) }
}
fn next_month(m: u32, y: i32) -> (u32, i32) {
    if m == 12 { (1, y + 1) } else { (m + 1, y) }
}

#[component]
pub fn Transactions() -> Element {
    let now = Local::now();
    let mut sel_month = use_signal(|| now.month());
    let mut sel_year  = use_signal(|| now.year());

    let nav = use_navigator();

    let groups = use_memo(move || mock_groups(*sel_month.read(), *sel_year.read()));

    let mtd_total: f32 = groups.read().iter()
        .flat_map(|g| g.items.iter())
        .map(|t| t.amount)
        .sum();

    // Build a window of 5 months to show in the tab strip
    let month_window: Vec<(u32, i32)> = {
        let m = *sel_month.read();
        let y = *sel_year.read();
        let (pm1, py1) = prev_month(m, y);
        let (nm1, ny1) = next_month(m, y);
        vec![(pm1, py1), (m, y), (nm1, ny1)]
    };

    let mtd_sign   = if mtd_total >= 0.0 { "+" } else { "" };
    let mtd_class  = if mtd_total >= 0.0 { "mtd-total mtd-total--pos" } else { "mtd-total mtd-total--neg" };
    let cur_mn     = month_name(*sel_month.read());
    let cur_yr     = *sel_year.read();

    rsx! {
        div {
            class: "ledger-screen",

            // ── Header ──────────────────────────────────────
            div {
                class: "ledger-header",
                h1 { class: "ledger-title", "THE LEDGER" }
                div {
                    class: "ledger-actions",
                    button { class: "ledger-icon-btn", "⌕" }
                    button { class: "ledger-icon-btn", "⚌" }
                }
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
                for group in groups.read().iter() {
                    // Date header
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
                    // Transactions
                    for tx in group.items.iter() {
                        div {
                            class: "tx-row",
                            onclick: move |_| { nav.push(Route::AddTransaction); },
                            div { class: "tx-icon", "{tx.icon}" }
                            span { class: "tx-title", "{tx.title}" }
                            span { class: "tx-dots" }
                            span {
                                class: if tx.amount >= 0.0 { "tx-amount tx-amount--pos" } else { "tx-amount" },
                                { if tx.amount >= 0.0 {
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
