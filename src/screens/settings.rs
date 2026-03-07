use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let nav = use_navigator();

    rsx! {
        div {
            class: "settings",

            div {
                class: "settings-header",
                span { class: "txform-title", "SETTINGS" }
            }

            div {
                class: "settings-grid",

                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Accounts {}); },
                    i { class: "settings-grid-icon icon-credit-card" }
                    span { class: "settings-grid-label", "Accounts" }
                }
                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Categories {}); },
                    i { class: "settings-grid-icon icon-tags" }
                    span { class: "settings-grid-label", "Categories" }
                }
                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Goals {}); },
                    i { class: "settings-grid-icon icon-target" }
                    span { class: "settings-grid-label", "Goals" }
                }
                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Loans {}); },
                    i { class: "settings-grid-icon icon-handshake" }
                    span { class: "settings-grid-label", "Loans" }
                }
            }
        }
    }
}
