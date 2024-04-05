use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    name: String,
    price: f64,
    #[serde(skip_serializing)]
    pub category_index: usize,
    #[serde(alias = "photo")]
    pub food_image_url: Option<String>,
    #[serde(alias = "photo_note")]
    product_description: Option<String>,
    #[serde(default)]
    pub category_name: Option<String>,
}

impl Product {}
