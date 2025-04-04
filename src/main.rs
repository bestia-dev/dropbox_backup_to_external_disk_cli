// dropbox_backup_to_external_disk_cli/src/bin.rs
// CLI binary project around the library.
// All work with input/output should be inside the bin project, and nothing in the lib project.
// Inside bin I should print on the screen and open or create Files. Then pass the Files to the lib part to operate on them.
// But to be interactive I cannot wait for a lib function to finish. The lib functions should be in another thread.
// Then send msg to the bin main thread that print that to the screen.

#![doc=include_str!("../README.md")]

mod app_state_mod;
mod crossterm_cli_mod;

use std::path::{Path, PathBuf};

use crossterm_cli_mod::*;

// use exported code from the lib project
use dropbox_backup_to_external_disk_lib as lib;
use dropbox_backup_to_external_disk_lib::{global_config, LibError};
use lib::FileTxt;

fn main() {
    /*     ctrlc::set_handler(move || {
        println!("terminated with ctrl+c. {}", *UNHIDE_CURSOR);
        std::process::exit(exitcode::OK);
    })
    .expect("Bug: setting Ctrl-C handler"); */
    pretty_env_logger::init();
    // catch propagated errors and communicate errors to user or developer
    match main_with_catch_errors() {
        Ok(()) => (),
        Err(err) => println!("{RED}{err}{RESET}"),
    }
}

