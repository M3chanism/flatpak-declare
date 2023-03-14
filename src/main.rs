use std::env;
use std::fs;
use std::process::Command;

// NOTE: DO NOT RUN THIS WITH shell.nix, just use regular 'cargo run'

// array: pkg_add:
// array: pkg_remove:
// array: remote_add:
// array: remote_remove:

fn main() {

    let username = env::var("USER").expect("failed to get username");
    let config_path = format!("/home/{}/.config/flatpak-declare/config", &username);
    let config_contents = fs::read_to_string(&config_path).expect("could not read contents of config file");

    println!("In file {}", &config_path);
    println!("With text:\n{config_contents}");

    let apps = Command::new("flatpak")
        .arg("list")
        .arg("--app")
        .arg("--columns=application")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&apps.stdout));

    let remotes = Command::new("flatpak")
        .arg("remotes")
        .arg("--columns=name")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&remotes.stdout));
}

// check package for remote

// function: flatpak execute
    // if app is listed in pkg_add, install the app.
    // if app is listed in pkg_remove, remove the app.
    // if remote is listed in remote_add, add the remote
    // if remote is listed in remote_remove, remove the remote

// function: app parser
    // if app is on 'flatpak list --app --columns=application' but not in the config file, add to pkg_remove
    // if app is in the config file but not on 'flatpak list --app --columns=application', add to pkg_add

// function: remote parser
    // if app is on 'flatpak remotes' but not in the config file, add to pkg_remove
    // if app is in the config file but not on 'flatpak remotes', add to pkg_add
