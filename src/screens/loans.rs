use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;
use pennywise::db::DbConnection;
use pennywise::models::model::Loans as LoanModel;
use pennywise::repository::{loans as repo, transactions as tx_repo};

struct LoanAnalytics {
    remaining: f32,
    pct: f32,
    monthly_rate: Option<f32>,
    months_left: Option<f32>,
    est_payoff: Option<String>,
    status: LoanStatus,
}

enum LoanStatus {
    PaidOff,
    OnTrack,
    Behind,
    NoData,
}

fn compute_analytics(l: &LoanModel, db: &DbConnection) -> LoanAnalytics {
    let paid = tx_repo::sum_for_loan(db, l.id).unwrap_or(0.0);
    let remaining = l.total_amount - paid;
    let pct = if l.total_amount > 0.0 { (paid / l.total_amount * 100.0).min(100.0) } else { 0.0 };

    if remaining <= 0.0 {
        return LoanAnalytics { remaining: 0.0, pct: 100.0, monthly_rate: None, months_left: None, est_payoff: None, status: LoanStatus::PaidOff };
    }

    let (_, first_date, last_date, count) = repo::payment_stats(db, l.id).unwrap_or((0.0, None, None, 0));

    if count < 2 {
        return LoanAnalytics { remaining, pct, monthly_rate: None, months_left: None, est_payoff: None, status: LoanStatus::NoData };
    }

    let first = NaiveDate::parse_from_str(first_date.as_deref().unwrap_or(""), "%Y-%m-%d");
    let last  = NaiveDate::parse_from_str(last_date.as_deref().unwrap_or(""), "%Y-%m-%d");

    if let (Ok(f), Ok(la)) = (first, last) {
        let span_days = (la - f).num_days().max(1) as f32;
        let span_months = span_days / 30.44;
        let monthly_rate = paid / span_months;

        if monthly_rate <= 0.0 {
            return LoanAnalytics { remaining, pct, monthly_rate: None, months_left: None, est_payoff: None, status: LoanStatus::NoData };
        }

        let months_left = remaining / monthly_rate;
        let est_payoff_date = Local::now().date_naive() + chrono::Duration::days((months_left * 30.44) as i64);
        let est_payoff = est_payoff_date.format("%b %Y").to_string();

        let due_date = NaiveDate::from_num_days_from_ce_opt(l.due as i32);
        let status = match due_date {
            Some(dd) if est_payoff_date <= dd => LoanStatus::OnTrack,
            Some(_) => LoanStatus::Behind,
            None => LoanStatus::OnTrack,
        };

        LoanAnalytics { remaining, pct, monthly_rate: Some(monthly_rate), months_left: Some(months_left), est_payoff: Some(est_payoff), status }
    } else {
        LoanAnalytics { remaining, pct, monthly_rate: None, months_left: None, est_payoff: None, status: LoanStatus::NoData }
    }
}

