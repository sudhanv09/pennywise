use dioxus::prelude::*;
use crate::db::DbConnection;
use crate::icons::LUCIDE_ICONS;
use crate::models::model::Category;
use crate::repository::categories as repo;

#[component]
pub fn Categories() -> Element {
    let db  = use_context::<DbConnection>();
    let nav = use_navigator();

    let mut items:      Signal<Vec<Category>> = use_signal(Vec::new);
    let mut editing_id: Signal<i32>           = use_signal(|| -1i32);
    let mut drawer_open                       = use_signal(|| false);

    let mut f_name = use_signal(String::new);
    let mut f_icon = use_signal(String::new);
    let mut icon_search = use_signal(String::new);

    {
        let db = db.clone();
        use_effect(move || {
            items.set(repo::get_all(&db).unwrap_or_default());
        });
    }

    let is_new = *editing_id.read() == -1;

    let open_new = move |_| {
        f_name.set(String::new());
        f_icon.set(String::new());
        icon_search.set(String::new());
        editing_id.set(-1);
        drawer_open.set(true);
    };

    rsx! {
        div {
            class: "subscreen",

            div {
                class: "subscreen-header",
                button {
                    class: "txform-close",
                    onclick: move |_| nav.go_back(),
                    "×"
                }
                span { class: "txform-title", "CATEGORIES" }
                button {
                    class: "subscreen-add-btn",
                    onclick: open_new,
                    "+"
                }
            }

            div {
                class: "subscreen-list",
                {items.read().iter().map(|c| {
                    let id   = c.id;
                    let name = c.name.clone();
                    let icon = c.icon.clone();
                    let c    = c.clone();
                    rsx! {
                        button {
                            key: "{id}",
                            class: "settings-row",
                            onclick: move |_| {
                                f_name.set(c.name.clone());
                                f_icon.set(c.icon.clone());
                                icon_search.set(String::new());
                                editing_id.set(id);
                                drawer_open.set(true);
                            },
                            div { class: "settings-row-main",
                                div { class: "settings-row-with-icon",
                                    i { class: "settings-row-icon icon-{icon}" }
                                    span { class: "settings-row-label", "{name}" }
                                }
                            }
                            span { class: "settings-row-chevron", "›" }
                        }
                    }
                })}
            }
        }

        div {
            class: if *drawer_open.read() { "drawer-backdrop visible" } else { "drawer-backdrop" },
            onclick: move |_| drawer_open.set(false),
        }

        div {
            class: if *drawer_open.read() { "settings-drawer open" } else { "settings-drawer" },
            div { class: "drawer-handle" }
            p { class: "drawer-title", { if is_new { "NEW CATEGORY" } else { "EDIT CATEGORY" } } }
            div { class: "drawer-fields",
                div { class: "drawer-field",
                    label { "NAME" }
                    input {
                        class: "drawer-input", r#type: "text", placeholder: "Category name",
                        value: "{f_name}",
                        oninput: move |e| f_name.set(e.value()),
                    }
                }
                div { class: "drawer-field",
                    label { "ICON" }
                    div { class: "icon-picker-row",
                        if !f_icon.read().is_empty() {
                            div { class: "icon-picker-preview",
                                i { class: "icon-{f_icon}" }
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
                                let selected = *f_icon.read() == n;
                                rsx! {
                                    button {
                                        key: "{n}",
                                        class: if selected { "icon-picker-cell icon-picker-cell--on" } else { "icon-picker-cell" },
                                        onclick: move |_| f_icon.set(n.clone()),
                                        i { class: "icon-{name}" }
                                    }
                                }
                            })
                        }
                    }
                }
            }
            div { class: "drawer-actions",
                if !is_new {
                    button {
                        class: "drawer-btn drawer-btn--delete",
                        onclick: {
                            let db = db.clone();
                            move |_| {
                                let _ = repo::delete(&db, *editing_id.read());
                                items.set(repo::get_all(&db).unwrap_or_default());
                                drawer_open.set(false);
                            }
                        },
                        "DELETE"
                    }
                }
                button {
                    class: "drawer-btn drawer-btn--save",
                    onclick: {
                        let db = db.clone();
                        move |_| {
                            let id = *editing_id.read();
                            let item = Category {
                                id,
                                name: f_name.read().clone(),
                                icon: f_icon.read().clone(),
                            };
                            if id == -1 { let _ = repo::insert(&db, &item); }
                            else        { let _ = repo::update(&db, &item); }
                            items.set(repo::get_all(&db).unwrap_or_default());
                            drawer_open.set(false);
                        }
                    },
                    "SAVE"
                }
            }
        }
    }
}
