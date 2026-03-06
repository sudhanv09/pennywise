use dioxus::prelude::*;

#[component]
pub fn Transactions() -> Element {
    rsx! {
        div {
            h1 { "Transactions" }
        }
    }
}
