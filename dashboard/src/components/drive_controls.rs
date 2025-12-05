use dioxus::prelude::*;

use crate::{components::ControlButton, ws::DashboardCommand};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Forward"),
            Direction::Down => write!(f, "Backward"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
            Direction::Stop => write!(f, "Stop"),
        }
    }
}

impl Into<DashboardCommand> for Direction {
    fn into(self) -> DashboardCommand {
        match self {
            Direction::Up => DashboardCommand::new(255, 255),
            Direction::Down => DashboardCommand::new(-255, -255),
            Direction::Left => DashboardCommand::new(0, 255),
            Direction::Right => DashboardCommand::new(255, 0),
            Direction::Stop => DashboardCommand::new(0, 0),
        }
    }
}

#[component]
pub fn DriveControls(on_command: EventHandler<DashboardCommand>) -> Element {
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
            if let Some(d) = dir {
                on_command(d.into());
            }
        }
    };

    let handle_keyup = {
        move |_: KeyboardEvent| {
            active.set(None);
            on_command(Direction::Stop.into());
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
