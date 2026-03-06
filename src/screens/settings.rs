use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            h1 { "Hi from Screen 2" }
            Link { to: Route::Home, "Back to Screen 1" }
        }
    }
}
