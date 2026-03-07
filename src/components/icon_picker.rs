use dioxus::prelude::*;
use crate::icons::LUCIDE_ICONS;

#[component]
pub fn IconPicker(value: Signal<String>, open: Signal<bool>) -> Element {
    let mut icon_search = use_signal(String::new);

    let close = move |_| {
        open.set(false);
        icon_search.set(String::new());
    };

    rsx! {
        div {
            class: if *open.read() { "icon-drawer-backdrop visible" } else { "icon-drawer-backdrop" },
            onclick: close,
        }

        div {
            class: if *open.read() { "icon-drawer open" } else { "icon-drawer" },

            div { class: "drawer-handle" }
            p { class: "drawer-title", "SELECT ICON" }

            div { class: "icon-picker-row",
                if !value.read().is_empty() {
                    div { class: "icon-picker-preview",
                        i { class: "icon-{value}" }
                    }
                }
                input {
                    class: "drawer-input icon-picker-search",
                    r#type: "text",
                    placeholder: "Search icons…",
                    value: "{icon_search}",
                    oninput: move |e| icon_search.set(e.value()),
                }
            }

            div { class: "icon-picker-grid",
                {
                    let search = icon_search.read().to_lowercase();
                    let filtered: Vec<&str> = LUCIDE_ICONS.iter()
                        .filter(|name| search.is_empty() || name.contains(search.as_str()))
                        .take(60)
                        .copied()
                        .collect();
                    filtered.into_iter().map(|name| {
                        let n = name.to_string();
                        let selected = *value.read() == n;
                        rsx! {
                            button {
                                key: "{n}",
                                class: if selected { "icon-picker-cell icon-picker-cell--on" } else { "icon-picker-cell" },
                                onclick: move |_| {
                                    value.set(n.clone());
                                    open.set(false);
                                    icon_search.set(String::new());
                                },
                                i { class: "icon-{name}" }
                            }
                        }
                    })
                }
            }
        }
    }
}
