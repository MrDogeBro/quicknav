use std::fs;
use std::env;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use colored::*;

#[derive(Serialize, Deserialize)]
struct Shortcut {
    name: String,
    description: String,
    location: String,
    calls: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    shortcuts: Vec<Shortcut>
}

fn generate_config() {
    let config_folder = env::var("XDG_CONFIG_HOME").or_else(|_| env::var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    let config_path = format!("{}/quicknav", config_folder);
    fs::create_dir(&config_path).expect("Error: Unable to generate config directory.");
    fs::write(format!("{}/quicknav.json", &config_path), r#"{
    "shortcuts": []
}"#).expect("Error: Unable to generate config.");
}

pub fn add(shortcut: String, location: String, name: Option<String>, description: Option<String>) {
    let config_folder = env::var("XDG_CONFIG_HOME").or_else(|_| env::var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    let config_path = format!("{}/quicknav/quicknav.json", config_folder);

    if !Path::new(&config_path).exists() {
        generate_config();
    }

    let data = File::open(&config_path).expect("Error: Unable to open config file.");
    let mut config: Config = serde_json::from_reader(data).expect("Error: Unable to read config file.");

    for shortcut_conf in &mut config.shortcuts {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            println!("{} {} {}", "Error: Shortcut with call".red(), &shortcut.red(), "already exists.".red());
            exit(1)
        }
    }

    let mut shortcut_name: String = shortcut.to_string();
    let mut shortcut_description: String = shortcut.to_string();
    let mut shortcut_location = location.to_string();
    let cwd = env::current_dir().unwrap().display().to_string();

    if let Some(name) = name { shortcut_name = name; }
    if let Some(description) = description { shortcut_description = description; }
    if location == "." { shortcut_location = cwd; }
    else if !location.starts_with(&env::var("HOME").unwrap()) { shortcut_location = format!("{}/{}", cwd, location); }

    let new_shortcut = Shortcut {
        name: shortcut_name,
        description: shortcut_description,
        location: shortcut_location,
        calls: vec![ shortcut.to_string() ]
    };

    config.shortcuts.push(new_shortcut);
    fs::write(config_path, serde_json::to_string(&config).unwrap()).expect("Error: Unable to generate config.");
    println!("{} {}", "New shortcut added:".green(), &shortcut);
    exit(0)
}
