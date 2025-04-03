
#[derive(Debug, Default)]
pub struct UsbPortBuilder {
    serial_number: Option<String>,
    product_id: Option<String>,
    vendor_id: Option<String>,
}

impl UsbPortBuilder {
    pub fn new() -> UsbPortBuilder {
        UsbPortBuilder::default()
    }
    pub fn serial_number(mut self, sn: &str) -> Self {
        self.serial_number = Some(sn.to_string());
        self
    }

    pub fn product_id(mut self, pid: &str) -> Self {
        self.product_id = Some(pid.to_string());
        self
    }

    pub fn vendor_id(mut self, vid: &str) -> Self {
        self.vendor_id = Some(vid.to_string());
        self
    }
}
