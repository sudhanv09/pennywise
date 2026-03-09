use pennywise::db::DbConnection;
use pennywise::models::model::{TransactionType, Transactions};
use pennywise::repository::{
    accounts as acct_repo,
    categories as cat_repo,
    goals as goal_repo,
    loans as loan_repo,
    transactions as tx_repo,
};
use crate::components::balance_hero::BalanceHero;
use crate::components::spending_chart::{SpendingChart, CatSlice, PIE_COLORS};
use crate::components::recent_transactions::{RecentTransactions, RecentTx};
use crate::components::goals_section::{GoalsSection, GoalRow};
use crate::components::loans_section::{LoansSection, LoanRow};
use chrono::{Datelike, Duration, Local, NaiveDate};
use dioxus::prelude::*;
use std::collections::HashMap;

fn month_name(m: u32) -> &'static str {
    match m {
        1 => "JAN", 2 => "FEB", 3  => "MAR", 4  => "APR",
        5 => "MAY", 6 => "JUN", 7  => "JUL", 8  => "AUG",
        9 => "SEP", 10 => "OCT", 11 => "NOV", _  => "DEC",
    }
}

fn month_sum_by_type(
    all_txs: &[Transactions],
    month: u32,
    year: i32,
    tx_type: TransactionType,
) -> f32 {
    let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let month_end = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
    };

    all_txs
        .iter()
        .filter_map(|tx| {
            if tx.tx_type != tx_type {
                return None;
            }

            match tx.frequency.as_deref() {
                None => {
                    if tx.tx_date >= month_start && tx.tx_date < month_end {
                        Some(tx.amount)
                    } else {
                        None
                    }
                }
                Some(freq) => {
                    let till = tx
                        .recurring_till
                        .as_deref()
                        .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
                    let effective_end = match till {
                        Some(t) if t < month_end => t + Duration::days(1),
                        _ => month_end,
                    };
                    if tx.tx_date >= effective_end {
                        return None;
                    }

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
                        "YEARLY" => {
                            tx.tx_date.month() == month
                                && NaiveDate::from_ymd_opt(year, month, tx.tx_date.day())
                                    .map(|d| d >= tx.tx_date && d < effective_end)
                                    .unwrap_or(false)
                        }
                        _ => false,
                    };
                    if has_occurrence {
                        Some(tx.amount)
                    } else {
                        None
                    }
                }
            }
        })
        .sum()
}

#[derive(Clone)]
struct HomeData {
    balance: f32,
    month_income: f32,
    month_expense: f32,
    recent_txs: Vec<RecentTx>,
    top_categories: Vec<CatSlice>,
    goals: Vec<GoalRow>,
    loans: Vec<LoanRow>,
}

