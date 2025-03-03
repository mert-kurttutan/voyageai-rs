use core::fmt;

use reqwest::Client as RwClient;

use crate::error::Result;

const BASE_URL: &str = "https://api.voyageai.com";

#[non_exhaustive]
pub enum Capabilities {
    Embedding,
    MultimodalEmbedding,
    Reranker,
}


pub struct Client {
    client: RwClient,
    base_url: String,
    api_key: String,
}


pub(crate) const SUPPORTED_MODELS: [&str; 11] = [
    "voyage-large-2-instruct",
    "voyage-law-2",
    "voyage-code-2",
    "voyage-02",
    "voyage-2",
    "voyage-01",
    "voyage-lite-01",
    "voyage-lite-01-instruct",
    "voyage-lite-02-instruct",
    "voyage-multilingual-2",
    "voyage-large-2",
];
impl Client {
    pub fn new(api_key: &str) -> Self {
        Client {
            client: RwClient::new(),
            api_key: api_key.to_string(),
            base_url: format!("{}/v1", BASE_URL),
        }
    }
    pub async fn embed(&self, input_text: &[&str], model: &str) -> Result<String> {

        if !SUPPORTED_MODELS.contains(&model) {
            return Err(crate::error::Error{
                kind: crate::error::ErrorKind::InvalidModelName(model.to_string())
            });
        }
        
        let url = format!("{}/embedding", self.base_url);
        use serde_json::json;
        let json_data = json!({
            "input": input_text,
            "model": model
        });

        let mut data_map_1 = std::collections::HashMap::new();
        let mut data_map_2 = std::collections::HashMap::new();
        data_map_1.insert("input", input_text);
        data_map_2.insert("model", model);
        let response = self.client.post(&url).bearer_auth(&self.api_key).body(json_data.to_string()).send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    // pub async fn multimodal_embed(&self, )
}


#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_get() {
        dotenv().ok();
        let api_key = std::env::var("VOYAGE_API_KEY").unwrap();
        let client = Client::new(&api_key);
        let model = "voyage-01";
        let response = client.embed(&["test"], model).await.unwrap();
        // assert_eq!(response, "test");
    }
}