fn main_with_catch_errors() -> Result<(), LibError> {
    // init the global struct APP_STATE defined in the lib project
    app_state_mod::init_app_state();

    //create the directory tmp/temp_data/
    std::fs::create_dir_all("tmp/temp_data")?;

    /*   let ext_disk_base_path = if std::path::Path::new(APP_CONFIG.path_list_ext_disk_base_path).exists() {
        std::fs::read_to_string(APP_CONFIG.path_list_ext_disk_base_path)?
    } else {
        String::new()
    }; */

    // look at the arguments and route to appropriate function
    // and catch errors and communicate errors to user or developer
    argument_router()?;
    Ok(())
}
/// look at the arguments and route to appropriate function
fn argument_router() -> Result<(), LibError> {
    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("completion") => completion(),
        Some("encode_token") => ui_encode_token(),
        Some("test") => ui_test_connection(),
        /*
        Some("list_and_sync") => match env::args().nth(2).as_deref() {
            Some(path) => {
                let ns_started = ns_start("list_and_sync");
                print!("{}", *CLEAR_ALL);
                list_and_sync(path, &APP_CONFIG);
                ns_print_ms("list_and_sync", ns_started);
            }
            _ => println!("Unrecognized arguments. Try dropbox_backup_to_external_disk_cli --help"),
        },
        Some("sync_only") => {
            let ns_started = ns_start("sync_only");
            print!("{}", *CLEAR_ALL);
            sync_only(&APP_CONFIG);
            ns_print_ms("sync_only", ns_started);
        }
        */
        Some("remote_list") => remote_list(),
        Some("local_list") => {
            // the command local_list must have 1 argument: the path to local external disk folder
            match std::env::args().nth(2).as_deref() {
                Some(ext_disk_base_path) => local_list(Path::new(ext_disk_base_path)),
                None => Err(LibError::ErrorFromString(format!("{RED}Missing arguments. Try `dropbox_backup_to_external_disk_cli --help`{RESET}"))),
            }
        }
        Some("all_list") => {
            // the command all_list must have 1 argument: the path to local external disk folder
            match std::env::args().nth(2).as_deref() {
                Some(ext_disk_base_path) => all_list(Path::new(ext_disk_base_path)),
                None => Err(LibError::ErrorFromString(format!("{RED}Missing arguments. Try `dropbox_backup_to_external_disk_cli --help`{RESET}"))),
            }
        }
        Some("read_only_remove") => read_only_remove(),
        Some("compare_files") => compare_files(),
        Some("compare_folders") => compare_folders(),
        Some("create_folders") => create_folders(),
        Some("move_or_rename_local_files") => move_or_rename_local_files(),
        Some("trash_files") => trash_files(),
        Some("trash_folders") => trash_folders(),
        Some("one_file_download") => match std::env::args().nth(2).as_deref() {
            Some(path_str) => download_one_file(path_str),
            None => Err(LibError::ErrorFromString(format!("{RED}Missing arguments. Try `dropbox_backup_to_external_disk_cli --help`{RESET}"))),
        },
        Some("download_from_list") => download_from_list(),
        _ => Err(LibError::ErrorFromStr("Unrecognized command line arguments. Try `dropbox_backup_to_external_disk_cli --help`")),
    }
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
/// `complete -C "dropbox_backup_to_external_disk_cli completion" dropbox_backup_to_external_disk_cli`
/// `complete -p`  - shows all the completion commands
/// `complete -r xxx` - deletes a completion command
fn completion() -> Result<(), LibError> {
    // println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if !sub_found {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    // `complete -C "dropbox_backup_to_external_disk_cli completion" dropbox_backup_to_external_disk_cli`
    // this completion always sends this arguments:
    // 0. executable path
    // 1. word completion
    // 2. executable file name
    // 3. word_being_completed (even if it is empty)
    // 4. last_word
    let word_being_completed = args[3].as_str();
    let last_word = args[4].as_str();

    if last_word.ends_with("dropbox_backup_to_external_disk_cli") {
        let sub_commands = vec![
            "--help",
            "-h",
            "all_list",
            "compare_files",
            "compare_folders",
            "create_folders",
            "read_only_remove",
            "download_from_list",
            "list_and_sync",
            "local_list",
            "move_or_rename_local_files",
            "one_file_download",
            "remote_list",
            "second_backup",
            "encode_token",
            "sync_only",
            "test",
            "trash_folders",
            "trash_files",
        ];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    // the second level if needed
    else if last_word == "list_and_sync" || last_word == "local_list" || last_word == "all_list" {
        let sub_commands = vec!["/mnt/e/DropboxBackup2"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    Ok(())
}

/// print help
fn print_help() -> Result<(), LibError> {
    let date = chrono::offset::Utc::now().format("%Y%m%dT%H%M%SZ");
    // app_config variable will lock the mutex until it is in scope. For this short function this is ok.
    let path_list_source_files = global_config().path_list_source_files.to_string_lossy();
    let path_list_destination_files = global_config().path_list_destination_files.to_string_lossy();
    let path_list_for_download = global_config().path_list_for_download.to_string_lossy();
    let path_list_for_trash_files = global_config().path_list_for_trash_files.to_string_lossy();
    let path_list_for_readonly = global_config().path_list_destination_readonly_files.to_string_lossy();
    let path_list_for_trash_folders = global_config().path_list_destination_folders.to_string_lossy();
    let path_list_for_create_folders = global_config().path_list_for_create_folders.to_string_lossy();

    println!(
        r#"
  {YELLOW}{BOLD}Welcome to dropbox_backup_to_external_disk_cli{RESET}

  {YELLOW}1. Before first use, create your private Dropbox app:{RESET}
  - Open browser on {GREEN}<https://www.dropbox.com/developers/apps?_tk=pilot_lp&_ad=topbar4&_camp=myapps>{RESET}
  - Click Create app, choose Scoped access, choose Full dropbox
  - Choose a globally unique app name like {GREEN}`backup_{date}`{RESET}
  - Go to tab Permissions, check `files.metadata.read` and `files.content.read`, click Submit, close browser

  {YELLOW}2. Before every use, create a short-lived access token (secret):{RESET}
  - In you Linux terminal session run:
{GREEN}eval $(dropbox_backup_to_external_disk_cli encode_token){RESET}
  - Let it wait for your input
  - Open browser on {GREEN}<https://www.dropbox.com/developers/apps?_tk=pilot_lp&_ad=topbar4&_camp=myapps>{RESET}
  - Choose your existing private Dropbox app like {GREEN}`backup_{date}`{RESET}
  - Click button `Generate` to generated short-lived access token and copy it, close browser
  - Return to the terminal, that is waiting for your input
  - Paste the copied short-lived token with shift+ctr+v and press Enter.  
  - The token is saved in env var and will be used in subsequent commands.
  - This temporary token will be deleted when the session ends.
  - Test if the authentication works:
{GREEN}dropbox_backup_to_external_disk_cli test{RESET}

  {YELLOW}Commands:{RESET}
  Full list and sync - from dropbox to external disk
  This command has 2 phases. 
  1. First it lists all remote and local files. That can take a lot of time if you have lot of files.
  For faster work it uses concurrent threads. 
  If you interrupt the execution with ctrl+c in this phase, before the lists are completed, the lists are empty.
  You will need to rerun the command and wait for the lists to be fully completed.
  2. The second phase is the same as the command `sync_only`. 
  It can be interrupted with crl+c. The next `sync_only` will continue where it was interrupted.
{GREEN}dropbox_backup_to_external_disk_cli list_and_sync /mnt/e/DropBoxBackup2{RESET}

  Sync only - one-way sync from dropbox to external disk
  It starts the sync only. Does NOT list again the remote and local files, the lists must already be completed 
  from the first command `list_and_sync`.
  It can be interrupted with crl+c. The next `sync_only` will continue where it was interrupted
{GREEN}dropbox_backup_to_external_disk_cli sync_only{RESET}

  {YELLOW}Just for debugging purpose, you can run every step separately.{RESET}
  Test connection and authorization:
{GREEN}dropbox_backup_to_external_disk_cli test{RESET}
  List remote files from Dropbox to `{path_list_source_files}`:
{GREEN}dropbox_backup_to_external_disk_cli remote_list{RESET}
  List local files to `{path_list_destination_files}`:
{GREEN}dropbox_backup_to_external_disk_cli local_list /mnt/e/DropBoxBackup2{RESET}
  List all - both remote and local files to `temp_date/`:
{GREEN}dropbox_backup_to_external_disk_cli all_list /mnt/e/DropBoxBackup2{RESET}  
  Read-only files toggle `{path_list_for_readonly}`:
{GREEN}dropbox_backup_to_external_disk_cli read_only_remove  {RESET}
  Compare file lists and generate `{path_list_for_download}`, `{path_list_for_trash_files}`:
{GREEN}dropbox_backup_to_external_disk_cli compare_files{RESET}
  Compare folders lists and generate `{path_list_for_trash_folders}`:
{GREEN}dropbox_backup_to_external_disk_cli compare_folders{RESET}
  Create folders from `{path_list_for_create_folders}`:
{GREEN}dropbox_backup_to_external_disk_cli create_folders{RESET}
  Move or rename local files if they are equal in trash_files and download_from_list:
{GREEN}dropbox_backup_to_external_disk_cli move_or_rename_local_files{RESET}
  Move to trash from `{path_list_for_trash_folders}`:
{GREEN}dropbox_backup_to_external_disk_cli trash_folders{RESET}
  Move to trash from `{path_list_for_trash_files}`:
{GREEN}dropbox_backup_to_external_disk_cli trash_files{RESET}
  Download files from `{path_list_for_download}`:
{GREEN}dropbox_backup_to_external_disk_cli download_from_list{RESET}
  One single file download:
{GREEN}dropbox_backup_to_external_disk_cli one_file_download <path>{RESET}

  For bash auto-completion:
{GREEN}alias dropbox_backup_to_external_disk_cli=./dropbox_backup_to_external_disk_cli{RESET}
{GREEN}complete -C "dropbox_backup_to_external_disk_cli completion" dropbox_backup_to_external_disk_cli{RESET}

  Visit open-source repository: https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli
    "#
    );
    Ok(())
}

/// Ask the user to paste the token interactively and press Enter. Then calculate the master_key and the token_enc.
/// I need to store the token somewhere because the CLI can be executed many times sequentially.
/// The result of the function must be correct bash commands. They must be executed in the current shell and not in a sub-shell.
/// This command should be executed with `eval $(dropbox_backup_to_external_disk_cli encode_token)` to store the env var in the current shell.
/// Similar to how works `eval $(ssh-agent)`
fn ui_encode_token() -> Result<(), LibError> {
    //input secret token like password in command line
    let token = inquire::Password::new("").without_confirmation().prompt()?;
    // return bash commands because of eval$(...) or
    // communicate errors to user - also as bash command because of eval$(...)
    match lib::encode_token(token) {
        Ok((master_key, token_enc)) => println!(
            r#"
export DBX_KEY_1={master_key}
export DBX_KEY_2={token_enc}
"#
        ),
        Err(err) => println!("printf {RED}{}{RESET}\n", err),
    }
    Ok(())
}

/// ui_test_connection
fn ui_test_connection() -> Result<(), LibError> {
    // communicate errors to user here (if needed)
    // send function pointer
    match lib::test_connection() {
        Ok(_) => {
            println!("Test connection and authorization ok.");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

/// check if external disk base path exists and then
/// saves the base local path for later use like "/mnt/e/DropBoxBackup2"
fn check_and_save_ext_disk_base_path(ext_disk_base_path: &Path) -> Result<(), LibError> {
    if !ext_disk_base_path.exists() {
        println!("{RED}error: ext_disk_base_path not exists {}{RESET}", ext_disk_base_path.to_string_lossy());
        std::process::exit(1);
    }
    let store_path = global_config().path_list_ext_disk_base_path;
    std::fs::write(store_path, ext_disk_base_path.to_string_lossy().as_bytes())?;
    Ok(())
}

/// read ext_disk_base_path from file path_list_ext_disk_base_path
fn get_ext_disk_base_path() -> Result<PathBuf, LibError> {
    let store_path = global_config().path_list_ext_disk_base_path;
    let ext_disk_base_path = std::fs::read_to_string(store_path)?;
    let ext_disk_base_path = std::path::PathBuf::from(&ext_disk_base_path);
    if ext_disk_base_path.to_string_lossy().is_empty() {
        return Err(LibError::ErrorFromStr("ext_disk_base_path is empty!"));
    }
    // return
    Ok(ext_disk_base_path)
}

/// empty lists created by compare
pub fn empty_lists_compared() -> Result<(), LibError> {
    // empty the files, they will be created by compare
    let mut file_list_destination_readonly_files = FileTxt::open_for_read_and_write(global_config().path_list_destination_readonly_files)?;
    file_list_destination_readonly_files.empty()?;
    let mut file_list_for_create_folders = FileTxt::open_for_read_and_write(global_config().path_list_for_create_folders)?;
    file_list_for_create_folders.empty()?;
    let mut file_list_for_download = FileTxt::open_for_read_and_write(global_config().path_list_for_download)?;
    file_list_for_download.empty()?;
    let mut file_list_for_trash_folders = FileTxt::open_for_read_and_write(global_config().path_list_for_trash_folders)?;
    file_list_for_trash_folders.empty()?;
    let mut file_list_for_trash_files = FileTxt::open_for_read_and_write(global_config().path_list_for_trash_files)?;
    file_list_for_trash_files.empty()?;
    let mut file_list_just_downloaded = FileTxt::open_for_read_and_write(global_config().path_list_just_downloaded)?;
    file_list_just_downloaded.empty()?;
    Ok(())
}

/// list in a new thread, then receive messages to print on screen
fn local_list(ext_disk_base_path: &Path) -> Result<(), LibError> {
    println!("{YELLOW}List local.{RESET}");
    empty_lists_compared()?;
    check_and_save_ext_disk_base_path(ext_disk_base_path)?;
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    spawn_list_local(ui_tx.clone())?;

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}: {}", received.1, received.0);
    }

    Ok(())
}

fn spawn_list_local(ui_tx: std::sync::mpsc::Sender<(String, String)>) -> Result<(), LibError> {
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let base_path = FileTxt::open_for_read(global_config().path_list_ext_disk_base_path)?.read_to_string()?;
        let file_list_destination_files = FileTxt::open_for_read_and_write(global_config().path_list_destination_files)?;
        let file_list_destination_folders = FileTxt::open_for_read_and_write(global_config().path_list_destination_folders)?;
        let file_list_destination_readonly_files = FileTxt::open_for_read_and_write(global_config().path_list_destination_readonly_files)?;
        // only the closure is actually spawned, because it is the return value of the block
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::list_local(ui_tx, base_path, file_list_destination_files, file_list_destination_folders, file_list_destination_readonly_files) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });
    Ok(())
}

fn remote_list() -> Result<(), LibError> {
    println!("{YELLOW}List remote.{RESET}");
    empty_lists_compared()?;
    ui_test_connection()?;
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    spawn_list_remote(ui_tx.clone())?;

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}: {}", received.1, received.0);
    }

    Ok(())
}

