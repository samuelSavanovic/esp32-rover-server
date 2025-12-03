use dioxus::prelude::*;
#[component]
pub fn SensorCard(distance: f32) -> Element {
    rsx! {
        div { class: "w-full max-w-md bg-white shadow rounded p-4 mb-6",
            h2 { class: "text-lg font-semibold text-gray-700 mb-2", "Distance Sensor" }
            div { class: "text-4xl font-bold text-blue-600 text-center py-4", "{distance} cm" }
        }
    }
}
