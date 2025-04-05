use std::fmt::{Display, Formatter};
use std::process::Command;

#[derive(Debug, Default)]
pub struct JtagSwdPortBuilder {
    pub frequency: Option<usize>,
    pub index: Option<usize>,
    pub serial_number: Option<String>,
    pub access_port: Option<String>,
    pub mode: Option<JtagSwdMode>,
    pub reset: Option<JtagSwdResetMode>,
    pub shared: bool,
    pub tcp_port: Option<String>,
    pub lpm: Option<JtagSwdLowPower>,
    pub get_auth_id: bool,
    pub speed: Option<String>
}

impl From<JtagSwdPortBuilder> for Command {
    fn from(value: JtagSwdPortBuilder) -> Self {
        let mut cmd = Command::new("");

        if let Some(freq) = value.frequency {
            cmd.arg(format!("freq={freq}"));
        }

        if let Some(idx) = value.index {
            cmd.arg(format!("index={idx}"));
        }

        if let Some(sn) = value.serial_number {
            cmd.arg(format!("sn={sn}"));
        }

        if let Some(ap) = value.access_port {
            cmd.arg(format!("ap={ap}"));
        }

        if let Some(mode) = value.mode {
            cmd.arg(format!("mode={}", mode));
        }

        if let Some(reset) = value.reset {
            cmd.arg(format!("reset={}", reset));
        }

        if value.shared {
            cmd.arg("shared");
        }

        if let Some(port) = value.tcp_port {
            cmd.arg(format!("tcpport={port}"));
        }

        if let Some(lpm) = value.lpm {
            match lpm {
                JtagSwdLowPower::Enabled => {
                    cmd.arg("LPM");
                }
                JtagSwdLowPower::Disabled => {
                    cmd.arg("dLPM");
                }
            }
        }

        if value.get_auth_id {
            cmd.arg("getAuthId");
        }

        if let Some(speed) = value.speed {
            cmd.arg(format!("speed={speed}"));
        }

        cmd
    }
}

#[derive(Debug, Default)]
pub enum JtagSwdMode {
    Ur,
    HotPlug,
    #[default]
    Normal,
    PowerDown,
    HardwareResetPulse
}

impl Display for JtagSwdMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JtagSwdMode::Ur => {"UR".fmt(f)}
            JtagSwdMode::HotPlug => {"HOTPLUG".fmt(f)}
            JtagSwdMode::Normal => {"NORMAL".fmt(f)}
            JtagSwdMode::PowerDown => {"POWERDOWN".fmt(f)}
            JtagSwdMode::HardwareResetPulse => {"HWRSTPULSE".fmt(f)}
        }
    }
}

#[derive(Debug, Default)]
pub enum JtagSwdResetMode {
    #[default]
    SoftwareReset,
    HardwareReset,
    ChipReset
}

impl Display for JtagSwdResetMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JtagSwdResetMode::SoftwareReset => {"SWrst".fmt(f)}
            JtagSwdResetMode::HardwareReset => {"HWrst".fmt(f)}
            JtagSwdResetMode::ChipReset => {"Crst".fmt(f)}
        }
    }
}

#[derive(Debug, Default)]
pub enum JtagSwdLowPower {
    #[default]
    Enabled,
    Disabled
}

#[derive(Debug, Default)]
pub enum JtagSwdSpeed {
    #[default]
    Reliable,
    Fast
}

impl Display for JtagSwdSpeed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JtagSwdSpeed::Reliable => {"Reliable".fmt(f)}
            JtagSwdSpeed::Fast => {"fast".fmt(f)}
        }
    }
}
