use super::{iding_api::IdingApi, info_api::DateInfo, product::Product};
use crate::utils::deserialize::bool_not;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Restaurant {
    #[serde(skip_serializing)]
    pub identifier: String,
    pub name: String,
    lat: Option<String>,
    lng: Option<String>,
    address: Option<String>,
    #[serde(default)]
    pub is_delivery: bool,
    #[serde(default)]
    pub is_take_out: bool,
    #[serde(default)]
    pub is_eat_in: bool,
    #[serde(deserialize_with = "bool_not", alias = "is_closed")]
    pub is_enabled: bool,
    #[serde(alias = "cover")]
    image: String,
    #[serde(default)]
    pub work_time: HashMap<String, Vec<DateInfo>>,
    #[serde(default)]
    pub payment_methods: Option<Vec<String>>,
    #[serde(default)]
    pub store_link: String,
    pub phone: Option<String>,
    #[serde(default)]
    pub menu: Vec<Product>,
}

impl Restaurant {
    pub async fn fetch_by_page(page: usize) -> Result<Option<Vec<Restaurant>>> {
        let restaurant_url = format!("https://iding.tw/api/stores?page={page}&per_page=10000");
        let mut response = reqwest::get(restaurant_url)
            .await?
            .json::<IdingApi>()
            .await?;

        Ok(match response.is_empty() {
            true => None,
            false => Some(response.get_restaurants()),
        })
    }
}
