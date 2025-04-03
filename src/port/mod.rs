use self::can::CanPortBuilder;
use self::i2c::I2cPortBuilder;
use self::jtag_swd::JtagSwdPortBuilder;
use self::spi::SpiPortBuilder;
use self::uart::UartPortBuilder;
use self::usb::UsbPortBuilder;
use std::process::Command;

pub mod can;
pub mod i2c;
pub mod jtag_swd;
pub mod spi;
pub mod uart;
pub mod usb;

#[derive(Debug, Default)]
pub enum PortIdentifier {
    PortName(String),
    Can(CanPortBuilder),
    I2c(I2cPortBuilder),
    Jtag(JtagSwdPortBuilder),
    #[default]
    Swd(JtagSwdPortBuilder),
    Spi(SpiPortBuilder),
    Uart(UartPortBuilder),
    Usb(UsbPortBuilder),
}

impl From<PortIdentifier> for Command {
    fn from(value: PortIdentifier) -> Self {
        let mut cmd = Command::new("");

        match value {
            PortIdentifier::PortName(name) => {
                cmd.arg(format!("port={name}"));
            }
            PortIdentifier::Can(_) => {}
            PortIdentifier::I2c(_) => {}
            PortIdentifier::Jtag(_) => {}
            PortIdentifier::Swd(_) => {}
            PortIdentifier::Spi(_) => {}
            PortIdentifier::Uart(_) => {}
            PortIdentifier::Usb(_) => {}
        }

        cmd
    }
}
