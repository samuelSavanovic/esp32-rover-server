use serde::Serialize;

use crate::protocol::{TYPE_TELEMETRY, Telemetry};

#[derive(Serialize)]
pub struct DashboardTelemetry {
    pub kind: u8,
    pub distance_mm: u32,
}

impl From<&Telemetry> for DashboardTelemetry {
    fn from(t: &Telemetry) -> Self {
        Self {
            kind: TYPE_TELEMETRY,
            distance_mm: t.distance_mm,
        }
    }
}

impl From<Telemetry> for DashboardTelemetry {
    fn from(t: Telemetry) -> Self {
        DashboardTelemetry::from(&t)
    }
}
