use dioxus::prelude::*;

#[component]
pub fn AppDock() -> Element {
    rsx! {
        div {
            id: "appdock",
            ul { 
                li { "Home" },
                li { "Transactions" },
                li { "Other" },
            }
        }
    }
}