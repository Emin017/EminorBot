use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

const DEEPSEEK_API_URL: &str = "https://api.deepseek.com/chat/completions";

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct DeepSeekRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f64,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct DeepSeekResponse {
    choices: Vec<Choice>,
}

pub struct DeepSeekClient {
    client: Client,
    api_key: String,
    model: String,
}

impl DeepSeekClient {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let api_key = env::var("DEEPSEEK_API_KEY")
            .map_err(|_| "DEEPSEEK_API_KEY not found in environment variables")?;

        let model = env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string());

        Ok(Self {
            client: Client::new(),
            api_key,
            model,
        })
    }

    pub async fn send_message(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        log::info!(
            "Sending request to DeepSeek API with model: {}",
            &self.model
        );

        let request = DeepSeekRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: "You are a helpful assistant.".to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: 0.7,
        };

        let response = self.send_api_request(&request).await?;

        Ok(response)
    }

    async fn send_api_request(
        &self,
        request: &DeepSeekRequest,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", self.api_key))?,
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let response = self
            .client
            .post(DEEPSEEK_API_URL)
            .headers(headers)
            .json(request)
            .send()
            .await
            .map_err(|e| format!("Failed to send API request: {}", e))?;

        match !response.status().is_success() {
            true => {
                let error_text = response.text().await?;
                return Err(format!("API returned error: {}", error_text).into());
            }
            false => {}
        }

        let response_data: DeepSeekResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Get response content
        match response_data.choices.first() {
            Some(choice) => Ok(choice.message.content.clone()),
            None => Err("No response content received".into()),
        }
    }
}