/// list local and remote in a multiple threads, then receive messages to print on screen
fn all_list(ext_disk_base_path: &Path) -> Result<(), LibError> {
    println!("{YELLOW}List local and remote.{RESET}");
    empty_lists_compared()?;
    check_and_save_ext_disk_base_path(ext_disk_base_path)?;
    ui_test_connection()?;
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();

    spawn_list_local(ui_tx.clone())?;
    spawn_list_remote(ui_tx.clone())?;

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}: {}", received.1, received.0);
    }

    Ok(())
}

fn spawn_list_remote(ui_tx: std::sync::mpsc::Sender<(String, String)>) -> Result<(), LibError> {
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let file_list_source_files = FileTxt::open_for_read_and_write(global_config().path_list_source_files)?;
        let file_list_source_folders = FileTxt::open_for_read_and_write(global_config().path_list_source_folders)?;
        // only the closure is actually spawned, because it is the return value of the block
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::list_remote(ui_tx, file_list_source_files, file_list_source_folders) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });
    Ok(())
}

/// The backup files must not be readonly to allow copying the modified file from the remote.
fn read_only_remove() -> Result<(), LibError> {
    println!("{YELLOW}Remove readonly attribute from files.{RESET}");

    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let mut file_destination_readonly_files = FileTxt::open_for_read_and_write(global_config().path_list_destination_readonly_files)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            // TODO: readonly naturally from windows
            match lib::read_only_remove(ui_tx, &ext_disk_base_path, &mut file_destination_readonly_files) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}", received);
    }

    Ok(())
}

