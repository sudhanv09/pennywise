use dioxus::prelude::*;
use crate::Route;

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
                    span { class: "settings-grid-icon", "💳" }
                    span { class: "settings-grid-label", "Accounts" }
                }
                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Categories {}); },
                    span { class: "settings-grid-icon", "🗂" }
                    span { class: "settings-grid-label", "Categories" }
                }
                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Goals {}); },
                    span { class: "settings-grid-icon", "🎯" }
                    span { class: "settings-grid-label", "Goals" }
                }
                button {
                    class: "settings-grid-card",
                    onclick: move |_| { nav.push(Route::Loans {}); },
                    span { class: "settings-grid-icon", "🤝" }
                    span { class: "settings-grid-label", "Loans" }
                }
            }
        }
    }
}
