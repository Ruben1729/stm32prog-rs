
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
    wusr_key_type: Option<WusrKeyType>
}

#[derive(Debug, Default)]
pub struct FwUpgrade {
    file_path: String,
    address: String,
    first_install: bool,
    start_stack: bool
}

pub struct WusrKey {
    file_path: String,
    key_type: WusrKeyType,
}

#[derive(Debug, Default)]
pub enum WusrKeyType {
    #[default]
    SimpleKey,
    MasterKey,
    EncryptedKey
}
