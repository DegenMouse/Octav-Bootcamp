use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub category: String,
}

impl Quote {
    pub async fn fetch() -> Result<Quote> {
        let client = reqwest::Client::new();
        let response: Vec<Quote> = client
            .get("https://api.api-ninjas.com/v1/quotes")
            .header("X-Api-Key", "voe5NiACO3j0AHTikHjSWg==wM8N4BTd6qPOSfGP")
            .send()
            .await?
            .json()
            .await?;
        Ok(response[0].clone())
    }
}
