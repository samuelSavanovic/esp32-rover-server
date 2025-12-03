use dioxus::prelude::*;

#[component]
pub fn ControlButton(label: &'static str, is_active: bool) -> Element {
    let class = if is_active {
        "bg-blue-700 text-white rounded shadow px-4 py-3 text-xl text-center"
    } else {
        "bg-blue-600 text-white rounded shadow px-4 py-3 text-xl text-center active:bg-blue-700"
    };

    rsx! {
        button { class, "{label}" }
    }
}
