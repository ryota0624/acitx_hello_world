use config::{ConfigError, Config, File, Environment};
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
            _ => Err(serde::de::Error::custom(format!("not available pattern -> {}", s)))
        }
    }
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server_name: String,
    #[serde(deserialize_with="ServerMode::deserialize_with")]
    pub mode: ServerMode,
    pub server: ServerSettings,
    pub hello_server_host: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub timeout: usize,
    pub workers: usize,
    pub backlog: i32
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env_s = Environment::new();
        let file_s =File::with_name("config/settings");
        let mut conf = Config::new();
        conf
            .merge(file_s)?
            .merge(env_s)?;

        conf.try_into()
    }
}

#[derive(Clone)]
pub struct GlobalSetting(pub Settings);

lazy_static! {
    pub static ref global_settings: GlobalSetting = {
        GlobalSetting(Settings::new().expect("fail create settings"))
    };
}

pub trait UseSetting {
    fn settings(&self) -> Settings;
}

pub struct SettingProvider;

impl UseSetting for SettingProvider {
    fn settings(&self) -> Settings {
        global_settings.0.clone()
    }
}