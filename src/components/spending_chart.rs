use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub struct CatSlice {
    pub name: String,
    pub icon: String,
    pub amount: f32,
    pub pct: f32,
    pub color: &'static str,
}

pub const PIE_COLORS: &[&str] = &[
    "#f0b429", "#ff5b5b", "#5effa3", "#5bc8ff",
    "#c084fc", "#fb923c", "#f472b6", "#a3e635",
];

#[component]
pub fn SpendingChart(categories: Vec<CatSlice>, month_label: String) -> Element {
    rsx! {
        div {
            class: "home-section",
            style: "animation-delay: 200ms",
            div {
                class: "home-section-header",
                span { class: "home-section-title", "SPENDING" }
                span { class: "home-section-sub", "{month_label}" }
            }
            div {
                class: "pie-container",
                div {
                    class: "pie-chart",
                    style: {
                        let mut stops = Vec::new();
                        let mut cursor = 0.0f32;
                        for cat in categories.iter() {
                            let start = cursor;
                            cursor += cat.pct;
                            stops.push(format!("{} {:.1}% {:.1}%", cat.color, start, cursor));
                        }
                        if cursor < 100.0 {
                            stops.push(format!("#1a1f2e {:.1}% 100%", cursor));
                        }
                        format!("background: conic-gradient({});", stops.join(", "))
                    },
                    div { class: "pie-hole" }
                }
                div {
                    class: "pie-legend",
                    for cat in categories.iter() {
                        div {
                            class: "pie-legend-row",
                            span {
                                class: "pie-legend-dot",
                                style: "background: {cat.color}",
                            }
                            span { class: "pie-legend-name", "{cat.name}" }
                            span { class: "pie-legend-amt", { format!("${:.0}", cat.amount) } }
                        }
                    }
                }
            }
        }
    }
}
