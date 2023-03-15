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

    let username = env::var("USER").expect("failed to get username");
    let config_path = format!("/home/{}/.config/flatpak-declare/config", &username);
    let config_contents = fs::read_to_string(&config_path).expect("could not read contents of config file");
    let lines: Vec<&str> = config_contents.split(',').collect();
    let mut list_config: Vec<Application> = Vec::new();
    let mut list_system: Vec<Application> = Vec::new();
    let mut list_install: Vec<Application> = Vec::new();
    let mut list_remove: Vec<Application> = Vec::new();

    genlist_config(&lines, &mut list_config);
    genlist_system(&mut list_system);
    genlist_install(&list_system, &list_config, &mut list_install);
    genlist_remove(&list_system, &list_config, &mut list_remove);
    install_apps(&list_install);
    remove_apps(&list_remove);

    for apps in list_config {
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

fn genlist_config(lines: &Vec<&str>, list_config: &mut Vec<Application>){
    for line in lines {
        // push_fields(line, list_config);
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
}

fn genlist_system(list_system: &mut Vec<Application>){
    let apps = Command::new("flatpak")
        .arg("list")
        .arg("--app")
        .arg("--columns=application")
        .output()
        .expect("failed to execute 'flatpak list --app --columns=application'");
    println!("{}", String::from_utf8_lossy(&apps.stdout));
    // TODO: parse apps string into apps_system vector

}

fn genlist_install(list_system: &Vec<Application>,
                   list_config: &Vec<Application>,
                   mut list_install: &mut Vec<Application>){
    // for every struct in the app vec
    for app in list_install{
        // if remote and appid from list_config is NOT present on system (hint: use 'flatpak list')
            // add appid to list_install
    }
}

fn genlist_remove(list_system: &Vec<Application>,
                  list_config: &Vec<Application>,
                  mut list_remove: &mut Vec<Application>){
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
