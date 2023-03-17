// #![allow(dead_code)]
// #![allow(unused_variables)]

use std::env;
use std::fs;
use std::process::Command;
// use libflatpak::prelude::*;

// NOTE: DO NOT RUN THIS INSIDE NIX SHELL. 'cargo run' as normal

#[derive(Debug)]
struct Application {
    installation: String,
    remote: String,
    appid: String,
    local: Option<bool>,
}

// #[derive(Debug)]
// struct Remote {
//     name: String,
//     options: String,
// }

fn main() {

    let config_entries: Vec<String> = retrieve_config_entries();
    let mut list_config: Vec<Application> = genlist_config(&config_entries);
    let mut list_system: Vec<Application> = genlist_system();

    check_system(&list_config, &mut list_system);
    remove_apps(&list_system);

    check_config(&mut list_config, &list_system);
    install_apps(&list_config);

    println!("*** list_config ***");
    for apps in list_config {
        println!("install: {}, remote: {}, appid: {}\n", apps.installation, apps.remote, apps.appid);
    }
    println!("*** list_system ***");
    for apps in list_system {
        println!("install: {}, remote: {}, appid: {}\n", apps.installation, apps.remote, apps.appid);
    }
}

fn retrieve_config_entries() -> Vec<String> {
    let username = env::var("USER").expect("failed to get username");
    let config_path = format!("/home/{}/.config/flatpak-declare/config", &username);
    let config_contents = fs::read_to_string(&config_path).expect("could not read contents of config file");
    let config_entries: Vec<String> = config_contents
        .split('\n')
        .map(|field| field.trim()) // ignore whitespace!
        .filter(|item| !item.starts_with('#'))
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
                local: None,
            })
        }
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
                appid: fields[2].to_string(),
                local: None,
            })
        }
    }
    list_system
}

fn check_system(list_config: &Vec<Application>, list_system: &mut Vec<Application>){
    for app_sys in list_system {
        for app_config in list_config {
            if app_sys.installation == app_config.installation && app_sys.remote == app_config.remote && app_sys.appid == app_config.appid {
                // println!("The system app {:?} is present in the config file.", app_sys);
                app_sys.local = Some(true);
            }
        }
    }
}

fn remove_apps(list_system: &Vec<Application>){
    for app_sys in list_system {
        // println!("{:?}",app_sys);
        if app_sys.local != Some(true){
            // remove the app [ flatpak --system uninstall --noninteractive org.gnucash.GnuCash ]
            // [ flatpak <installation flag> uninstall --noninteractive <appid/ref> ]
            println!("{} is not present in the config file! Removing...",app_sys.appid);
            let install_flag = format!("--{}", app_sys.installation);
            let app_id = format!("{}", app_sys.appid);
            let remove = Command::new("flatpak")
                .arg(install_flag)
                .arg("uninstall")
                .arg("--noninteractive")
                .arg(app_id)
                .output()
                .expect("failed to execute process");
            if remove.status.success() {
                println!("Removed {} successfully", app_sys.appid);
            } else {
                eprintln!("Failed to remove {}", String::from_utf8_lossy(&remove.stderr));
            }
        }
    }
}

fn check_config(list_config: &mut Vec<Application>, list_system: &Vec<Application>){
    for app_config in list_config {
        for app_sys in list_system {
            // if app is present in config file but not on system
            if app_config.installation == app_sys.installation && app_config.remote == app_sys.remote && app_config.appid == app_sys.appid {
                // println!("The config entry {:?} is present on the system.", app_config);
                // set local field to false
                app_config.local = Some(false);
            }
        }
    }
}

fn install_apps(list_config: &Vec<Application>){
    for app_config in list_config {
        // println!("{:?}",app_config);
        if app_config.local != Some(false){
            // install the app [ flatpak --system install --noninteractive flathub org.gnucash.GnuCash ]
            // [ flatpak <installation flag> install --noninteractive <remote> <appid/ref> ]
            println!("{} is not installed locally! Installing...",app_config.appid);
            let install_flag = format!("--{}", app_config.installation);
            let remote = format!("{}", app_config.remote);
            let app_id = format!("{}", app_config.appid);
            let install = Command::new("flatpak")
                .arg(install_flag)
                .arg("install")
                .arg("--noninteractive")
                .arg(remote)
                .arg(app_id)
                .output()
                .expect("failed to execute process");
            if install.status.success() {
                println!("Installed {} successfully", app_config.appid);
            } else {
                eprintln!("Failed to install {}", String::from_utf8_lossy(&install.stderr));
            }
        }
    }
}
