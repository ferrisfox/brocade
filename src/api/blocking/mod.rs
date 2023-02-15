use super::{Product, ProductList};
use crate::gtin::GTIN;

pub struct Brocade {
    client: reqwest::blocking::Client,
}

static BASE_URL: &str = super::brocade::BASE_URL;

impl Brocade {
    pub fn new() -> Brocade {
        // Create headers for the client.
        let mut headers = reqwest::header::HeaderMap::with_capacity(1);
        headers.insert("accept", "application/json".parse().unwrap());

        // Create the reqwest client.
        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Brocade { client }
    }

    pub fn get_product_async(&self, gtin: GTIN) -> reqwest::Result<Product> {
        self.client.get(format!("{BASE_URL}/{gtin}")).send()?.json()
    }

    pub fn get_product_list_async(&self) -> reqwest::Result<ProductList> {
        self.client.get(BASE_URL).send()?.json()
    }

    pub fn query_product_async(&self, query: &str) -> reqwest::Result<ProductList> {
        self.client
            .get(format!("{BASE_URL}?query={query}"))
            .send()?
            .json()
    }
}

impl Default for Brocade {
    fn default() -> Self {
        Self::new()
    }
}
