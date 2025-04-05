use std::process::Command;

#[derive(Debug, Default)]
pub struct SpiPortBuilder {
    baudrate: Option<String>,
    cpha: Option<String>,
    cpol: Option<String>,
    crc: bool,
    crc_polynomial: Option<String>,
    data_size: Option<String>,
    direction: Option<String>,
    first_bit: Option<String>,
    frame_format: Option<String>,
    mode: Option<String>,
    nss: Option<String>,
    nss_pulse: Option<String>,
    delay: Option<String>,
    no_init: bool,
}

impl From<SpiPortBuilder> for Command {
    fn from(_value: SpiPortBuilder) -> Self {
        Command::new("")
    }
}
