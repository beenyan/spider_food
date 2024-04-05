use super::product::Product;
use crate::utils::{
    deserialize::{deserialize_bool, str_to_number},
    is_time,
};
use anyhow::Result;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

const RESTAURANT_URL: &'static str =
    "https://eats.quickclick.cc/apis/portals/gofoodie/shops?category=catering&is_to_go=undefined";

#[derive(Debug, Deserialize)]
pub struct Restaurant {
    id: usize,
    name: String,
    lat: Option<String>,
    lng: Option<String>,
    address: Option<String>,
    info: Option<String>,
    // 外送
    #[serde(deserialize_with = "deserialize_bool", alias = "isDelivery")]
    is_delivery: bool,
    // 外帶
    #[serde(deserialize_with = "deserialize_bool", alias = "isTakeout")]
    is_take_out: bool,
    // 內用
    #[serde(deserialize_with = "deserialize_bool", alias = "isEatIn")]
    is_eat_in: bool,
    // 內用平均等待時間
    #[serde(deserialize_with = "str_to_number", alias = "eatInWaitingTime")]
    eat_in_waiting_time: isize,
    // 外帶平均等待時間
    #[serde(deserialize_with = "str_to_number", alias = "toGoWaitingTime")]
    to_go_waiting_time: isize,
    // 外送平均等待時間
    #[serde(deserialize_with = "str_to_number", alias = "deliveryWaitingTime")]
    delivery_waiting_time: isize,
    #[serde(deserialize_with = "deserialize_bool", alias = "isEnabled")]
    is_enabled: bool,
    image: Option<String>,
    #[serde(alias = "paymentMethods")]
    payment_methods: Option<String>,
    #[serde(deserialize_with = "deserialize_bool", alias = "isCash")]
    is_cash: bool,
    #[serde(default)]
    menu: Vec<Product>,
}

impl Serialize for Restaurant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Restaurant", 13)?;
        let store_link = format!("https://eats.quickclick.cc/togo/{}", self.id);
        state.serialize_field("name", &self.name)?;
        state.serialize_field("image_url", &self.image)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("is_delivery", &self.is_delivery)?;
        state.serialize_field("is_take_out", &self.is_take_out)?;
        state.serialize_field("is_eat_in", &self.is_eat_in)?;
        state.serialize_field("eat_in_waiting_time", &self.eat_in_waiting_time)?;
        state.serialize_field("to_go_waiting_time", &self.to_go_waiting_time)?;
        state.serialize_field("delivery_waiting_time", &self.delivery_waiting_time)?;
        state.serialize_field("lat", &self.lat)?;
        state.serialize_field("lng", &self.lng)?;
        state.serialize_field("store_link", &store_link)?;
        state.serialize_field("is_enabled", &self.is_enabled)?;
        state.serialize_field("menu", &self.menu)?;

        let mut work_time = Vec::new();
        let info = self.info.clone().map(|s| {
            s.lines()
                .filter_map(|line| {
                    let trimmed_line = line.trim();
                    match trimmed_line.is_empty() {
                        true => None,
                        false => {
                            let trimmed_line = trimmed_line.to_owned();
                            if is_time(&trimmed_line) {
                                work_time.push(trimmed_line.to_owned())
                            }

                            Some(trimmed_line)
                        }
                    }
                })
                .collect::<Vec<_>>()
        });
        state.serialize_field("work_time", &work_time)?;
        state.serialize_field("info", &info)?;

        match &self.payment_methods {
            Some(payment_methods) => {
                let mut payment_methods: Vec<&str> = payment_methods
                    .split(',')
                    .filter(|&s| !s.is_empty())
                    .collect();

                if self.is_cash {
                    payment_methods.push("cash");
                }

                state.serialize_field("payment_methods", &payment_methods)?;
            }
            None => state.serialize_field("payment_methods", &Vec::<bool>::new())?,
        }

        state.end()
    }
}

impl Restaurant {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_is_enabled(&self) -> bool {
        self.is_enabled
    }

    pub fn get_store_link(&self) -> String {
        format!("https://eats.quickclick.cc/togo/{}", self.id)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_menu(&mut self, menu: Vec<Product>) {
        self.menu = menu
    }
}

impl Restaurant {
    pub async fn fetch_all() -> Result<Vec<Restaurant>> {
        let response = reqwest::get(RESTAURANT_URL).await?.text().await?;

        Ok(serde_json::from_str::<Vec<Restaurant>>(&response)?)
    }
}
