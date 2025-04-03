use crate::port::PortIdentifier;
use crate::stm32wb::Stm32wbCommands;
use crate::verbosity::Verbosity;

pub mod port;
pub mod stm32wb;
pub mod verbosity;

#[derive(Debug, Default)]
pub struct Stm32ProgrammerBuilder {
    connect: Option<PortIdentifier>,
    verbosity: Option<Verbosity>,
    stm32wb_commands: Option<Stm32wbCommands>
}
