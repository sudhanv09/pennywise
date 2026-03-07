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
                class: "settings-menu",

                button {
                    class: "settings-menu-row",
                    onclick: move |_| { nav.push(Route::Accounts {}); },
                    div { class: "settings-menu-left",
                        span { class: "settings-menu-icon", "💳" }
                        span { class: "settings-menu-label", "Accounts" }
                    }
                    span { class: "settings-row-chevron", "›" }
                }
                button {
                    class: "settings-menu-row",
                    onclick: move |_| { nav.push(Route::Categories {}); },
                    div { class: "settings-menu-left",
                        span { class: "settings-menu-icon", "🗂" }
                        span { class: "settings-menu-label", "Categories" }
                    }
                    span { class: "settings-row-chevron", "›" }
                }
                button {
                    class: "settings-menu-row",
                    onclick: move |_| { nav.push(Route::Goals {}); },
                    div { class: "settings-menu-left",
                        span { class: "settings-menu-icon", "🎯" }
                        span { class: "settings-menu-label", "Goals" }
                    }
                    span { class: "settings-row-chevron", "›" }
                }
                button {
                    class: "settings-menu-row",
                    onclick: move |_| { nav.push(Route::Loans {}); },
                    div { class: "settings-menu-left",
                        span { class: "settings-menu-icon", "🤝" }
                        span { class: "settings-menu-label", "Loans" }
                    }
                    span { class: "settings-row-chevron", "›" }
                }
            }
        }
    }
}
