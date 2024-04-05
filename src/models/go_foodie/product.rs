use anyhow::Result;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(alias = "productName")]
    name: String,
    #[serde(alias = "productAmount")]
    price: usize,
    #[serde(alias = "productImage")]
    food_image_url: String,
    #[serde(alias = "productDescription")]
    product_description: String,
    #[serde(default)]
    category_name: String,
}

impl Product {
    pub async fn get_shop_link(id: usize) -> Result<Option<String>> {
        let url = format!("https://eats.quickclick.cc/apis/shops/{id}");
        let response = &reqwest::get(url).await?.text().await?;
        let value = serde_json::from_str::<Value>(response)?;

        let link = match value.get("webLink") {
            Some(link) => link.as_str().map(|l| l.to_owned()),
            None => None,
        };

        Ok(link)
    }

    async fn get_product_link(id: usize) -> Result<Option<String>> {
        let product_link = match Product::get_shop_link(id).await? {
            Some(product_link) => product_link,
            None => return Ok(None),
        };
        let parsed_url = Url::parse(&product_link)?;
        let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();

        match hash_query.get("accountId") {
            Some(account) => Ok(Some(format!(
                "https://line.quickclick.cc/line/system/accounts/{account}/products"
            ))),
            None => Ok(None),
        }
    }

    pub async fn fetch_all(id: usize) -> Result<Vec<Product>> {
        let product_link = match Product::get_product_link(id).await? {
            Some(product_link) => product_link,
            None => return Ok(Vec::new()),
        };

        let response = reqwest::get(product_link).await?.text().await?;
        let categorys = serde_json::from_str::<Vec<Value>>(&response)?;
        let products = categorys
            .into_iter()
            .flat_map(|category| {
                let category_name = category
                    .get("categoryName")
                    .and_then(|name| name.as_str())
                    .unwrap_or("undefined")
                    .to_string();

                let products = category
                    .get("products")
                    .and_then(|products| {
                        let mut products =
                            serde_json::from_value::<Vec<Product>>(products.to_owned())
                                .unwrap_or(Vec::new());
                        for product in products.iter_mut() {
                            product.category_name = category_name.clone();
                        }

                        Some(products)
                    })
                    .unwrap_or(Vec::new());

                products
            })
            .collect::<Vec<_>>();

        Ok(products)
    }
}
