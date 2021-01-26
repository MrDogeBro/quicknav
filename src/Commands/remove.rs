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

pub fn remove(shortcut: String) {
    let config_folder = env::var("XDG_CONFIG_HOME").or_else(|_| env::var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    let config_path = format!("{}/quicknav/quicknav.json", config_folder);

    if !Path::new(&config_path).exists() {
        generate_config();
    }

    let data = File::open(&config_path).expect("Error: Unable to open config file.");
    let mut config: Config = serde_json::from_reader(data).expect("Error: Unable to read config file.");
    let mut found_shortcut = false;
    let mut index_to_remove: usize = 0;

    for (i, shortcut_conf) in &mut config.shortcuts.iter().enumerate() {
        if shortcut_conf.calls.iter().any(|c| c == &shortcut) {
            found_shortcut = true;
            index_to_remove = i;
        }
    }

    if !found_shortcut {
        println!("{} {} {}", "Error: Shortcut with call".red(), shortcut.red(), "was not found.".red());
        exit(1);
    }

    config.shortcuts.remove(index_to_remove);
    fs::write(config_path, serde_json::to_string_pretty(&config).unwrap()).expect("Error: Failed to write config to file.");
    println!("{} {}", "Shortcut removed:".green(), &shortcut);
    exit(0)
}
