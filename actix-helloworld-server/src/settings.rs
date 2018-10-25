use config::{ConfigError, Config, File};
use serde;
use serde::de::Deserializer;
use serde::Deserialize;


pub trait DeserializeWith: Sized {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>;
}

#[derive(Debug, Deserialize, Clone)]
pub enum ServerMode {
    Dev,
    Production
}

impl DeserializeWith for ServerMode {
    fn deserialize_with<'de, D>(de: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(de)?;

        match s.as_ref() {
            "dev" => Ok(ServerMode::Dev),
            "production" => Ok(ServerMode::Production),
            _ => Err(serde::de::Error::custom("not available pattern"))
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server_name: String,
    #[serde(deserialize_with="ServerMode::deserialize_with")]
    pub mode: ServerMode,
    pub server: ServerSettings

}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub timeout: usize,
    pub workers: usize,
    pub backlog: i32
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/settings"))?;
        s.try_into()
    }
}