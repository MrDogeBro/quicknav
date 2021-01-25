use std::fs::File;
use std::env::var;

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

pub fn list() {
    let config_folder = var("XDG_CONFIG_HOME").or_else(|_| var("HOME").map(|home|format!("{}/.config", home))).unwrap();
    let config_path = format!("{}/quicknav/quicknav.json", config_folder);

    let data = File::open(config_path).expect("Error: Unable to open config file.");
    let config: Config = serde_json::from_reader(data).expect("Error: Unable to read config file.");

    for shortcut in config.shortcuts {
        println!("{}", shortcut.location)
    }
}
