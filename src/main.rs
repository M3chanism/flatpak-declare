#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::fs;
use std::process::Command;

// NOTE: DO NOT RUN THIS WITH shell.nix, just use regular 'cargo run'

#[derive(Debug)]
struct Application {
    installation: String,
    remote: String,
    appid: String,
}

struct Remote {
    name: String,
    options: String,
}

fn main() {

    let config_entries: Vec<String> = retrieve_config_entries();
    let list_config: Vec<Application> = genlist_config(&config_entries);
    let list_system: Vec<Application> = genlist_system();
    let list_install: Vec<Application> = genlist_install(&list_system, &list_config);
    let list_remove: Vec<Application> = genlist_remove(&list_system, &list_config);

    install_apps(&list_install);
    remove_apps(&list_remove);

    println!("*** list_config ***");
    for apps in list_config {
        println!("install: {}, remote: {}, appid: {}\n", apps.installation, apps.remote, apps.appid);
    }
    println!("*** list_system ***");
    for apps in list_system {
        println!("install: {}, remote: {}, appid: {}\n", apps.installation, apps.remote, apps.appid);
    }

    // println!("In file {}", &config_path);
    // println!("With text:\n{config_contents}");

    // let remotes = Command::new("flatpak")
    //     .arg("remotes")
    //     .arg("--columns=name")
    //     .output()
    //     .expect("failed to execute process");
    // println!("{}", String::from_utf8_lossy(&remotes.stdout));
}

fn retrieve_config_entries() -> Vec<String> {
    let username = env::var("USER").expect("failed to get username");
    let config_path = format!("/home/{}/.config/flatpak-declare/config", &username);
    let config_contents = fs::read_to_string(&config_path).expect("could not read contents of config file");
    let config_entries: Vec<String> = config_contents
        .split(',')
        .map(|entry| entry.into()) // transform each &str element in the iterator produced by config_contents.split(',') into a String. This avoids referencing the local variable 'config_contents' since it will be dropped when function scope ends.
        .collect();
    config_entries
}

fn genlist_config(config_entries: &Vec<String>) -> Vec<Application> {
    let mut list_config = Vec::new();
    for line in config_entries {
        let fields: Vec<&str> = line.split("::").collect();
        if fields.len() >= 3 {
            list_config.push(Application {
                installation: fields[0].to_string(),
                remote: fields[1].to_string(),
                appid: fields[2].to_string(),
            })
        }
        // else {
        //     println!("Error: line does not contain enough fields");
        // }
    }
    list_config
}

fn genlist_system() -> Vec<Application> {
    let mut list_system: Vec<Application> = Vec::new();
    let apps = Command::new("flatpak")
        .arg("list")
        .arg("--app")
        .arg("--columns=installation,origin,application")
        .output()
        .expect("failed to execute 'flatpak list --app --columns=application'");
    // println!("*** from inside genlist_system ***");
    // println!("{}", String::from_utf8_lossy(&apps.stdout));
    let apps_stdout = String::from_utf8_lossy(&apps.stdout);
    for line in apps_stdout.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 3 {
            list_system.push(Application {
                installation: fields[0].to_string(),
                remote: fields[1].to_string(),
                appid: fields[2].to_string()
            })
        }
    }
    list_system
}

fn genlist_install(list_system: &Vec<Application>,
                   list_config: &Vec<Application>,) -> Vec<Application> {
    let list_install: Vec<Application> = Vec::new();
    // TODO: Generate list_install vector
    // for every struct in list_config
        // if (remote AND appid AND installation) from list_config is NOT present in list_system
            // add relevant struct to list_install
    list_install
}

fn genlist_remove(list_system: &Vec<Application>,
                  list_config: &Vec<Application>,) -> Vec<Application> {
    let list_remove: Vec<Application> = Vec::new();
    // your code here!
    list_remove
}

fn install_apps(list_install: &Vec<Application>){
    // iterate through remotes
        // run 'flatpak --system install flathub org.qutebrowser.qutebrowser'
        // run 'flatpak <install flag> install <remote> <appid>'
}

fn remove_apps(list_remove: &Vec<Application>){
    // if applications vector contains struct with given appid AND remote, remove the app with the given appid AND remote
    // uninstall unused apps
}
