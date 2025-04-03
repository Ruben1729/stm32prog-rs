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
    pub lpm: JtagSwdLowPower,
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
            cmd.arg(format!("mode={}", mode.to_cli_string()));
        }

        if let Some(reset) = value.reset {
            cmd.arg(format!("reset={}", reset.to_cli_string()));
        }

        if value.shared {
            cmd.arg("shared=true");
        }

        if let Some(port) = value.tcp_port {
            cmd.arg(format!("tcpport={port}"));
        }

        cmd.arg(format!("lpm={}", value.lpm.to_cli_string()));

        if value.get_auth_id {
            cmd.arg("getauthid=true");
        }

        if let Some(speed) = value.speed {
            cmd.arg(format!("speed={speed}"));
        }

        cmd
    }
}

impl From<Command> for JtagSwdPortBuilder {
    fn from(cmd: Command) -> Self {
        let mut builder = JtagSwdPortBuilder::default();

        // Collect args from command
        let args: Vec<String> = cmd.get_args()
            .map(|arg| arg.to_string_lossy().to_string())
            .collect();

        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                match key.to_lowercase().as_str() {
                    "freq" | "frequency" => {
                        if let Ok(freq) = value.parse::<usize>() {
                            builder.frequency = Some(freq);
                        }
                    },
                    "index" => {
                        if let Ok(idx) = value.parse::<usize>() {
                            builder.index = Some(idx);
                        }
                    },
                    "sn" => {
                        builder.serial_number = Some(value.to_string());
                    },
                    "ap" => {
                        builder.access_port = Some(value.to_string());
                    },
                    "mode" => {
                        builder.mode = Some(JtagSwdMode::from_cli_string(value));
                    },
                    "reset" => {
                        builder.reset = Some(JtagSwdResetMode::from_cli_string(value));
                    },
                    "shared" => {
                        builder.shared = value.to_lowercase() == "true";
                    },
                    "tcpport" => {
                        builder.tcp_port = Some(value.to_string());
                    },
                    "lpm" => {
                        builder.lpm = JtagSwdLowPower::from_cli_string(value);
                    },
                    "getauthid" => {
                        builder.get_auth_id = value.to_lowercase() == "true";
                    },
                    "speed" => {
                        builder.speed = Some(value.to_string());
                    },
                    _ => {}
                }
            }
        }

        builder
    }
}

#[derive(Debug, Default)]
pub enum JtagSwdMode {
    Ur,
    HotPlug,
    #[default]
    Normal,
    PowerDown
}

impl JtagSwdMode {
    pub fn to_cli_string(&self) -> String {
        match self {
            JtagSwdMode::Ur => "UR".to_string(),
            JtagSwdMode::HotPlug => "HotPlug".to_string(),
            JtagSwdMode::Normal => "Normal".to_string(),
            JtagSwdMode::PowerDown => "PowerDown".to_string(),
        }
    }

    pub fn from_cli_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ur" => JtagSwdMode::Ur,
            "hotplug" => JtagSwdMode::HotPlug,
            "powerdown" => JtagSwdMode::PowerDown,
            _ => JtagSwdMode::Normal,
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

impl JtagSwdResetMode {
    pub fn to_cli_string(&self) -> String {
        match self {
            JtagSwdResetMode::SoftwareReset => "Software".to_string(),
            JtagSwdResetMode::HardwareReset => "Hardware".to_string(),
            JtagSwdResetMode::ChipReset => "Chip".to_string(),
        }
    }

    pub fn from_cli_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "hardware" => JtagSwdResetMode::HardwareReset,
            "chip" => JtagSwdResetMode::ChipReset,
            _ => JtagSwdResetMode::SoftwareReset,
        }
    }
}

#[derive(Debug, Default)]
pub enum JtagSwdLowPower {
    #[default]
    Enabled,
    Disabled
}

impl JtagSwdLowPower {
    pub fn to_cli_string(&self) -> String {
        match self {
            JtagSwdLowPower::Enabled => "true".to_string(),
            JtagSwdLowPower::Disabled => "false".to_string(),
        }
    }

    pub fn from_cli_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "false" | "0" | "no" | "disabled" => JtagSwdLowPower::Disabled,
            _ => JtagSwdLowPower::Enabled,
        }
    }
}

#[derive(Debug, Default)]
pub enum JtagSwdSpeed {
    #[default]
    Reliable,
    Fast
}

impl JtagSwdSpeed {
    pub fn to_cli_string(&self) -> String {
        match self {
            JtagSwdSpeed::Reliable => "Reliable".to_string(),
            JtagSwdSpeed::Fast => "Fast".to_string(),
        }
    }

    pub fn from_cli_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "fast" => JtagSwdSpeed::Fast,
            _ => JtagSwdSpeed::Reliable,
        }
    }
}
