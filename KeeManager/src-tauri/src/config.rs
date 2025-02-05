use directories::BaseDirs; // Platform specific directories
use serde::{Deserialize, Serialize}; // To store and retrieve data in specific formats
use std::error::Error;
use std::path::PathBuf; // Building and handling filesystem paths
use std::{fs, io};

// Fortmating Config struct to a specific
// format to be able to send and recieve Config struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub database_path: String,
}

impl Config {
    //
    // Get the base directories for the current platform
    // to get the configuration directory
    //
    fn get_config_path() -> PathBuf {
        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = base_dirs.config_dir();
            let mut config_path = config_dir.to_path_buf();
            config_path.push("KeeManager"); // Program config file name
            config_path.push("config.json");
            config_path
        } else {
            // If not able to find base directories, use current directory
            PathBuf::from("config.json")
        }
    }

    //
    // Loads the configurations from the configuration file
    // If not avilable creates an empty configuration
    //
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            let content = match fs::read_to_string(config_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Failed to read the configuration file: {}", e);
                    return Err(Box::new(e));
                }
            };

            // Deserializes the JSON content
            let config: Config = match serde_json::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Failed to parse the configuration file: {}", e);
                    return Err(Box::new(e));
                }
            };

            Ok(config)
        } else {
            // If configuration file does not exist
            // creates an empty configuration file
            // ( Default configuration )
            Ok(Config {
                database_path: String::new(),
            })
        }
    }

    //
    // Save configuration to a file
    //
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_config_path();

        // Getting the parent path of configuration file directory
        let parent_dir = match config_path.parent() {
            Some(parent) => parent,
            None => {
                eprintln!("Failed to determine the parent directory of the configuration path.");
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Invalid configuration path: No parent directory found",
                )));
            }
        };

        // Creating directories
        match fs::create_dir_all(parent_dir) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Failed to create directories for the configuration file: {}",
                    e
                );
                return Err(Box::new(e));
            }
        }

        // Serializing the configuration in JSON format
        let content = match serde_json::to_string_pretty(self) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Failed to serialize the configuration to JSON: {}", e);
                return Err(Box::new(e));
            }
        };

        // Writing the JSON content to the configuration file
        match fs::write(config_path, content) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to write the configuration file: {}", e);
                Err(Box::new(e))
            }
        }
    }
}
