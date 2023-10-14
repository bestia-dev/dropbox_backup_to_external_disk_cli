// src/bin/dropbox_backup_to_external_disk.rs

// CLI binary project around the library.
// All work with input/output should be inside the bin project, and nothing in the lib project.
// Inside bin I should print on the screen and open or create Files. Then pass the Files to the lib part to operate on them.
// But to be interactive I cannot wait for a lib function to finish. The lib functions should be in another thread.
// Then send msg to the bin main thread that print that to the screen.

// use exported code from the lib project
use dropbox_backup_to_external_disk::*;

// define paths in bin, not in lib
static APP_CONFIG: AppConfig = AppConfig {
    path_list_base_local_path: "temp_data/list_base_local_path.csv",
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

/// AppState struct contains only private fields.
/// Those will be used as global mutable, but only using methods from AppStateTrait.
#[derive(Debug)]
struct AppState {
    string_x: String,
}

/// implementation of AppStateTrait that is defined in the lib project
impl AppStateTrait for AppState {
    fn load_keys_from_io(&self) -> Result<(String, String), LibError> {
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

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    /*     ctrlc::set_handler(move || {
        println!("terminated with ctrl+c. {}", *UNHIDE_CURSOR);
        std::process::exit(exitcode::OK);
    })
    .expect("Error setting Ctrl-C handler"); */

    // init the global struct APP_STATE defined in the lib project
    let _ = APP_STATE.set(std::sync::Mutex::new(Box::new(AppState { string_x: String::from("") })));

    //create the directory temp_data/
    std::fs::create_dir_all("temp_data").unwrap();

    /*   let base_path = if std::path::Path::new(APP_CONFIG.path_list_base_local_path).exists() {
        std::fs::read_to_string(APP_CONFIG.path_list_base_local_path).unwrap()
    } else {
        String::new()
    }; */

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
            _ => println!("Unrecognized arguments. Try dropbox_backup_to_external_disk --help"),
        },
        Some("sync_only") => {
            let ns_started = ns_start("sync_only");
            print!("{}", *CLEAR_ALL);
            sync_only(&APP_CONFIG);
            ns_print_ms("sync_only", ns_started);
        }
        Some("remote_list") => {
            print!("{}", *CLEAR_ALL);
            println!("{}{}{}remote_list into {}{}", at_line(1), *CLEAR_LINE, *YELLOW, APP_CONFIG.path_list_source_files, *RESET,);
            let ns_started = ns_start("");
            test_connection();
            list_remote(&APP_CONFIG);
            ns_print_ms("remote_list", ns_started);
        }
        Some("local_list") => match env::args().nth(2).as_deref() {
            Some(path) => {
                print!("{}", *CLEAR_ALL);
                println!("{}{}{}local_list into {}{}", at_line(1), *CLEAR_LINE, *YELLOW, APP_CONFIG.path_list_destination_files, *RESET,);
                let ns_started = ns_start("");
                list_local(path, &APP_CONFIG);
                ns_print_ms("local_list", ns_started);
            }
            _ => println!("Unrecognized arguments. Try `dropbox_backup_to_external_disk --help`"),
        },
        Some("all_list") => match env::args().nth(2).as_deref() {
            Some(path) => {
                print!("{}", *CLEAR_ALL);
                println!("{}{}{}remote and local lists into temp_data{}", at_line(1), *CLEAR_LINE, *YELLOW, *RESET);
                let ns_started = ns_start("");
                test_connection();
                all_list_remote_and_local(path, &APP_CONFIG);
                ns_print_ms("all_list", ns_started);
            }
            _ => println!("Unrecognized arguments. Try `dropbox_backup_to_external_disk --help`"),
        },
        Some("read_only_toggle") => {
            let ns_started = ns_start("read_only_toggle");
            println!("{}read_only_toggle{}", *YELLOW, *RESET);
            // open file as read and write
            let mut file_destination_readonly_files = FileTxt::open_for_read_and_write(APP_CONFIG.path_list_destination_readonly_files).unwrap();
            read_only_toggle(&mut file_destination_readonly_files, &base_path);
            ns_print_ms("read_only_toggle", ns_started);
        }
        Some("compare_files") => {
            let ns_started = ns_start("compare lists");
            println!("{}compare remote and local files{}", *YELLOW, *RESET);
            compare_files(&APP_CONFIG);
            ns_print_ms("compare_files", ns_started);
        }
        Some("compare_folders") => {
            let ns_started = ns_start("compare_folders");
            println!("{}compare remote and local folders{}", *YELLOW, *RESET);
            let string_list_source_folder = std::fs::read_to_string(APP_CONFIG.path_list_source_folders).unwrap();
            let string_list_destination_folders = std::fs::read_to_string(APP_CONFIG.path_list_destination_folders).unwrap();
            let mut file_list_for_trash_folders = FileTxt::open_for_read_and_write(APP_CONFIG.path_list_for_trash_folders).unwrap();
            let mut file_list_for_create_folders = FileTxt::open_for_read_and_write(APP_CONFIG.path_list_for_create_folders).unwrap();
            compare_folders(
                &string_list_source_folder,
                &string_list_destination_folders,
                &mut file_list_for_trash_folders,
                &mut file_list_for_create_folders,
            );
            println!("Created files: list_for_trash_folders.csv and list_for_create_folders.csv");
            ns_print_ms("compare_folders", ns_started);
        }
        Some("create_folders") => {
            if base_path.is_empty() {
                println!("error: base_path is empty!");
            } else {
                let ns_started = ns_start(&format!("create_folders {}", APP_CONFIG.path_list_for_create_folders));
                let mut file_list_for_create_folders = FileTxt::open_for_read_and_write(APP_CONFIG.path_list_for_create_folders).unwrap();
                create_folders(&mut file_list_for_create_folders, &base_path);
                ns_print_ms("create_folders", ns_started);
            }
        }
        Some("trash_folders") => {
            if base_path.is_empty() {
                println!("error: base_path is empty!");
            } else {
                let ns_started = ns_start(&format!("trash_folders {}", APP_CONFIG.path_list_for_trash_folders));
                let mut file_list_for_trash_folders = FileTxt::open_for_read_and_write(APP_CONFIG.path_list_for_trash_folders).unwrap();
                trash_folders(&mut file_list_for_trash_folders, &base_path);
                ns_print_ms("trash_folders", ns_started);
            }
        }
        Some("move_or_rename_local_files") => {
            let ns_started = ns_start("move_or_rename_local_files");
            move_or_rename_local_files(&APP_CONFIG);
            ns_print_ms("move_or_rename_local_files", ns_started);
        }
        Some("trash_from_list") => {
            let ns_started = ns_start(&format!("trash from {}", APP_CONFIG.path_list_for_trash));
            trash_from_list(&APP_CONFIG);
            ns_print_ms("trash_from_list", ns_started);
        }
        Some("correct_time_from_list") => {
            let ns_started = ns_start(&format!("correct time of files from {}", APP_CONFIG.path_list_for_correct_time));
            correct_time_from_list(&APP_CONFIG);
            ns_print_ms("correct_time_from_list", ns_started);
        }
        Some("download_from_list") => {
            let ns_started = ns_start(&format!("download from {}", APP_CONFIG.path_list_for_download));
            download_from_list(&APP_CONFIG);
            ns_print_ms("download_from_list", ns_started);
        }
        Some("one_file_download") => match env::args().nth(2).as_deref() {
            Some(path) => download_one_file(path, &APP_CONFIG),
            _ => println!("Unrecognized arguments. Try `dropbox_backup_to_external_disk --help`"),
        }, */
        _ => println!("Unrecognized arguments. Try `dropbox_backup_to_external_disk --help`"),
    }
    // TODO: receive msg from other threads

    Ok(())
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
/// `complete -C "dropbox_backup_to_external_disk completion" dropbox_backup_to_external_disk`
/// `complete -p`  - shows all the completion commands
/// `complete -r xxx` - deletes a completion command
fn completion() {
    /// println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if sub_found == false {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    // `complete -C "dropbox_backup_to_external_disk completion" dropbox_backup_to_external_disk`
    // this completion always sends this arguments:
    // 0. executable path
    // 1. word completion
    // 2. executable file name
    // 3. word_being_completed (even if it is empty)
    // 4. last_word
    let word_being_completed = args[3].as_str();
    let last_word = args[4].as_str();

    if last_word.ends_with("dropbox_backup_to_external_disk") {
        let sub_commands = vec![
            "--help",
            "-h",
            "all_list",
            "compare_files",
            "compare_folders",
            "create_folders",
            "read_only_toggle",
            "correct_time_from_list",
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
            "trash_from_list",
        ];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    // the second level if needed
    else if last_word == "list_and_sync" || last_word == "local_list" || last_word == "all_list" {
        let sub_commands = vec!["/mnt/d/DropboxBackup1"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
}

/// print help
fn print_help() {
    println!(
        r#"
  {YELLOW}{BOLD}Welcome to dropbox_backup_to_external_disk{RESET}

  {YELLOW}1. Before first use, create your private Dropbox app:{RESET}
  - Open browser on {GREEN}<https://www.dropbox.com/developers/apps?_tk=pilot_lp&_ad=topbar4&_camp=myapps>{RESET}
  - Click Create app, choose Scoped access, choose Full dropbox
  - Choose a globally unique app name like {GREEN}`backup_{date}`{RESET}
  - Go to tab Permissions, check `files.metadata.read` and `files.content.read`, click Submit, close browser

  {YELLOW}2. Before every use, create a short-lived access token (secret):{RESET}
  - Open browser on {GREEN}<https://www.dropbox.com/developers/apps?_tk=pilot_lp&_ad=topbar4&_camp=myapps>{RESET}
  - Choose your existing private Dropbox app like {GREEN}`backup_{date}`{RESET}
  - Click button `Generate` to generated short-lived access token and copy it, close browser
  - In you Linux terminal session store the token to use it then in multiple sequential commands in your current shell with eval:
{GREEN}  eval $(dropbox_backup_to_external_disk encode_token){RESET}
  - This temporary token will be deleted when the session ends.
  - Test if the authentication works:
{GREEN}  dropbox_backup_to_external_disk test{RESET}

  {YELLOW}Commands:{RESET}
  Full list and sync - from dropbox to external disk
  This command has 2 phases. 
  1. First it lists all remote and local files. That can take a lot of time if you have lot of files.
  For faster work it uses concurrent threads. 
  If you interrupt the execution with ctrl+c in this phase, before the lists are completed, the lists are empty.
  You will need to rerun the command and wait for the lists to be fully completed.
  2. The second phase is the same as the command `sync_only`. 
  It can be interrupted with crl+c. The next `sync_only` will continue where it was interrupted.
{GREEN}dropbox_backup_to_external_disk list_and_sync /mnt/d/DropBoxBackup1{RESET}

  Sync only - one-way sync from dropbox to external disk
  It starts the sync only. Does NOT list again the remote and local files, the lists must already be completed 
  from the first command `list_and_sync`.
  It can be interrupted with crl+c. The next `sync_only` will continue where it was interrupted
{GREEN}dropbox_backup_to_external_disk sync_only{RESET}

  {YELLOW}Just for debugging purpose, you can run every step separately.{RESET}
  Test connection and authorization:
{GREEN}dropbox_backup_to_external_disk test{RESET}
  List remote files from Dropbox to `{path_list_source_files}`:
{GREEN}dropbox_backup_to_external_disk remote_list{RESET}
  List local files to `{path_list_destination_files}`:
{GREEN}dropbox_backup_to_external_disk local_list /mnt/d/DropBoxBackup1{RESET}
  List all - both remote and local files to `temp_date/`:
{GREEN}dropbox_backup_to_external_disk all_list /mnt/d/DropBoxBackup1{RESET}  
  Read-only files toggle `{path_list_for_readonly}`:
{GREEN}dropbox_backup_to_external_disk read_only_toggle  {RESET}
  Compare file lists and generate `{path_list_for_download}`, `{path_list_for_trash}` and `{path_list_for_correct_time}`:
{GREEN}dropbox_backup_to_external_disk compare_files{RESET}
  Compare folders lists and generate `{path_list_for_trash_folders}`:
{GREEN}dropbox_backup_to_external_disk compare_folders{RESET}
  Create folders from `{path_list_for_create_folders}`:
{GREEN}dropbox_backup_to_external_disk create_folders{RESET}
  Move or rename local files if they are equal in trash_from_list and download_from_list:
{GREEN}dropbox_backup_to_external_disk move_or_rename_local_files{RESET}
  Move to trash from `{path_list_for_trash_folders}`:
{GREEN}dropbox_backup_to_external_disk trash_folders{RESET}
  Move to trash from `{path_list_for_trash}`:
{GREEN}dropbox_backup_to_external_disk trash_from_list{RESET}
  Correct time of files from `{path_list_for_correct_time}`:
{GREEN}dropbox_backup_to_external_disk correct_time_from_list{RESET}
  Download files from `{path_list_for_download}`:
{GREEN}dropbox_backup_to_external_disk download_from_list{RESET}
  One single file download:
{GREEN}dropbox_backup_to_external_disk one_file_download <path>{RESET}

  For bash auto-completion:
{GREEN}alias dropbox_backup_to_external_disk=./dropbox_backup_to_external_disk{RESET}
{GREEN}complete -C "dropbox_backup_to_external_disk completion" dropbox_backup_to_external_disk{RESET}

  Visit open-source repository: https://github.com/bestia-dev/dropbox_backup_to_external_disk
    "#,
        path_list_source_files = APP_CONFIG.path_list_source_files,
        path_list_destination_files = APP_CONFIG.path_list_destination_files,
        path_list_for_download = APP_CONFIG.path_list_for_download,
        path_list_for_correct_time = APP_CONFIG.path_list_for_correct_time,
        path_list_for_trash = APP_CONFIG.path_list_for_trash,
        path_list_for_readonly = APP_CONFIG.path_list_destination_readonly_files,
        path_list_for_trash_folders = APP_CONFIG.path_list_destination_folders,
        path_list_for_create_folders = APP_CONFIG.path_list_for_create_folders,
        date = chrono::offset::Utc::now().format("%Y%m%dT%H%M%SZ"),
    );
}

/// Ask the user to paste the token interactively and press Enter. Then calculate the master_key and the token_enc.
/// I need to store the token somewhere because the CLI can be executed many times sequentially.
/// The result of the function must be correct bash commands. They must be executed in the current shell and not in a sub-shell.
/// This command should be executed with `eval $(dropbox_backup_to_external_disk encode_token)` to store the env var in the current shell.
/// Similar to how works `eval $(ssh-agent)`
fn ui_encode_token() {
    /// Inner function is a separate function so I can use the `?` control flow.
    fn ui_encode_token_inner() -> Result<(String, String), LibError> {
        //input secret token like password in command line
        let token = inquire::Password::new("").without_confirmation().prompt()?;
        let (master_key, token_enc) = encode_token(token)?;
        Ok((master_key, token_enc))
    }

    // return bash commands because of eval$(...) or
    // communicate errors to user - also as bash command because of eval$(...)
    match ui_encode_token_inner() {
        Ok((master_key, token_enc)) => println!(
            r#"
export DBX_KEY_1={master_key}
export DBX_KEY_2={token_enc}
"#
        ),
        Err(err) => println!("echo {}", err),
    }
}

/// ui_test_connection
fn ui_test_connection() {
    // communicate errors to user here (if needed)
    // send function pointer
    match test_connection() {
        Ok(_) => println!("Test connection and authorization ok."),
        Err(err) => println!("{}", err),
    }
}
