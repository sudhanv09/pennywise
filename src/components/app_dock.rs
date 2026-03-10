use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn AppDock() -> Element {
    rsx! {
        div {
            id: "appdock",
            ul {
                li { Link { to: Route::Home, i { class: "icon-home" } "Home" } }
                li { Link { to: Route::Transactions, i { class: "icon-receipt-text" } "Transactions" } }
                li {
                    class: "dock-fab-item",
                    Link {
                        to: Route::AddTransaction,
                        class: "dock-fab",
                        i { class: "icon-plus" }
                    }
                }
                li { Link { to: Route::Settings, i { class: "icon-settings" } "Settings" } }
            }
        }
    }
}