use super::{Product, ProductList};
use crate::gtin::GTIN;

pub struct Brocade {
    client: reqwest::Client,
}

pub(super) static BASE_URL: &str = "https://www.brocade.io/products";

impl Brocade {
    pub fn new() -> Brocade {
        // Create headers for the client.
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert("accept", "application/json".parse().unwrap());

        // Create the reqwest client.
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Brocade { client }
    }

    pub async fn get_product_async(&self, gtin: GTIN) -> reqwest::Result<Product> {
        self.client
            .get(format!("{BASE_URL}/{gtin}"))
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_product_list_async(&self) -> reqwest::Result<ProductList> {
        self.client.get(BASE_URL).send().await?.json().await
    }

    pub async fn query_product_async(&self, query: &str) -> reqwest::Result<ProductList> {
        self.client
            .get(format!("{BASE_URL}?query={query}"))
            .send()
            .await?
            .json()
            .await
    }
}

impl Default for Brocade {
    fn default() -> Self {
        Self::new()
    }
}
