use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();

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
                    span { class: "balance-currency", "$" }
                    span { class: "balance-whole", "0" }
                    span { class: "balance-cents", ".00" }
                }
                div {
                    class: "burn-pill",
                    span { class: "burn-value", "−$0.00" }
                    span { class: "burn-sep" }
                    span { class: "burn-tag", "30D BURN" }
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