/// compare files
fn compare_files() -> Result<(), LibError> {
    println!("{YELLOW}Compare files.{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::compare_files(ui_tx, global_config()) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}", received);
    }

    Ok(())
}

/// compare folders
fn compare_folders() -> Result<(), LibError> {
    println!("{YELLOW}Compare folders.{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let string_list_source_folder = FileTxt::open_for_read(global_config().path_list_source_folders)?.read_to_string()?;
        let string_list_destination_folders = FileTxt::open_for_read(global_config().path_list_destination_folders)?.read_to_string()?;
        let mut file_list_for_trash_folders = FileTxt::open_for_read_and_write(global_config().path_list_for_trash_folders)?;
        let mut file_list_for_create_folders = FileTxt::open_for_read_and_write(global_config().path_list_for_create_folders)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        // only the closure is actually spawned, because it is the return value of the block
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::compare_folders(
                ui_tx,
                &string_list_source_folder,
                &string_list_destination_folders,
                &mut file_list_for_trash_folders,
                &mut file_list_for_create_folders,
            ) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}", received);
    }

    Ok(())
}

// create folders
fn create_folders() -> Result<(), LibError> {
    println!("{YELLOW}Create folders from list.{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let mut file_list_for_create_folders = FileTxt::open_for_read_and_write(global_config().path_list_for_create_folders)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        // only the closure is actually spawned, because it is the return value of the block
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::create_folders(ui_tx, &ext_disk_base_path, &mut file_list_for_create_folders) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}", received);
    }

    Ok(())
}

