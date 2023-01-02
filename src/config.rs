use crate::time;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::{fs, io};

/// App configuration, serialized as json in $XDG_CONFIG_HOME/tod.cfg
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct Config {
    /// The Todoist Api token
    pub token: String,
    /// List of Todoist projects and their project numbers
    pub projects: HashMap<String, u32>,
    /// Path to config file
    pub path: String,
    /// The ID of the next task
    pub next_id: Option<String>,
    pub timezone: Option<String>,
    pub last_version_check: Option<String>,
}

impl Config {
    pub fn new(token: &str) -> Result<Config, String> {
        let projects: HashMap<String, u32> = HashMap::new();
        Ok(Config {
            path: generate_path()?,
            token: String::from(token),
            next_id: None,
            last_version_check: None,
            timezone: None,
            projects,
        })
    }

    pub fn create(self) -> Result<Config, String> {
        let json = json!(self).to_string();
        let mut file = fs::File::create(&self.path).or(Err("Could not create file"))?;
        file.write_all(json.as_bytes())
            .or(Err("Could not write to file"))?;
        println!("Config successfully created in {}", &self.path);
        Ok(self)
    }

    pub fn save(self) -> std::result::Result<String, String> {
        let json = json!(self);
        let string = serde_json::to_string_pretty(&json).or(Err("Could not convert to JSON"))?;

        fs::OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.path)
            .or(Err("Could not find config"))?
            .write_all(string.as_bytes())
            .or(Err("Could not write to file"))?;

        Ok(String::from("✓"))
    }

    pub fn load(path: &str) -> Result<Config, String> {
        let mut json = String::new();

        fs::File::open(path)
            .or(Err("Could not find file"))?
            .read_to_string(&mut json)
            .or(Err("Could not read to string"))?;

        serde_json::from_str::<Config>(&json).map_err(|_| String::from("Could not parse JSON"))
    }

    pub fn set_path(self, path: &str) -> Config {
        Config {
            path: String::from(path),
            ..self
        }
    }

    pub fn set_next_id(&self, next_id: String) -> Config {
        let next_id: Option<String> = Some(next_id);

        Config {
            next_id,
            ..self.clone()
        }
    }

    pub fn clear_next_id(self) -> Config {
        let next_id: Option<String> = None;

        Config { next_id, ..self }
    }

    fn check_for_timezone(self: Config) -> Result<Config, String> {
        if self.timezone.is_none() {
            time::list_timezones();
            let desc = "Please enter the number of your timezone";
            let num: usize = get_input(desc)?
                .parse::<usize>()
                .map_err(|_| String::from("Could not parse string into number"))?;
            let config = Config {
                timezone: Some(time::get_timezone(num)),
                ..self
            };

            config.clone().save()?;

            Ok(config)
        } else {
            Ok(self)
        }
    }
}

pub fn get_or_create(config_path: Option<&str>) -> Result<Config, String> {
    let path: String = match config_path {
        None => generate_path()?,
        Some(path) => String::from(path).trim().to_owned(),
    };
    let desc = "Please enter your Todoist API token from https://todoist.com/prefs/integrations ";

    if !path_exists(&path) {
        // We used to store config in $HOME/.tod.cfg
        // This moves it to new path
        let legacy_path = generate_legacy_path()?;
        if path_exists(&legacy_path) {
            println!(
                "INFO: Moving the config file from \"{}\" to \"{}\".\n",
                legacy_path, path
            );
            fs::rename(legacy_path, &path).map_err(|e| e.to_string())?;
        }
    }

    match fs::File::open(&path) {
        Ok(_) => {
            let config = Config::load(&path)?;

            // When we move the config file we also need to rename the path in JSON
            if config.path != path {
                let new_config = config.set_path(&path);
                new_config.clone().save()?;
                Ok(new_config)
            } else {
                Ok(config)
            }
        }
        Err(_) => {
            let token = get_input(desc)?;
            Config::new(&token)?.create()?.check_for_timezone()
        }
    }
}

fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn generate_path() -> Result<String, String> {
    let filename = if cfg!(test) { "test" } else { "tod.cfg" };

    let config_directory = dirs::config_dir()
        .ok_or_else(|| String::from("Could not find config directory"))?
        .to_str()
        .ok_or_else(|| String::from("Could not convert config directory to string"))?
        .to_owned();
    Ok(format!("{}/{}", config_directory, filename))
}

pub fn generate_legacy_path() -> Result<String, String> {
    let filename = if cfg!(test) { "test" } else { ".tod.cfg" };

    let home_directory = dirs::home_dir()
        .ok_or_else(|| String::from("Could not find home directory"))?
        .to_str()
        .ok_or_else(|| String::from("Could not convert directory to string"))?
        .to_owned();
    Ok(format!("{}/{}", home_directory, filename))
}

pub fn get_input(desc: &str) -> Result<String, String> {
    if cfg!(test) {
        return Ok(String::from("5"));
    }

    let mut input = String::new();
    println!("{}", desc);
    io::stdin()
        .read_line(&mut input)
        .or(Err("error: unable to read user input"))?;

    Ok(String::from(input.trim()))
}
