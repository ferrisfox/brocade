use std::ops::Deref;

use serde::{Serialize, Deserialize};

pub mod gtin;

pub struct Brocade {
    client: reqwest::Client,
}

static BASE_URL: &str = "https://www.brocade.io/products";

impl Brocade {
    pub fn new() -> Brocade {
        // Create headers for the client.
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert("accept", "application/json".parse().unwrap());
        
        // Create the reqwest client.
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build().unwrap();

        Brocade { client }
    }

    pub async fn get_product_async(&self, gtin: gtin::GTIN) -> reqwest::Result<Product> {
        self.client.get(format!("{BASE_URL}/{gtin}"))
            .send().await?
            .json().await
    }

    pub async fn get_product_list_async(&self) -> reqwest::Result<ProductList> {
        self.client.get(BASE_URL)
            .send().await?
            .json().await
    }

    pub async fn query_product_async(&self, query: &str) -> reqwest::Result<ProductList> {
        self.client.get(format!("{BASE_URL}?query={query}"))
            .send().await?
            .json().await
    }
}

impl Default for Brocade {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct BrocadeError;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Product {
    pub gtin14: gtin::GTIN,
    pub brand_name: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ProductList(Vec<Product>);

impl Deref for ProductList {
    type Target = Vec<Product>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: Test should not be dependent on the internet or specific data from the Brocade API.
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_item() {
        let brocade = Brocade::new();
        let item = brocade.get_product_async("00074887615305".parse().unwrap()).await.unwrap();
        assert_eq!(item, Product {
            gtin14: "00074887615305".parse().unwrap(),
            brand_name: Some("Up & Up".to_string()),
            name: Some("Exfoliating Cucumber Cleansing Towelettes".to_string()),
        });
    }

    #[tokio::test]
    async fn test_get_item_with_null() {
        let brocade = Brocade::new();
        let item = brocade.get_product_async("09780262134729".parse().unwrap()).await.unwrap();
        assert_eq!(item, Product {
            gtin14: "09780262134729".parse().unwrap(),
            brand_name: None,
            name: Some("The Laws Of Simplicity".to_string()),
        });
    }

    #[tokio::test]
    async fn test_get_product_list() {
        let brocade = Brocade::new();
        let items = brocade.get_product_list_async().await.unwrap();
        assert_eq!(items.len(), 100);
    }
}