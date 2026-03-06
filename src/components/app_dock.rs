use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn AppDock() -> Element {
    rsx! {
        div {
            id: "appdock",
            ul {
                li { Link { to: Route::Home, "Home" } }
                li { Link { to: Route::Transactions, "Transactions" } }
                li { Link { to: Route::Settings, "Settings" } }
            }
        }
    }
}