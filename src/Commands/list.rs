use std::fs::File;
use std::env::var;
use prettytable::{Table};
use prettytable::format;

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

    let mut shortcut_list = Table::new();
    shortcut_list.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    shortcut_list.set_titles(row!["Shortcuts", "Shortcut Name", "Shortcut Location"]);

    for shortcut in config.shortcuts {
        let calls: String = shortcut.calls.join(", ");
        shortcut_list.add_row(row![calls, shortcut.name, shortcut.location]);
    }

    shortcut_list.printstd();
}
