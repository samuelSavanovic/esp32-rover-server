mod components;
mod ws;
use components::*;
use dioxus::prelude::*;

use crate::ws::use_car_ws;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        Home {}
    }
}

#[component]
fn Home() -> Element {
    let car = use_car_ws("ws://127.0.0.1:9000/dashboard-ws");
    rsx! {
        div { class: "min-h-screen bg-gray-100 flex flex-col items-center p-6",

            h1 { class: "text-3xl font-bold text-gray-800 mb-6", "RC Rover Dashboard" }

            SensorCard { distance: car.distance.read().unwrap_or(0.0)}

            DriveControls {on_command: car.send_command}
            // --- Camera placeholder ---
            div { class: "w-full max-w-2xl bg-gray-200 shadow-inner rounded h-64 flex items-center justify-center text-gray-500",
                "Camera feed coming soon…"
            }
        }
    }
}
