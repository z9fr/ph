use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    path: String,
    names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    workspaces: HashMap<String, Workspace>,
}

impl Config {
    fn load() -> io::Result<Self> {
        let config_dir = Config::config_dir()?;
        let config_path = config_dir.join("ph.conf");
        let mut file = match File::open(&config_path) {
            Ok(file) => file,
            Err(_) => {
                return Ok(Config {
                    workspaces: HashMap::new(),
                })
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&contents).unwrap();
        Ok(config)
    }

    fn save(&self) -> io::Result<()> {
        let config_dir = Config::config_dir()?;
        let config_path = config_dir.join("ph.conf");
        let mut file = File::create(&config_path)?;
        let contents = toml::to_string(&self).unwrap();
        file.write_all(contents.as_bytes())?;
        Ok(())
    }

    fn add_workspace(&mut self, name: String, path: String) -> io::Result<()> {
        let workspace = self
            .workspaces
            .entry(name.clone())
            .or_insert_with(|| Workspace {
                path: path.clone(),
                names: Vec::new(),
            });
        workspace.path = path;
        if !workspace.names.contains(&name) {
            workspace.names.push(name);
        }
        self.save()
    }

    fn find_workspace(&self, name: &str) -> Option<&Workspace> {
        self.workspaces.get(name)
    }

    fn config_dir() -> io::Result<PathBuf> {
        let home_dir = match env::var_os("HOME") {
            Some(path) => path,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "missing HOME env var",
                ))
            }
        };
        let config_dir = Path::new(&home_dir).join(".config");
        if !config_dir.exists() {
            fs::create_dir(&config_dir)?;
        }
        Ok(config_dir)
    }
}

pub mod workspace {
    use std::io::{self, Write};

    use super::Config;

    pub fn create(name: String, path: String) {
        let mut config = Config::load().unwrap();
        config.add_workspace(name, path).unwrap();
    }

    pub fn get(name: String) -> io::Result<()> {
        let config = Config::load()?;
        match config.find_workspace(&name) {
            Some(workspace) => {
                for name in &workspace.names {
                    println!("{} => {}", name, workspace.path);
                }
            }
            None => {
                writeln!(
                    io::stderr(),
                    "error: no workspace found for name '{}'",
                    name
                )?;
            }
        }
        Ok(())
    }
}
