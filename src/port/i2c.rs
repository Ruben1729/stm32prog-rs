use std::process::Command;

#[derive(Debug, Default)]
pub struct I2cPortBuilder {
    address: String,
    baudrate: Option<String>,
    speed_mode: Option<String>,
    address_mode: Option<String>,
    analog_filter: Option<String>,
    digital_filter: Option<String>,
    digital_noise_filter: Option<String>,
    rise_time: Option<String>,
    fall_time: Option<String>,
    no_init: bool
}

impl From<I2cPortBuilder> for Command {
    fn from(_value: I2cPortBuilder) -> Self {
        Command::new("")
    }
}
