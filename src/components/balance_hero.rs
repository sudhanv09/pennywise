use dioxus::prelude::*;

#[component]
pub fn BalanceHero(bal: f32, month_income: f32, month_expense: f32, month_label: String) -> Element {
    let bal_whole = bal.abs().trunc() as i64;
    let bal_cents = ((bal.abs() - bal.abs().trunc()) * 100.0).round() as u32;
    let bal_neg = bal < 0.0;

    rsx! {
        div {
            class: "balance-hero",
            p { class: "balance-label", "TOTAL BALANCE" }
            div {
                class: "balance-display",
                if bal_neg { span { class: "balance-currency", "\u{2212}$" } }
                else { span { class: "balance-currency", "$" } }
                span { class: "balance-whole", "{bal_whole}" }
                span { class: "balance-cents", ".{bal_cents:02}" }
            }
            div {
                class: "month-pills",
                div {
                    class: "burn-pill burn-pill--income",
                    span { class: "burn-icon icon-trending-up" }
                    span { class: "burn-value burn-value--income", { format!("${:.0}", month_income.abs()) } }
                }
                div {
                    class: "burn-pill burn-pill--expense",
                    span { class: "burn-icon icon-trending-down" }
                    span { class: "burn-value burn-value--expense", { format!("${:.0}", month_expense) } }
                }
                div {
                    class: "burn-pill",
                    span { class: "burn-tag", "{month_label}" }
                }
            }
        }
    }
}
