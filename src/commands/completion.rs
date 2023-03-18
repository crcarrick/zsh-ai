use anyhow::Result;
use clap::Parser;

use crate::config::ensure_config;
use crate::openai::{ChatCompletionResponse, OpenAIClient};

#[derive(Debug, Parser)]
pub struct Completion {
    #[clap(num_args(0..))]
    input: Vec<String>,
}

pub fn completion(args: Completion) -> Result<ChatCompletionResponse> {
    let cfg = ensure_config()?;

    let client = OpenAIClient::new(cfg);
    let prompt = format!("{}", args.input.join(" "));
    let resp = client.completion(&prompt)?;

    Ok(resp)
}
