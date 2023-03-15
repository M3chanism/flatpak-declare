use std::env;
use std::fs;
//use std::process::Command;

// NOTE: DO NOT RUN THIS WITH shell.nix, just use regular 'cargo run'

#[derive(Debug)]
struct Application {
    installation: String,
    remote: String,
    id: String,
}

fn main() {

    let username = env::var("USER").expect("failed to get username");
    let config_path = format!("/home/{}/.config/flatpak-declare/config", &username);
    let config_contents = fs::read_to_string(&config_path).expect("could not read contents of config file");
    let lines: Vec<&str> = config_contents.split(',').collect();
    let mut apps: Vec<Application> = Vec::new();

    for line in lines {
        // push_fields(line, apps);
        let fields: Vec<&str> = line.split("::").collect();
        if fields.len() >= 3 {
            apps.push(Application {
                installation: fields[0].to_string(),
                remote: fields[1].to_string(),
                id: fields[2].to_string(),
            })
        }
        // else {
        //     println!("Error: line does not contain enough fields");
        // }
    }

    for apps in apps {
        println!("install: {}, remote: {}, id: {}\n", apps.installation, apps.remote, apps.id);
    }


    // println!("In file {}", &config_path);
    // println!("With text:\n{config_contents}");

    // let apps = Command::new("flatpak")
    //     .arg("list")
    //     .arg("--app")
    //     .arg("--columns=application")
    //     .output()
    //     .expect("failed to execute process");
    // println!("{}", String::from_utf8_lossy(&apps.stdout));

    // let remotes = Command::new("flatpak")
    //     .arg("remotes")
    //     .arg("--columns=name")
    //     .output()
    //     .expect("failed to execute process");
    // println!("{}", String::from_utf8_lossy(&remotes.stdout));
}

// fn push_fields(line: &str, mut apps: Vec<Application>) {
//     let fields: Vec<&str> = line.split("::").collect();
//     if fields.len() >= 3 {
//         apps.push(Application {
//             installation: fields[0].to_string(),
//             remote: fields[1].to_string(),
//             id: fields[2].to_string(),
//         })
//     }
// }

// function add apps
    // if applications vector does not contain struct with given id AND remote, install the given app for the given remote

// function remove apps
    // if applications vector contains struct with given id AND remote, remove the app with the given id AND remote
