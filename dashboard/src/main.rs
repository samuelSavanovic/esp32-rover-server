mod ws;
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

            DriveControls {}
            // --- Camera placeholder ---
            div { class: "w-full max-w-2xl bg-gray-200 shadow-inner rounded h-64 flex items-center justify-center text-gray-500",
                "Camera feed coming soon…"
            }
        }
    }
}

#[component]
fn SensorCard(distance: f32) -> Element {
    rsx! {
        div { class: "w-full max-w-md bg-white shadow rounded p-4 mb-6",
            h2 { class: "text-lg font-semibold text-gray-700 mb-2", "Distance Sensor" }
            div { class: "text-4xl font-bold text-blue-600 text-center py-4", "{distance} cm" }
        }
    }
}

#[component]
fn DriveControls() -> Element {
    let mut active = use_signal(|| None);

    let handle_keydown = {
        move |ev: KeyboardEvent| {
            let dir = match ev.key() {
                Key::ArrowUp => Some(Direction::Up),
                Key::ArrowDown => Some(Direction::Down),
                Key::ArrowLeft => Some(Direction::Left),
                Key::ArrowRight => Some(Direction::Right),

                Key::Character(s) if s == "w" || s == "W" => Some(Direction::Up),
                Key::Character(s) if s == "s" || s == "S" => Some(Direction::Down),
                Key::Character(s) if s == "a" || s == "A" => Some(Direction::Left),
                Key::Character(s) if s == "d" || s == "D" => Some(Direction::Right),
                _ => None,
            };
            active.set(dir);
        }
    };

    let handle_keyup = { move |_: KeyboardEvent| active.set(None) };
    rsx! {
        div {
            class: "grid grid-cols-3 gap-4 w-full max-w-sm mb-10",
            tabindex: 0,

            onkeydown: handle_keydown,
            onkeyup: handle_keyup,
            autofocus: true,

            div {}
            ControlButton {
                label: "↑",
                is_active: *active.read() == Some(Direction::Up),
            }
            div {}

            ControlButton {
                label: "←",
                is_active: *active.read() == Some(Direction::Left),
            }
            ControlButton { label: "•", is_active: *active.read() == None }
            ControlButton {
                label: "→",
                is_active: *active.read() == Some(Direction::Right),
            }

            div {}
            ControlButton {
                label: "↓",
                is_active: *active.read() == Some(Direction::Down),
            }
            div {}
        }
    }
}

#[component]
fn ControlButton(label: &'static str, is_active: bool) -> Element {
    let class = if is_active {
        "bg-blue-700 text-white rounded shadow px-4 py-3 text-xl text-center"
    } else {
        "bg-blue-600 text-white rounded shadow px-4 py-3 text-xl text-center active:bg-blue-700"
    };

    rsx! {
        button { class, "{label}" }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
