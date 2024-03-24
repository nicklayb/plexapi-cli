use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CONFIG_NAMESPACE: &str = "plexamp-cli";
const CONFIG_FILE: &str = "config";

pub type Port = i32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub host: String,
    pub port: Port,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub default_player: String,
    pub players: HashMap<String, Player>,
}

impl Configuration {
    pub fn load() -> Configuration {
        confy::load(CONFIG_NAMESPACE, CONFIG_FILE)
            .expect("Something went wrong when loading the configuration")
    }

    pub fn is_default(&self, name: &String) -> bool {
        &self.default_player == name
    }

    pub fn player_exists(&self, name: &String) -> bool {
        self.players.contains_key(name) == true
    }

    pub fn get_player(&self, name: &String) -> Option<&Player> {
        if name == "default" {
            self.players.get(&self.default_player)
        } else {
            self.players.get(name)
        }
    }

    fn has_players(&self) -> bool {
        self.players.is_empty()
    }

    pub fn add_player(&mut self, name: String, host: String, port: Port) -> &Configuration {
        if !self.has_players() {
            self.default_player = name.clone()
        }
        self.players
            .insert(name.clone(), Player { name, host, port });
        self
    }

    pub fn set_default(&mut self, name: String) -> bool {
        if !self.player_exists(&name) {
            self.default_player = name;
            true
        } else {
            false
        }
    }

    pub fn remove_player(&mut self, name: String) -> &Configuration {
        self.players.remove(&name);
        self
    }

    pub fn store(&self) -> &Configuration {
        if let Ok(_) = confy::store(CONFIG_NAMESPACE, Some(CONFIG_FILE), self) {
            println!("Configuration updated")
        } else {
            println!("Configuration failed")
        }
        self
    }
}
