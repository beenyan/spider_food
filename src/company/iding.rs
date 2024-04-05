use crate::models::iding::{info_api::InfoApi, menu_api::MenuApi, restaurant::Restaurant};
use anyhow::Result;
use futures::future::join_all;
use serde::Serialize;
use serde_json::json;
use std::{
    fs::{self, File},
    io::Write,
};

fn output<T>(value: &T) -> Result<()>
where
    T: ?Sized + Serialize,
{
    fs::create_dir_all("./output/")?;
    let mut file = File::create("./output/iding.json")?;
    file.write(serde_json::to_string(&value)?.as_bytes())?;

    Ok(())
}

async fn set_detail(restaurant: &mut Restaurant) {
    restaurant.store_link = format!("https://iding.tw/stores/{}/menu", restaurant.identifier);

    // 餐廳資訊
    match InfoApi::fetch(&restaurant.identifier).await {
        Ok(Some(info)) => {
            restaurant.work_time = info.business_hour;
            restaurant.payment_methods = info.payment_config.get_payment();
            let order_type = info.payment_config.get_order();

            if order_type.contains("dining_in") {
                restaurant.is_eat_in = true;
            }

            if order_type.contains("delivery") {
                restaurant.is_delivery = true;
            }

            if order_type.contains("to go") {
                restaurant.is_take_out = true;
            }

            restaurant.phone = match info.phone.is_empty() {
                true => None,
                false => Some(info.phone),
            };
        }
        Err(err) => eprintln!(
            "\nIding Get Info Failed: \n{:#?}, {:#?}",
            json!({"name": restaurant.name,"store_link": restaurant.store_link}),
            err
        ),
        _ => {}
    }

    // 餐廳菜單
    match MenuApi::fetch_all(&restaurant.identifier).await {
        Ok(menu) => restaurant.menu = menu,
        Err(err) => eprintln!(
            "\nIding Get Menu Failed: \n{:#?}, {:#?}",
            json!({"name": restaurant.name,"store_link": restaurant.store_link}),
            err
        ),
    }
}

pub async fn fetch_all(chunk_size: usize) -> Result<()> {
    let mut all_restaurants: Vec<Restaurant> = Vec::new();
    let mut page = 1;
    while let Some(mut restaurants) = Restaurant::fetch_by_page(page).await? {
        println!("Page: {page}");

        let total = restaurants.len();
        let mut process = 0;
        for restaurants in restaurants.chunks_mut(chunk_size) {
            process += restaurants.len();
            let futures = restaurants
                .iter_mut()
                .map(|restaurant| set_detail(restaurant))
                .collect::<Vec<_>>();
            join_all(futures).await;

            println!("\tIding Menu: {process} / {total}");
        }

        all_restaurants.append(&mut restaurants);
        page += 1;
    }

    output(&all_restaurants)?;
    println!("Finish Fetch Iding");
    Ok(())
}
