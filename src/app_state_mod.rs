// dropbox_backup_to_external_disk_cli/src/app_state_mod.rs

// use exported code from the lib project
use dropbox_backup_to_external_disk_lib as lib;

/// AppState struct contains only private fields.
/// Those will be used as global mutable, but only using methods from AppStateTrait.
#[derive(Debug)]
struct AppState {
    app_config: lib::AppConfig,
}

/// implementation of AppStateTrait functions that is defined in the lib project
/// and will be used in the lib project, because I want the lib project to have no idea where the tokens are stored.
impl lib::AppStateTrait for AppState {
    fn load_keys_from_io(&self) -> Result<(String, String), lib::LibError> {
        let master_key = std::env::var("DBX_KEY_1")?;
        let token_enc = std::env::var("DBX_KEY_2")?;
        dbg!(&master_key);
        Ok((master_key, token_enc))
    }
    fn ref_app_config(&self) -> &lib::AppConfig {
        &self.app_config
    }
}

/// init the global struct APP_STATE defined in the lib project
pub fn init_app_state() {
    // define paths in bin, not in lib
    let app_config = lib::AppConfig {
        path_list_ext_disk_base_path: "temp_data/list_base_local_path.csv",
        path_list_source_files: "temp_data/list_source_files.csv",
        path_list_destination_files: "temp_data/list_destination_files.csv",
        path_list_source_folders: "temp_data/list_source_folders.csv",
        path_list_destination_folders: "temp_data/list_destination_folders.csv",
        path_list_destination_readonly_files: "temp_data/list_destination_readonly_files.csv",
        path_list_for_download: "temp_data/list_for_download.csv",
        path_list_for_trash: "temp_data/list_for_trash.csv",
        path_list_for_correct_time: "temp_data/list_for_correct_time.csv",
        path_list_just_downloaded_or_moved: "temp_data/list_just_downloaded_or_moved.csv",
        path_list_for_trash_folders: "temp_data/list_for_trash_folders.csv",
        path_list_for_create_folders: "temp_data/list_for_create_folders.csv",
    };
    let _ = lib::APP_STATE.set(std::sync::Mutex::new(Box::new(AppState { app_config })));
}