// move or rename local files
fn move_or_rename_local_files() -> Result<(), LibError> {
    println!("{YELLOW}Move or rename local files{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let mut path_list_for_trash_files = FileTxt::open_for_read_and_write(global_config().path_list_for_trash_files)?;
        let mut path_list_for_download = FileTxt::open_for_read_and_write(global_config().path_list_for_download)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx_move_to_closure = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::move_or_rename_local_files(ui_tx_move_to_closure, &ext_disk_base_path, &mut path_list_for_trash_files, &mut path_list_for_download) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    let mut is_last_progress_bar = false;
    for received in ui_rx {
        if received == "." {
            // this special character is used to create like a progress bar
            print!("{}", received);
            std::io::Write::flush(&mut std::io::stdout()).ok().expect("Could not flush stdout");
            is_last_progress_bar = true;
        } else {
            if is_last_progress_bar {
                // just newline, if last print was for progress_bar
                println!("");
            } else {
                println!("{}", received);
            }
        }
    }

    Ok(())
}

// trash files from list
fn trash_files() -> Result<(), LibError> {
    println!("{YELLOW}Trash files from list{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let mut file_list_for_trash_files = FileTxt::open_for_read_and_write(global_config().path_list_for_trash_files)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::trash_files(ui_tx, &ext_disk_base_path, &mut file_list_for_trash_files) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}", received);
    }

    Ok(())
}