#[component]
pub fn Home() -> Element {
    let db = use_context::<DbConnection>();
    let now = Local::now();

    let mut data = use_signal(|| None::<HomeData>);
    let mut initialized = use_signal(|| false);

    {
        let db = db.clone();
        use_effect(move || {
            if *initialized.read() {
                return;
            }
            initialized.set(true);

            let accts = acct_repo::get_all(&db).unwrap_or_default();
            let all_txs = tx_repo::get_all(&db).unwrap_or_default();
            let categories = cat_repo::get_all(&db).unwrap_or_default();
            let goals = goal_repo::get_all(&db).unwrap_or_default();
            let loans = loan_repo::get_all(&db).unwrap_or_default();

            let base: f32 = accts.iter().map(|a| a.starting_balance).sum();
            let net: f32 = all_txs
                .iter()
                .map(|tx| match tx.tx_type {
                    TransactionType::Income => tx.amount,
                    TransactionType::Expense => -tx.amount,
                    TransactionType::Transfer => 0.0,
                })
                .sum();
            let balance = base + net;

            let month_income = month_sum_by_type(&all_txs, now.month(), now.year(), TransactionType::Income);
            let month_expense = month_sum_by_type(&all_txs, now.month(), now.year(), TransactionType::Expense);

            let cat_map: HashMap<i16, (String, String)> = categories
                .iter()
                .map(|c| (c.id as i16, (c.name.clone(), c.icon.clone())))
                .collect();

            let month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap();
            let month_end = if now.month() == 12 {
                NaiveDate::from_ymd_opt(now.year() + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(now.year(), now.month() + 1, 1).unwrap()
            };

            let mut month_txs: Vec<&Transactions> = all_txs
                .iter()
                .filter(|tx| {
                    tx.frequency.is_none()
                        && tx.tx_date >= month_start
                        && tx.tx_date < month_end
                })
                .collect();
            month_txs.sort_by(|a, b| {
                b.tx_date.cmp(&a.tx_date).then(b.tx_time.cmp(&a.tx_time))
            });

            let recent_txs: Vec<RecentTx> = month_txs
                .iter()
                .take(5)
                .map(|tx| {
                    let (_, icon) = cat_map
                        .get(&tx.category)
                        .cloned()
                        .unwrap_or_default();
                    let day = tx.tx_date.day();
                    let suffix = match day {
                        1 | 21 | 31 => "st",
                        2 | 22 => "nd",
                        3 | 23 => "rd",
                        _ => "th",
                    };
                    RecentTx {
                        id: tx.id,
                        title: tx.title.clone(),
                        amount: tx.amount,
                        icon,
                        is_income: tx.tx_type == TransactionType::Income,
                        date_label: format!("{}{}", day, suffix),
                    }
                })
                .collect();

            let mut cat_spend: HashMap<i16, f32> = HashMap::new();
            for tx in &month_txs {
                if tx.tx_type == TransactionType::Expense {
                    *cat_spend.entry(tx.category).or_default() += tx.amount;
                }
            }
            let total_cat_spend: f32 = cat_spend.values().sum();
            let mut sorted_cats: Vec<(i16, f32)> = cat_spend.into_iter().collect();
            sorted_cats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            sorted_cats.truncate(PIE_COLORS.len());
            let top_cats: Vec<CatSlice> = sorted_cats
                .iter()
                .enumerate()
                .map(|(i, (cat_id, amount))| {
                    let (name, icon) = cat_map
                        .get(cat_id)
                        .cloned()
                        .unwrap_or(("Other".into(), String::new()));
                    let pct = if total_cat_spend > 0.0 {
                        amount / total_cat_spend * 100.0
                    } else {
                        0.0
                    };
                    CatSlice {
                        name,
                        icon,
                        amount: *amount,
                        pct,
                        color: PIE_COLORS[i % PIE_COLORS.len()],
                    }
                })
                .collect();

            let goal_rows: Vec<GoalRow> = goals
                .iter()
                .map(|g| {
                    let saved = tx_repo::sum_for_goal(&db, g.id).unwrap_or(0.0);
                    let pct = if g.target > 0.0 {
                        (saved / g.target * 100.0).min(100.0)
                    } else {
                        0.0
                    };
                    GoalRow { name: g.name.clone(), target: g.target, current: saved, pct }
                })
                .collect();

            let loan_rows: Vec<LoanRow> = loans
                .iter()
                .map(|l| {
                    let paid = tx_repo::sum_for_loan(&db, l.id).unwrap_or(0.0);
                    let pct = if l.total_amount > 0.0 {
                        (paid / l.total_amount * 100.0).min(100.0)
                    } else {
                        0.0
                    };
                    LoanRow {
                        name: l.name.clone(),
                        total: l.total_amount,
                        paid,
                        pct,
                        is_lender: l.is_lender,
                    }
                })
                .collect();

            data.set(Some(HomeData {
                balance,
                month_income,
                month_expense,
                recent_txs,
                top_categories: top_cats,
                goals: goal_rows,
                loans: loan_rows,
            }));
        });
    }

    let d = data.read();
    let home = d.as_ref();

    let bal = home.map(|h| h.balance).unwrap_or(0.0);
    let month_income = home.map(|h| h.month_income).unwrap_or(0.0);
    let month_expense = home.map(|h| h.month_expense).unwrap_or(0.0);
    let month_label = month_name(now.month()).to_string();

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

            BalanceHero {
                bal,
                month_income,
                month_expense,
                month_label: month_label.clone(),
            }

            if let Some(h) = home {
                if !h.top_categories.is_empty() {
                    SpendingChart {
                        categories: h.top_categories.clone(),
                        month_label: month_label.clone(),
                    }
                }
            }

            if let Some(h) = home {
                if !h.recent_txs.is_empty() {
                    RecentTransactions { txs: h.recent_txs.clone() }
                }
            }

            if let Some(h) = home {
                if !h.goals.is_empty() {
                    GoalsSection { goals: h.goals.clone() }
                }
            }

            if let Some(h) = home {
                if !h.loans.is_empty() {
                    LoansSection { loans: h.loans.clone() }
                }
            }

            div { style: "height: 24px" }
        }
    }
}
