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

#[derive(Debug)]
pub enum PortIdentifier {
    PortName(String),
    Can(CanPortBuilder),
    I2c(I2cPortBuilder),
    Jtag(JtagSwdPortBuilder),
    Swd(JtagSwdPortBuilder),
    Spi(SpiPortBuilder),
    Uart(UartPortBuilder),
    Usb(UsbPortBuilder),
}

impl Default for PortIdentifier {
    fn default() -> PortIdentifier {
        PortIdentifier::Swd(JtagSwdPortBuilder::default())
    }
}

impl From<PortIdentifier> for Command {
    fn from(value: PortIdentifier) -> Self {
        let mut cmd = Command::new("");

        match value {
            PortIdentifier::PortName(name) => {
                cmd.arg(format!("port={name}"));
            }
            PortIdentifier::Can(port_builder) => {
                let port_cmd: Command = port_builder.into();
                cmd.args(port_cmd.get_args());
            }
            PortIdentifier::I2c(port_builder) => {
                let port_cmd: Command = port_builder.into();
                cmd.args(port_cmd.get_args());
            }
            PortIdentifier::Jtag(port_builder)|
            PortIdentifier::Swd(port_builder) => {
                let port_cmd: Command = port_builder.into();
                cmd.args(port_cmd.get_args());
            }
            PortIdentifier::Spi(port_builder) => {
                let port_cmd: Command = port_builder.into();
                cmd.args(port_cmd.get_args());
            }
            PortIdentifier::Uart(port_builder) => {
                let port_cmd: Command = port_builder.into();
                cmd.args(port_cmd.get_args());
            }
            PortIdentifier::Usb(port_builder) => {
                let port_cmd: Command = port_builder.into();
                cmd.args(port_cmd.get_args());
            }
        }

        cmd
    }
}
