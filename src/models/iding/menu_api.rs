use super::product::Product;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Categories {
    index: usize,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct MenuApi {
    photo_url: String,
    categories: Vec<Categories>,
    entrees: Vec<Product>,
}

impl MenuApi {
    pub async fn fetch_all(identifier: &str) -> Result<Vec<Product>> {
        let product_link = format!("https://iding.tw/api/menu/new/{identifier}");
        let mut response = reqwest::get(product_link).await?.json::<MenuApi>().await?;
        let mut index_name_map: HashMap<usize, String> = HashMap::new();
        for entry in &response.categories {
            index_name_map.insert(entry.index, entry.name.to_string());
        }

        for product in response.entrees.iter_mut() {
            if let Some(url) = &product.food_image_url {
                if !url.starts_with("http") {
                    product.food_image_url = Some(format!("{}/{url}", response.photo_url));
                }
            }

            product.category_name = index_name_map.get(&product.category_index).cloned();
        }

        Ok(response.entrees)
    }
}
