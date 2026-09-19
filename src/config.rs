use std::{
    error::Error,
    fs,
    path::{Path, PathBuf}
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub music_directories: Vec<String>
}

impl Default for Config {
    fn default() -> Self{
        Self{
            music_directories: Vec::new(),
        }
    }
}

fn config_path() -> Result<PathBuf, Box<dyn Error>> {
    let config_dir = dirs::config_dir()
        .ok_or("Could not find config directory")?
        .join("amp");

    Ok(config_dir.join("config.toml"))
}

pub fn read_config() -> Result<Config, Box<dyn Error>> {
    let path = config_path()?;
    if !path.exists() {
        let config = Config::default();
        if let Some(parent) = path.parent() {            
            fs::create_dir_all(parent)?;        
        }
        let contents = toml::to_string(&config)?;        
        fs::write(&path, contents)?;
        return Ok(config);    
    }
    let contents = fs::read_to_string(&path)?;    
    let config = toml::from_str(&contents)?;
    Ok(config)
}
