use std::fmt::{Display, Formatter};
use std::process::Command;

#[derive(Debug, Default)]
pub struct Stm32wbCommands {
    get_uid_64: bool,
    fus_get_state: bool,
    fus_op_get_version: bool,
    anti_rollback: bool,
    start_fus: bool,
    fw_delete: bool,
    fw_upgrade: Option<FwUpgrade>,
    start_wireless_stack: bool,
    auth_key_update: Option<String>,
    auth_key_lock: bool,
    wusr_key: Option<WusrKey>
}

impl From<Stm32wbCommands> for Command {
    fn from(value: Stm32wbCommands) -> Self {
        let mut cmd = Command::new("");

        if value.get_uid_64 {
            cmd.arg("-getuid64");
        }

        if value.fus_get_state {
            cmd.arg("-fusgetstate");
        }

        if value.fus_op_get_version {
            cmd.arg("-fusopgetversion");
        }

        if value.anti_rollback {
            cmd.arg("-antirollback");
        }

        if value.start_fus {
            cmd.arg("-startfus");
        }

        if value.fw_delete {
            cmd.arg("-fwdelete");
        }

        if let Some(fw_upgrade) = value.fw_upgrade {
            cmd.args(Into::<Command>::into(fw_upgrade).get_args());
        }

        if value.start_wireless_stack {
            cmd.arg("-startwirelessstack");
        }

        if let Some(auth_key_update) = value.auth_key_update {
            cmd.arg("-authkeyupdate");
            cmd.arg(auth_key_update);
        }

        if value.auth_key_lock {
            cmd.arg("-authkeylock");
        }

        if let Some(wusr_key) = value.wusr_key {
            cmd.args(Into::<Command>::into(wusr_key).get_args());
        }

        cmd
    }
}

#[derive(Debug, Default)]
pub struct FwUpgrade {
    file_path: String,
    address: String,
    first_install: Option<bool>,
    start_stack: Option<bool>,
    verify: bool
}

impl From<FwUpgrade> for Command {
    fn from(value: FwUpgrade) -> Self {
        let mut cmd = Command::new("");

        cmd.arg(format!("-fwupgrade {} {}", value.file_path, value.address));

        if let Some(first_install) = value.first_install {
            if first_install {
                cmd.arg("firstinstall=1");
            } else {
                cmd.arg("firstinstall=0");
            }
        }

        if let Some(start_stack) = value.start_stack {
            if start_stack {
                cmd.arg("start_stack=1");
            } else {
                cmd.arg("start_stack=0");
            }
        }

        if value.verify {
            cmd.arg("-v");
        }

        cmd
    }
}

#[derive(Debug, Default)]
pub struct WusrKey {
    file_path: String,
    key_type: WusrKeyType,
}

impl From<WusrKey> for Command {
    fn from(value: WusrKey) -> Self {
        let mut cmd = Command::new("");

        cmd.args(&["-wusrkey", &value.file_path, &format!("{}", value.key_type)]);

        cmd
    }
}

#[derive(Debug, Default)]
pub enum WusrKeyType {
    #[default]
    SimpleKey,
    MasterKey,
    EncryptedKey
}

impl Display for WusrKeyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WusrKeyType::SimpleKey => {"1".fmt(f) }
            WusrKeyType::MasterKey => {"2".fmt(f) }
            WusrKeyType::EncryptedKey => {"3".fmt(f) }
        }
    }
}
