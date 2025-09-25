// dropbox_backup_to_external_disk_cli/src/app_state_mod.rs

use crossplatform_path::CrossPathBuf;
use std::sync::MutexGuard;

// use exported code from the lib project
use dropbox_backup_to_external_disk_lib as lib;
use dropbox_backup_to_external_disk_lib::DropboxBackupToExternalDiskError;

/// AppState is used as a global variable/struct.
/// AppState struct contains only private fields. Some are immutable and other are mutable behind a Mutex.
/// The struct must be declared inside the bin project. Because only that way I can add the AppStateMethods from the LIB project. (Rust Orphan rule)
/// These methods from AppStateMethods are "dependency injection" or "inversion of control". Inside the LIB project these methods are declared and used, but there is no code.
/// The code is inside the bin project, because different bin projects can bring different methods implementations.
#[derive(Debug)]
struct AppState {
    // immutable
    app_config: lib::AppConfig,
    // mutable with Mutex
    string_proba_mutex: std::sync::Mutex<String>,
}

/// implementation of AppStateMethods functions that is defined in the lib project
/// and will be used in the lib project, because I want the lib project to have no idea where the tokens are stored.
impl lib::AppStateMethods for AppState {
    fn load_keys_from_io(&self) -> Result<(String, String), DropboxBackupToExternalDiskError> {
        let master_key = std::env::var("DBX_KEY_1")?;
        let token_enc = std::env::var("DBX_KEY_2")?;
        Ok((master_key, token_enc))
    }
    fn ref_app_config(&self) -> &lib::AppConfig {
        &self.app_config
    }
    fn lock_proba(&'_ self) -> MutexGuard<'_, String> {
        self.string_proba_mutex.lock().unwrap()
    }
}

/// init the global struct APP_STATE defined in the lib project
pub fn init_app_state() {
    // define paths in bin, not in lib
    let app_config = lib::AppConfig {
        path_list_ext_disk_base_path: CrossPathBuf::new("tmp/temp_data/list_base_local_path.csv").expect("It is ok."),
        path_list_source_files: CrossPathBuf::new("tmp/temp_data/list_source_files.csv").expect("It is ok."),
        path_list_destination_files: CrossPathBuf::new("tmp/temp_data/list_destination_files.csv").expect("It is ok."),
        path_list_source_folders: CrossPathBuf::new("tmp/temp_data/list_source_folders.csv").expect("It is ok."),
        path_list_destination_folders: CrossPathBuf::new("tmp/temp_data/list_destination_folders.csv").expect("It is ok."),
        path_list_destination_readonly_files: CrossPathBuf::new("tmp/temp_data/list_destination_readonly_files.csv").expect("It is ok."),
        path_list_for_download: CrossPathBuf::new("tmp/temp_data/list_for_download.csv").expect("It is ok."),
        path_list_for_trash_files: CrossPathBuf::new("tmp/temp_data/list_for_trash_files.csv").expect("It is ok."),
        path_list_just_downloaded: CrossPathBuf::new("tmp/temp_data/list_just_downloaded.csv").expect("It is ok."),
        path_list_for_trash_folders: CrossPathBuf::new("tmp/temp_data/list_for_trash_folders.csv").expect("It is ok."),
        path_list_for_create_folders: CrossPathBuf::new("tmp/temp_data/list_for_create_folders.csv").expect("It is ok."),
    };
    let string_proba_mutex = std::sync::Mutex::new(String::from("proba"));
    let _ = lib::APP_STATE.set(Box::new(AppState {
        app_config,
        string_proba_mutex,
    }));
}
