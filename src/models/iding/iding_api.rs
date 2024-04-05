use super::restaurant::Restaurant;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct IdingApi {
    all_stores: HashMap<String, Restaurant>,
    city: Vec<Value>,
}

impl IdingApi {
    pub fn is_empty(&self) -> bool {
        self.city.is_empty()
    }

    pub fn get_restaurants(&mut self) -> Vec<Restaurant> {
        let restaurants = self
            .all_stores
            .drain()
            .map(|(_, v)| v)
            .collect::<Vec<Restaurant>>();

        restaurants
    }
}
