// Read my ~/.zsh_history or read my .zshrc and get history file

use inotify::{Inotify, WatchMask};
use std::path::Path;

struct Machine {
    hostname: String,
    username: String,
    shell: String,
    user_home: Path,
}

struct History {
    begining_time: u64,
    elapsed_second: usize,
    command: String,
}

// get hostname
fn get_hostname() -> String {
    "foo".to_string()
}

fn read_history() -> Vec<History> {}

// get my libsql configs from system variables

// Get the latest shell timestamp

// my libsql file

fn main() {
    let home_dir = match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => path.to_owned(),
        _ => panic!("Unable to get your home dir!"),
    };

    let mut inotify =
        Inotify::init().expect("Error while initializing inotify instance");

    inotify
        .watches()
        .add(
            format!("{}/.zsh_history", home_dir.display()),
            WatchMask::MODIFY | WatchMask::CLOSE,
        )
        .expect("Failed to add file watch");

    let mut buffer = [0; 1024];
    let events = inotify
        .read_events_blocking(&mut buffer)
        .expect("Error while reading events");

    for event in events {
        // Handle event
        println!("Hello, world!");
    }
}
