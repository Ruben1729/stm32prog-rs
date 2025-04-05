use std::io;
use std::path::PathBuf;
use std::process::{Child, Command, Output};
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

impl Stm32ProgrammerBuilder {
    pub fn connect(mut self, port: PortIdentifier) -> Self {
        self.connect = Some(port);
        self
    }

    pub fn verbosity(mut self, verbosity: Verbosity) -> Self {
        self.verbosity = Some(verbosity);
        self
    }

    pub fn stm32wb_commands(mut self, commands: Stm32wbCommands) -> Self {
        self.stm32wb_commands = Some(commands);
        self
    }

    pub fn spawn(self) -> io::Result<Child>  {
        let mut cmd: Command = self.into();
        cmd.spawn()
    }

    pub fn output(self) -> io::Result<Output>{
        let mut cmd: Command = self.into();
        cmd.output()
    }
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
