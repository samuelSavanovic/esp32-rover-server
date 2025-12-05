use serde::Deserialize;

#[derive(Deserialize)]
pub struct DashboardCommand {
    pub kind: u8,
    pub left_pwm: i16,
    pub right_pwm: i16,
}
