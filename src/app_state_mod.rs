// dropbox_backup_to_external_disk_cli/src/app_state_mod.rs

// use exported code from the lib project
use dropbox_backup_to_external_disk_lib as lib;

/// AppState struct contains only private fields.
/// Those will be used as global mutable, but only using methods from AppStateTrait.
#[derive(Debug)]
struct AppState {
    string_x: String,
}

/// implementation of AppStateTrait that is defined in the lib project
impl lib::AppStateTrait for AppState {
    fn load_keys_from_io(&self) -> Result<(String, String), lib::LibError> {
        let master_key = std::env::var("DBX_KEY_1")?;
        let token_enc = std::env::var("DBX_KEY_2")?;
        dbg!(&master_key);
        Ok((master_key, token_enc))
    }
    fn get_first_field(&self) -> String {
        self.string_x.to_string()
    }
    fn set_first_field(&mut self, value: String) {
        self.string_x = value;
    }
}

/// init the global struct APP_STATE defined in the lib project
pub fn init_app_state() {
    let _ = lib::APP_STATE.set(std::sync::Mutex::new(Box::new(AppState { string_x: String::from("") })));
}
