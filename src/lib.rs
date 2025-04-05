use std::path::PathBuf;
use std::process::Command;
use crate::port::PortIdentifier;
use crate::stm32wb::Stm32wbCommands;
use crate::verbosity::Verbosity;

pub mod port;
pub mod stm32wb;
pub mod verbosity;

#[derive(Debug)]
pub struct Stm32ProgrammerBuilder {
    path: PathBuf,
    connect: Option<PortIdentifier>,
    verbosity: Option<Verbosity>,
    stm32wb_commands: Option<Stm32wbCommands>
}

impl Default for Stm32ProgrammerBuilder {
    fn default() -> Self {
        Stm32ProgrammerBuilder {
            path: PathBuf::from("C:/Program Files/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin"),
            connect: Default::default(),
            verbosity: Default::default(),
            stm32wb_commands: Default::default()
        }
    }
}

impl From<Stm32ProgrammerBuilder> for Command {
    fn from(value: Stm32ProgrammerBuilder) -> Self {
        let mut cmd = Command::new(value.path);
        if let Some(connect) = value.connect {
            cmd.args(Into::<Command>::into(connect).get_args());
        }

        if let Some(verbosity) = value.verbosity {
            cmd.arg(format!("--verbosity {}", verbosity));
        }

        if let Some(stm32wb_commands) = value.stm32wb_commands {
            cmd.args(Into::<Command>::into(stm32wb_commands).get_args());
        }
        cmd
    }
}
