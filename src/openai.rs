use anyhow::Result;
use reqwest::{
    blocking::{Client as ReqwestClient, Response},
    Method,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::config::Config;

const OPENAI_API_URL: &str = "https://api.openai.com/v1";

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChoice {
    pub text: String,
    pub index: i32,
    pub logprobs: Option<Value>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
}

pub struct OpenAIClient {
    config: Config,
    client: ReqwestClient,
}

impl OpenAIClient {
    pub fn new(config: Config) -> Self {
        let client = ReqwestClient::builder().build().unwrap();

        Self { config, client }
    }

    pub fn completion(&self, prompt: &str) -> Result<ChatCompletionResponse> {
        let resp = self
            .send_request(
                "completions",
                Method::POST,
                json!({
                   "model": &self.config.openai_mod,
                   "prompt": prompt,
                   "temperature": 0,
                   "stop": ["#", "```"],
                }),
            )?
            .json::<ChatCompletionResponse>()?;

        Ok(resp)
    }

    fn send_request(&self, url: &str, method: Method, body: Value) -> Result<Response> {
        let resp = self
            .client
            .request(method, format!("{OPENAI_API_URL}/{url}").as_str())
            .bearer_auth(&self.config.openai_key)
            .json(&body)
            .send()?;

        Ok(resp)
    }
}
