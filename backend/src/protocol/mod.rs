mod command;
mod dashboard_telemetry;
mod telemetry;
pub use command::Command;
pub use dashboard_telemetry::DashboardTelemetry;
pub use telemetry::Telemetry;

pub const TYPE_TELEMETRY: u8 = 0x01;
pub const TYPE_COMMAND: u8 = 0x02;
