use dioxus::prelude::*;

use crate::{components::ControlButton, ws::DashboardCommand};

#[derive(Default, Clone, Copy, PartialEq)]
struct KeyState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl KeyState {
    fn to_command(self) -> DashboardCommand {
        let (forward, backward) = (self.up, self.down);
        let (left, right) = (self.left, self.right);

        let (left_pwm, right_pwm) = match (forward, backward, left, right) {
            // Forward combinations
            (true, false, false, false) => (-255, -255),  // Forward
            (true, false, true, false)  => (-255, -128),  // Forward-left (right motor slower)
            (true, false, false, true)  => (-128, -255),  // Forward-right (left motor slower)
            // Backward combinations
            (false, true, false, false) => (255, 255),    // Backward
            (false, true, true, false)  => (255, 128),    // Backward-left (right motor slower)
            (false, true, false, true)  => (128, 255),    // Backward-right (left motor slower)
            // Pure rotation (pivot turns)
            (false, false, true, false) => (255, 0),      // Pivot left (left motor backward)
            (false, false, false, true) => (0, 255),      // Pivot right (right motor backward)
            // Stop (including conflicting inputs)
            _ => (0, 0),
        };
        DashboardCommand::new(left_pwm, right_pwm)
    }
}

#[component]
pub fn DriveControls(on_command: EventHandler<DashboardCommand>) -> Element {
    let mut keys = use_signal(KeyState::default);

    let handle_keydown = {
        move |ev: KeyboardEvent| {
            let mut k = *keys.read();
            match ev.key() {
                Key::ArrowUp => k.up = true,
                Key::ArrowDown => k.down = true,
                Key::ArrowLeft => k.left = true,
                Key::ArrowRight => k.right = true,
                Key::Character(ref s) if s == "w" || s == "W" => k.up = true,
                Key::Character(ref s) if s == "s" || s == "S" => k.down = true,
                Key::Character(ref s) if s == "a" || s == "A" => k.left = true,
                Key::Character(ref s) if s == "d" || s == "D" => k.right = true,
                _ => return,
            }
            keys.set(k);
            on_command(k.to_command());
        }
    };

    let handle_keyup = {
        move |ev: KeyboardEvent| {
            let mut k = *keys.read();
            match ev.key() {
                Key::ArrowUp => k.up = false,
                Key::ArrowDown => k.down = false,
                Key::ArrowLeft => k.left = false,
                Key::ArrowRight => k.right = false,
                Key::Character(ref s) if s == "w" || s == "W" => k.up = false,
                Key::Character(ref s) if s == "s" || s == "S" => k.down = false,
                Key::Character(ref s) if s == "a" || s == "A" => k.left = false,
                Key::Character(ref s) if s == "d" || s == "D" => k.right = false,
                _ => return,
            }
            keys.set(k);
            on_command(k.to_command());
        }
    };
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
                is_active: keys.read().up,
            }
            div {}

            ControlButton {
                label: "←",
                is_active: keys.read().left,
            }
            ControlButton { label: "•", is_active: !keys.read().up && !keys.read().down && !keys.read().left && !keys.read().right }
            ControlButton {
                label: "→",
                is_active: keys.read().right,
            }

            div {}
            ControlButton {
                label: "↓",
                is_active: keys.read().down,
            }
            div {}
        }
    }
}
