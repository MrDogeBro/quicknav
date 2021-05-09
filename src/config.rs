use anyhow::Result;
use std::env::var;
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Shortcut {
    pub name: String,
    pub description: String,
    pub location: String,
    pub calls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub create_missing_directories: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
    #[serde(default)]
    pub options: Options,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            create_missing_directories: false,
        }
    }
}

impl Config {
    fn generate() -> Result<i32> {
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

    pub fn load() -> Result<Config> {
        let config_folder = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
            .unwrap();
        let config_path = format!("{}/quicknav/quicknav.json", config_folder);

        if !Path::new(&config_path).exists() {
            Config::generate()?;
        }

        let data = File::open(config_path)?;
        let config: Config = serde_json::from_reader(data)?;
        println!("{:?}", config);

        Ok(config)
    }

    pub fn update(&self) -> Result<i32> {
        let config_folder = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|home| format!("{}/.config", home)))
            .unwrap();
        let config_path = format!("{}/quicknav/quicknav.json", config_folder);

        fs::write(config_path, serde_json::to_string_pretty(self).unwrap())?;

        Ok(0)
    }
}
