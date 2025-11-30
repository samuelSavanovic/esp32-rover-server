mod telemetry;
mod command;
pub use telemetry::Telemetry;
pub use command::Command;

pub const TYPE_TELEMETRY: u8 = 0x01;
pub const TYPE_COMMAND: u8 = 0x02;