// trash folders
fn trash_folders() -> Result<(), LibError> {
    println!("{YELLOW}Trash folders{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let mut file_list_for_trash_folders = FileTxt::open_for_read_and_write(global_config().path_list_for_trash_folders)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::trash_folders(ui_tx, &ext_disk_base_path, &mut file_list_for_trash_folders) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}", received);
    }

    Ok(())
}

// download one file
fn download_one_file(path_str: &str) -> Result<(), LibError> {
    println!("{YELLOW}Download one file{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let path_to_download = PathBuf::from(path_str);
        let mut file_list_just_downloaded = FileTxt::open_for_read_and_write(global_config().path_list_just_downloaded)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::download_one_file(ui_tx, &ext_disk_base_path, &path_to_download, &mut file_list_just_downloaded) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}: {}", received.1, received.0);
    }

    Ok(())
}

// download from list
fn download_from_list() -> Result<(), LibError> {
    println!("{YELLOW}Download from list{RESET}");
    // channel for thread communication for user interface
    let (ui_tx, ui_rx) = std::sync::mpsc::channel();
    std::thread::spawn({
        // Prepare variables to be moved/captured to the closure. All is isolated in a block scope.
        let ext_disk_base_path = get_ext_disk_base_path()?;
        let mut file_list_for_download = FileTxt::open_for_read_and_write(global_config().path_list_for_download)?;
        let mut file_list_just_downloaded = FileTxt::open_for_read_and_write(global_config().path_list_just_downloaded)?;
        // Shadow the clone with the same name before the closure.
        let ui_tx = ui_tx.clone();
        move || {
            // catch propagated errors and communicate errors to user or developer
            // spawned closure cannot propagate error with ?
            match lib::download_from_list(ui_tx, &ext_disk_base_path, &mut file_list_for_download, &mut file_list_just_downloaded) {
                Ok(()) => (),
                Err(err) => println!("{RED}{err}{RESET}"),
            }
        }
    });

    //receiver iterator
    drop(ui_tx);
    for received in ui_rx {
        println!("{}: {}", received.1, received.0);
    }

    Ok(())
}
