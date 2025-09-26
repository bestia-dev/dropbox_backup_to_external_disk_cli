<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# dropbox_backup_to_external_disk_cli

[//]: # (auto_cargo_toml_to_md start)

**CLI binary executable, one-way sync from Dropbox to external disc**  
***version: 2025.926.1538 date: 2025-09-26 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli/)***

 ![dropbox](https://img.shields.io/badge/dropbox-orange)
 ![maintained](https://img.shields.io/badge/maintained-green)
 ![ready-for-use](https://img.shields.io/badge/ready_for_use-green)

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-867-green.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-48-blue.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-129-purple.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_cli/)

[//]: # (auto_lines_of_code end)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_2/blob/main/LICENSE)
 [![Rust](https://github.com/bestia-dev/dropbox_backup_to_external_disk_2/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/dropbox_backup_to_external_disk_2/)
 ![dropbox_backup_to_external_disk_cli](https://bestia.dev/webpage_hit_counter/get_svg_image/772745756.svg)

Hashtags: #rustlang #tutorial #dropbox #maintained #work_in_progress #cli
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## CLI

This compiles into a CLI binary executable. All the user interface is inside this project.  
The main dependency is to the library project `dropbox_backup_to_external_disk_lib` that contains all the program logic.  I separated this projects to show how to use the same library from different binary projects. It is difficult to separate this two layers afterwards. They should be separated from the start.  
Different user-interfaces need different workflows and the common library must allow this. Modern computers and phones are all multi-core. Even javascript has multi-thread capabilities with web-workers. It is recommended to create multi-threaded applications. Most of the calls to the library will be done in a separate thread to have the possibility of communication between the 2 layers (UI and logic). For example for progress bars and similar long running tasks.  

## Cross compile to windows

On my machine I have Windows11 with WSL/Debian. My external drive is exFAT and it works nice with windows. But from WSL it does not work well. WSL/Debian cannot change the datetime of files on the external drive.

Therefore I will cross compile to Windows, copy the exe file with `scp` and run it on Windows.

From Windows it takes cold 16 seconds and hot 2 seconds to list the Local external drive folder. In WSL it takes forever.

Copy the exe file from the container 'crustde' to win folder. Run in windows git-bash:

```bash
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/dropbox_backup_to_external_disk_cli/target/x86_64-pc-windows-gnu/release/dropbox_backup_to_external_disk_cli.exe /c/Users/Luciano/rustprojects/dropbox_backup_to_external_disk/

# then run the local_list with the path like this in git-bash
cd rustprojects/dropbox_backup_to_external_disk
alias dropbox_backup_to_external_disk_cli=./dropbox_backup_to_external_disk_cli
complete -C "dropbox_backup_to_external_disk_cli completion" dropbox_backup_to_external_disk_cli


```

## Development

I use `cargo-auto` to write all repetitive tasks in `automation_tasks_rs`.  

## Try it

There are a few manual steps for the security of you files on Dropbox. Authentication on internet is a complex topic.  
You should be logged in Linux terminal (also in WSL2) with your account. So things you do in that session, are not visible to others. You will set some local environment variables that are private/secret to your linux Session.  After you logout from you Linux session these local environment variables will be deleted.  
The executable will create a sub-directory `tmp` in the current directory. Maybe it is best if you create a dedicated directory `~/dropbox_backup_to_external_disk_cli/` just for this executable and `tmp`.
Download the latest release from [Github](https://github.com/bestia-dev/dropbox_backup_to_external_disk_lib/releases) and make the file executable and enable auto-completion:

```bash
cd ~
mkdir dropbox_backup_to_external_disk_cli
cd dropbox_backup_to_external_disk_cli

curl -L https://github.com/bestia-dev/dropbox_backup_to_external_disk_lib/releases/latest/download/dropbox_backup_to_external_disk_cli --output dropbox_backup_to_external_disk_cli

chmod +x dropbox_backup_to_external_disk_cli
alias dropbox_backup_to_external_disk_cli=./dropbox_backup_to_external_disk_cli
complete -C "dropbox_backup_to_external_disk_cli completion" dropbox_backup_to_external_disk_cli

dropbox_backup_to_external_disk_cli
```

Run the executable without arguments and follow carefully the instructions.  

## Warning

I don't know why, but WSL2 sometimes does not see all the folders of the external disk.  
Instead of 12.000 folders it sees only 28 ???  
Be careful !  
Check it first with this commands to see if the removable disk is really mounted or you see a phantom cached file system.  

```bash
ls /mnt/d
# and/or
df
```

I then restart my Win10 and the problem magically disappears.

## bash auto-completion

This executable is prepared for auto-completion in bash.  
Run this command to define auto-completion in bash for the current session:  

```bash
alias dropbox_backup_to_external_disk_cli=./dropbox_backup_to_external_disk_cli
complete -C "dropbox_backup_to_external_disk_cli completion" dropbox_backup_to_external_disk_cli
```

To make it permanent add this command to the file `~/.bashrc` or some other file that runs commands on bash initialization.  

## Authorization OAuth2

Authorization on the internet is a mess. Dropbox api uses OAuth2.
Every app must be authorized on Dropbox and have its own `app key` and `app secret`.  
For commercial programs they probably embed them into the binary code somehow. But for OpenSource projects it is not possible to keep a secret. So the workaround is: every user must create its own new `dropbox app` exclusive only to him. Creating a new app is simple. This app will stay forever in `development status` in dropbox, to be more private and secure. The  
`$ dropbox_backup_to_external_disk_cli --help`  
has the detailed instructions.  
Then every time before use we need generate the "short-lived access token" for security reasons. There is the possibility to choose "no expiration" token, but I don't like it. Dropbox backup is used rarely and it is not super frustrating to make few clicks for security of your precious files. Having a "no expiration" token is like having another password for the hackers to try to hack. I like more the "short-lived" token. When I'm not using this backup program, there is no access token at all.  
TODO: use the complete OAuth protocol instead of short-lived token

![dropbox_2](https://github.com/bestia-dev/dropbox_backup_to_external_disk_lib/raw/main/images/dropbox_2.png "dropbox_2") ![dropbox_1](https://github.com/bestia-dev/dropbox_backup_to_external_disk_lib/raw/main/images/dropbox_1.png "dropbox_1")
Use this command to store the token (encrypted) in env variable. It will ask for your interactive input like a secret password.

```bash
dropbox_backup_to_external_disk_cli encode_token
```

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
