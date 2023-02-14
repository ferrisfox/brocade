#![allow(clippy::missing_errors_doc)]

use std::ops::Deref;

use serde::{Serialize, Deserialize};

pub mod gtin;

pub struct Brocade {
}

impl Brocade {
    pub async fn get_item_async(&self, gtin: gtin::GTIN) -> reqwest::Result<Item> {
        reqwest::get(&format!("https://www.brocade.io/api/items/{gtin}")).await?
            .json().await
    }
}

#[derive(Debug)]
pub struct BrocadeError;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Item {
    pub gtin14: gtin::GTIN,
    pub brand_name: Option<String>,
    pub name: Option<String>,
}

pub struct ItemList(Vec<Item>);

impl Deref for ItemList {
    type Target = Vec<Item>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_item() {
        let brocade = Brocade {};
        let item = brocade.get_item_async("00074887615305".parse().unwrap()).await.unwrap();
        assert_eq!(item, Item {
            gtin14: "00074887615305".parse().unwrap(),
            brand_name: Some("Up & Up".to_string()),
            name: Some("Exfoliating Cucumber Cleansing Towelettes".to_string()),
        });
    }

    #[tokio::test]
    async fn test_get_item_with_null() {
        let brocade = Brocade {};
        let item = brocade.get_item_async("09780262134729".parse().unwrap()).await.unwrap();
        assert_eq!(item, Item {
            gtin14: "09780262134729".parse().unwrap(),
            brand_name: None,
            name: Some("The Laws Of Simplicity".to_string()),
        });
    }
}