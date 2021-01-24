use std::fs;
use std::fs::File;
use std::env::var;
use std::path::Path;

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
    let home = var("XDG_CONFIG_HOME").or_else(|_| var("HOME")).unwrap();
    let config_path = format!("{}/.config/quicknav", home);
    fs::create_dir(&config_path).expect("Error: Unable to generate config directory.");
    fs::write(format!("{}/quicknav.json", &config_path), r#"{
    "shortcuts": []
}"#).expect("Error: Unable to generate config.");
}

pub fn get(location: String) {
    let home = var("XDG_CONFIG_HOME").or_else(|_| var("HOME")).unwrap();
    let config_path = format!("{}/.config/quicknav/quicknav.json", home);

    if !Path::new(&config_path).exists() {
        generate_config();
    }

    let data = File::open(config_path).expect("Error: Unable to open config file.");
    let config: Config = serde_json::from_reader(data).expect("Error: Unable to read config file.");

    for shortcut in config.shortcuts {
        if shortcut.calls.iter().any(|c| c == &location) {
            return println!("{}", shortcut.location.replace("~", &home));
        }
    }
    println!("Error: Navigation shortcut not found. Use quicknav list to view all your shortcuts.");
}
