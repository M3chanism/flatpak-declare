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
    let mut apps = Vec::new();

    for line in lines {
        // println!("{}", i);
        let item = parse_line(line);
        apps.push(item);
    }

    for apps in apps {
        println!("field1: {}, field2: {}, field3: {}", apps.installation, apps.remote, apps.id);
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

fn parse_line(input: &str) -> Application {
    let fields: Vec<&str> = input.split("::").collect();
    Application {
        //BUG: index out of bounds, line is not being separated by ::
        installation: fields[0].to_string(),
        remote: fields[1].to_string(),
        id: fields[2].to_string(),
    }
}

// function add apps
    // if applications vector does not contain struct with given id AND remote, install the given app for the given remote

// function remove apps
    // if applications vector contains struct with given id AND remote, remove the app with the given id AND remote
