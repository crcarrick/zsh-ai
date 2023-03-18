use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub openai_key: String,
    pub openai_mod: String,
}

const APP_NAME: &str = "com.github.crcarrick.zsh_ai";
const CONFIG_NAME: &str = "config";

fn get_config() -> Result<Config> {
    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME)?;

    Ok(cfg)
}

pub fn set_config(cfg: Config) -> Result<()> {
    confy::store(APP_NAME, CONFIG_NAME, cfg)?;

    Ok(())
}

pub fn ensure_config() -> Result<Config> {
    let cfg = get_config()?;
    if cfg.openai_key.is_empty() {
        return Err(anyhow!("You must login first"));
    }

    Ok(cfg)
}