fn days_to_date_str(days: f32) -> String {
    NaiveDate::from_num_days_from_ce_opt(days as i32)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn date_str_to_days(s: &str) -> f32 {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map(|d| d.num_days_from_ce() as f32)
        .unwrap_or(0.0)
}

#[component]
pub fn Loans() -> Element {
    let db  = use_context::<DbConnection>();
    let nav = use_navigator();

    let mut items:      Signal<Vec<LoanModel>> = use_signal(Vec::new);
    let mut editing_id: Signal<i32>        = use_signal(|| -1i32);
    let mut drawer_open                    = use_signal(|| false);

    let mut f_name   = use_signal(String::new);
    let mut f_total  = use_signal(String::new);
    let mut f_due    = use_signal(String::new);
    let mut f_lender = use_signal(|| false);

    {
        let db = db.clone();
        use_effect(move || {
            items.set(repo::get_all(&db).unwrap_or_default());
        });
    }

    let is_new = *editing_id.read() == -1;

    let open_new = move |_| {
        f_name.set(String::new());
        f_total.set(String::new());
        f_due.set(String::new());
        f_lender.set(false);
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
                span { class: "txform-title", "LOANS" }
                button {
                    class: "subscreen-add-btn",
                    onclick: open_new,
                    "+"
                }
            }

            div {
                class: "subscreen-list",
                {items.read().iter().map(|l| {
                    let id   = l.id;
                    let name = l.name.clone();
                    let kind = if l.is_lender { "LENDER" } else { "BORROWER" };
                    let a    = compute_analytics(l, &db);
                    let sub  = format!("{} • ${:.0} remaining", kind, a.remaining);
                    let pct_str  = format!("{:.0}%", a.pct);
                    let pct_width = format!("{}%", a.pct.min(100.0) as u32);
                    let pace_text = match (&a.monthly_rate, &a.months_left, &a.est_payoff) {
                        (Some(rate), Some(mo), Some(date)) => {
                            if *mo < 1.0 {
                                format!("${:.0}/mo — paid off within weeks", rate)
                            } else {
                                format!("${:.0}/mo — ~{:.1} months left (est. {})", rate, mo, date)
                            }
                        }
                        _ => String::new(),
                    };
                    let status_class = match a.status {
                        LoanStatus::PaidOff => "loan-status loan-status--done",
                        LoanStatus::OnTrack => "loan-status loan-status--ok",
                        LoanStatus::Behind  => "loan-status loan-status--behind",
                        LoanStatus::NoData  => "loan-status",
                    };
                    let status_text = match a.status {
                        LoanStatus::PaidOff => "PAID OFF",
                        LoanStatus::OnTrack => "ON TRACK",
                        LoanStatus::Behind  => "BEHIND SCHEDULE",
                        LoanStatus::NoData  => "",
                    };
                    let bar_class = match a.status {
                        LoanStatus::PaidOff => "loan-bar-fill loan-bar-fill--done",
                        LoanStatus::Behind  => "loan-bar-fill loan-bar-fill--behind",
                        _                   => "loan-bar-fill",
                    };
                    let l = l.clone();
                    rsx! {
                        button {
                            key: "{id}",
                            class: "loan-card",
                            onclick: move |_| {
                                f_name.set(l.name.clone());
                                f_total.set(format!("{:.2}", l.total_amount));
                                f_due.set(days_to_date_str(l.due));
                                f_lender.set(l.is_lender);
                                editing_id.set(id);
                                drawer_open.set(true);
                            },
                            div { class: "loan-card-top",
                                div { class: "settings-row-main",
                                    span { class: "settings-row-label", "{name}" }
                                    span { class: "settings-row-sub", "{sub}" }
                                }
                                span { class: "settings-row-chevron", "›" }
                            }
                            div { class: "loan-bar-row",
                                div { class: "loan-bar",
                                    div { class: "{bar_class}", style: "width: {pct_width}" }
                                }
                                span { class: "loan-pct", "{pct_str}" }
                            }
                            if !pace_text.is_empty() || !status_text.is_empty() {
                                div { class: "loan-insights",
                                    if !pace_text.is_empty() {
                                        span { class: "loan-pace", "{pace_text}" }
                                    }
                                    if !status_text.is_empty() {
                                        span { class: "{status_class}", "{status_text}" }
                                    }
                                }
                            }
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
            p { class: "drawer-title", { if is_new { "NEW LOAN" } else { "EDIT LOAN" } } }
            div { class: "drawer-fields",
                div { class: "drawer-field",
                    label { "NAME" }
                    input {
                        class: "drawer-input", r#type: "text", placeholder: "Person or lender name",
                        value: "{f_name}",
                        oninput: move |e| f_name.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "TOTAL AMOUNT" }
                    input {
                        class: "drawer-input", r#type: "number", step: "0.01", placeholder: "0.00",
                        value: "{f_total}",
                        oninput: move |e| f_total.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "DUE DATE" }
                    input {
                        class: "drawer-input", r#type: "date",
                        value: "{f_due}",
                        oninput: move |e| f_due.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "ROLE" }
                    div { class: "drawer-toggle-row",
                        button {
                            class: if !*f_lender.read() { "chip chip--on" } else { "chip" },
                            onclick: move |_| f_lender.set(false),
                            "BORROWER"
                        }
                        button {
                            class: if *f_lender.read() { "chip chip--on" } else { "chip" },
                            onclick: move |_| f_lender.set(true),
                            "LENDER"
                        }
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
                                let _ = repo::delete(&db, *editing_id.read());
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
                            let item = LoanModel {
                                id,
                                name:         f_name.read().clone(),
                                total_amount: f_total.read().parse().unwrap_or(0.0),
                                paid_amount:  0.0,
                                due:          date_str_to_days(&f_due.read()),
                                is_lender:    *f_lender.read(),
                            };
                            if id == -1 { let _ = repo::insert(&db, &item); }
                            else        { let _ = repo::update(&db, &item); }
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
