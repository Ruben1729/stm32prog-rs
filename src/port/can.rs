
#[derive(Default, Debug)]
pub struct CanPortBuilder {
    baudrate: Option<String>,
    mode: Option<String>,
    ide: Option<String>,
    frame_format: Option<String>,
    fifo: Option<String>,
    filter_mode: Option<String>,
    filter_scale: Option<String>,
    filter_activation: Option<String>,
    filter_bank_number: Option<String>,
    no_init: bool,
}