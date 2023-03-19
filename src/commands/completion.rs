use anyhow::Result;
use clap::Parser;

use crate::config::ensure_config;
use crate::openai::{ChatCompletionResponse, OpenAIClient};

#[derive(Debug, Parser)]
pub struct Completion {
    #[clap(num_args(0..))]
    input: Vec<String>,
}

const PROMPT: &str = "# find all typescript files in the current directory
find . -name '*.ts'

# create a directory called app
mkdir app

# push the current branch to the remote
git push -u origin HEAD

# print out the contents of the current directory
ls

# restart the Dock application
killall Dock

# create a new cargo project called proj
cargo new proj

# install the express package from npm
pnpm add express

# create a file called cool.html
touch cool.html

# {{input}}";

pub fn completion(args: Completion) -> Result<ChatCompletionResponse> {
    let cfg = ensure_config()?;

    let client = OpenAIClient::new(cfg);
    let input = args.input.join(" ");
    let prompt = PROMPT.replace("{{input}}", &input);
    let resp = client.completion(&prompt)?;

    Ok(resp)
}
