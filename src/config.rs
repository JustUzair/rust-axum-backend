use core::panic;
use std::{env, sync::OnceLock};

use crate::{Error, Result};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|exception| {
            panic!("FATAL: Couldn't load config\n\n Reason:\n {:?}", exception)
        })
    })
}
#[allow(non_snake_case)]
pub struct Config {
    pub WEB_FOLDER: String,
    pub DB_URL: String,
}
impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}
fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
