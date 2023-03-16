#![allow(dead_code)]
#![allow(unused_variables)]

use std::env;
use std::fs;
use std::process::Command;
// use libflatpak::prelude::*;

// NOTE: DO NOT RUN THIS WITH shell.nix, just use regular 'cargo run'

#[derive(Debug)]
struct Application {
    installation: String,
    remote: String,
    appid: String,
    local: Option<bool>,
}

#[derive(Debug)]
struct Remote {
    name: String,
    options: String,
}

fn main() {

    let config_entries: Vec<String> = retrieve_config_entries();
    let list_config: Vec<Application> = genlist_config(&config_entries);
    let mut list_system: Vec<Application> = genlist_system();

    check_system(&list_config, &mut list_system);
    remove_apps(&list_system);

    // check_config(&list_config, &mut list_system);
    install_apps(&list_config, &list_system);

    // println!("*** list_config ***");
    // for apps in list_config {
    //     println!("install: {}, remote: {}, appid: {}\n", apps.installation, apps.remote, apps.appid);
    // }
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
    // BUG: newline is being inserted before system!
    let username = env::var("USER").expect("failed to get username");
    let config_path = format!("/home/{}/.config/flatpak-declare/config", &username);
    let config_contents = fs::read_to_string(&config_path).expect("could not read contents of config file");
    let config_entries: Vec<String> = config_contents
        .split(',')
        .map(|field| field.trim())
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
                appid: fields[2].to_string(),
                local: None,
            })
        }
    }
    list_system
}

// fn genlist_install(list_system: &Vec<Application>,
//                    list_config: &Vec<Application>,) -> Vec<Application> {
//     let list_install: Vec<Application> = Vec::new();
//     // TODO: Generate list_install vector
//     // for every struct in list_config
//         // if (remote AND appid AND installation) from list_config is NOT present in list_system
//             // add relevant struct to list_install
//     list_install
// }

// fn genlist_remove(list_system: &Vec<Application>,
//                   list_config: &Vec<Application>,) -> Vec<Application> {
//     let list_remove: Vec<Application> = Vec::new();
//     // your code here!
//     list_remove
// }

fn check_system(list_config: &Vec<Application>, list_system: &mut Vec<Application>){
    for app_sys in list_system {
        for app_config in list_config {
            if app_sys.installation == app_config.installation && app_sys.remote == app_config.remote && app_sys.appid == app_config.appid {
                // println!("The system app {:?} is present in the config file.", app_sys);
                app_sys.local = Some(true);
                // BUG: This is setting ALL of the structs' local fields to true
            }
        }
    }
}

fn remove_apps(list_system: &Vec<Application>){
    for app_sys in list_system {
        println!("{:?}",app_sys);
        if app_sys.local != Some(true){
            // remove the app [ flatpak --system uninstall --noninteractive org.gnucash.GnuCash ]
            // [ flatpak <installation flag> uninstall --noninteractive <appid/ref> ]
            println!("{} is not not present in the config file!",app_sys.appid);
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
                eprintln!("Failed to remove package: {}", String::from_utf8_lossy(&remove.stderr));
            }
        }
    }

    // flatpak uninstall --unused (noninteractive)
    // let remove_unused = Command::new("flatpak")
    //     .arg("uninstall")
    //     .arg("--unused")
    //     .arg("--noninteractive")
    //     .output()
    //     .expect("failed to execute process");
    // if remove_unused.status.success() {
    //     println!("Removed unused packages successfully");
    // } else {
    //     eprintln!("Failed to remove unused packages: {}", String::from_utf8_lossy(&remove_unused.stderr));
    // }

    // flatpak uninstall --delete-data (NOTE: Make this interactive!)
    // let remove_unused = Command::new("flatpak")
    //     .arg("uninstall")
    //     .arg("--delete-data")
    //     .output()
    //     .expect("failed to execute process");
    // if remove_unused.status.success() {
    //     println!("Deleted orphaned app-data successfully");
    // } else {
    //     eprintln!("Failed to remove unused packages: {}", String::from_utf8_lossy(&remove_unused.stderr));
    // }
}

fn install_apps(list_config: &Vec<Application>, list_system: &Vec<Application>){
    // for every struct in list_config:
        // if (remote AND appid AND installation) from list_config is NOT present in list_system
            // set 'local' field from list_config copy to true

    // for every struct in list_config:
        // if local field is true
            // install the app [ flatpak --system install --noninteractive flathub org.gnucash.GnuCash ]
                // [ flatpak <installation flag> install --noninteractive <remote> <appid/ref> ]

    // let package_name = "com.example.MyApp";
    // let repository_name = "flathub";
    // let output = Command::new("flatpak")
    //     .arg("install")
    //     .arg("-y") // Assume 'yes' to prompts (optional, useful for non-interactive environments)
    //     .arg(repository_name)
    //     .arg(package_name)
    //     .output()
    //     .expect("Failed to execute command");
    // if output.status.success() {
    //     println!("Installed package successfully");
    // } else {
    //     eprintln!("Failed to install package: {}", String::from_utf8_lossy(&output.stderr));
    // }
}
