use anyhow::Result;
use clap::Parser;

use crate::config::{set_config, Config};

#[derive(Debug, Parser)]
pub struct Login {
    #[clap(short, long)]
    key: String,

    #[clap(short, long, default_value_t = String::from("code-davinci-002"))]
    model: String,
}

pub fn login(args: Login) -> Result<()> {
    set_config(Config {
        openai_key: args.key,
        openai_mod: args.model,
    })?;

    Ok(())
}
