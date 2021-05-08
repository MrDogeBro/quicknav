use anyhow::Result;
use std::env::var;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub description: String,
    pub location: String,
    pub calls: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub create_missing_directories: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
    pub options: Options,
}

fn generate_config() -> Result<i32> {
    let config_folder = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();
    let config_path = format!("{}/quicknav", config_folder);

    if !Path::new(&config_path).exists() {
        fs::create_dir(&config_path)?;
    }

    fs::write(
        format!("{}/quicknav.json", &config_path),
        r#"{
    "shortcuts": [],
    "options": {
        "create_missing_directories": false
    }
}"#,
    )?;

    Ok(0)
}

pub fn load_config() -> Result<Config> {
    let config_folder = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();
    let config_path = format!("{}/quicknav/quicknav.json", config_folder);

    if !Path::new(&config_path).exists() {
        generate_config()?;
    }

    let data = File::open(config_path)?;
    let config: Config = serde_json::from_reader(data)?;

    Ok(config)
}

pub fn update_config(config: Config) -> Result<i32> {
    let config_folder = var("XDG_CONFIG_HOME")
        .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
        .unwrap();
    let config_path = format!("{}/quicknav/quicknav.json", config_folder);

    fs::write(config_path, serde_json::to_string_pretty(&config).unwrap())?;

    Ok(0)
}
