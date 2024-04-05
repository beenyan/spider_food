use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

const API_KEY: &'static str = "d367c3b86844a4da0cae31b54fff4ee8";

#[derive(Debug, Serialize, Deserialize)]
pub struct DateInfo {
    weekday: String,
    hour: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Payment {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderType {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentConfig {
    payment: Vec<Payment>,
    order_type: Vec<OrderType>,
}

impl PaymentConfig {
    pub fn get_payment(&self) -> Option<Vec<String>> {
        match self.payment.is_empty() {
            true => None,
            false => Some(
                self.payment
                    .iter()
                    .map(|py| py.name.clone())
                    .collect::<Vec<_>>(),
            ),
        }
    }

    pub fn get_order(&self) -> HashSet<&str> {
        self.order_type.iter().map(|o| o.name.as_str()).collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct InfoApi {
    pub business_hour: HashMap<String, Vec<DateInfo>>,
    pub payment_config: PaymentConfig,
    pub phone: String,
}

impl InfoApi {
    pub async fn fetch(identifier: &str) -> Result<Option<Self>> {
        let client = reqwest::Client::new();
        let url = format!("https://istore.weibyapps.com/api/v2/iorder/stores/{identifier}");
        let response = client
            .get(&url)
            .header("X-Api-Key", API_KEY)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(data) = response.get("data") {
            if let Ok(info_api) = serde_json::from_value::<InfoApi>(data.clone()) {
                return Ok(Some(info_api));
            }
        }

        Ok(None)
    }
}
