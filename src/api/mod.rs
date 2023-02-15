use std::ops::Deref;

use serde::{Deserialize, Serialize};

mod brocade;
pub use brocade::Brocade;

#[cfg(feature = "blocking")]
pub mod blocking;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Product {
    pub gtin14: String,
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
        let item = brocade
            .get_product_async("00074887615305".parse().unwrap())
            .await
            .unwrap();
        assert_eq!(
            item,
            Product {
                gtin14: "00074887615305".parse().unwrap(),
                brand_name: Some("Up & Up".to_string()),
                name: Some("Exfoliating Cucumber Cleansing Towelettes".to_string()),
            }
        );
    }

    #[tokio::test]
    async fn test_get_item_with_null() {
        let brocade = Brocade::new();
        let item = brocade
            .get_product_async("09780262134729".parse().unwrap())
            .await
            .unwrap();
        assert_eq!(
            item,
            Product {
                gtin14: "09780262134729".parse().unwrap(),
                brand_name: None,
                name: Some("The Laws Of Simplicity".to_string()),
            }
        );
    }

    #[tokio::test]
    async fn test_get_product_list() {
        let brocade = Brocade::new();
        let items = brocade.get_product_list_async().await.unwrap();
        assert_eq!(items.len(), 100);
    }
}
