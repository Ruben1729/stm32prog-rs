use std::process::Command;

#[derive(Debug, Default)]
pub struct UartPortBuilder {
    baudrate: Option<String>,
    parity: Option<String>,
    data_bits: Option<String>,
    stop_bits: Option<String>,
    flow_control: Option<String>,
    rts: Option<String>,
    dtr: Option<String>,
    no_init: Option<String>,
    console: Option<String>,
}

impl From<UartPortBuilder> for Command {
    fn from(_value: UartPortBuilder) -> Self {
        Command::new("")
    }
}
