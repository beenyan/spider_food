use crate::models::go_foodie::{product::Product, restaurant::Restaurant};
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
    let mut file = File::create("./output/go_foodie.json")?;
    file.write(serde_json::to_string(&value)?.as_bytes())?;

    Ok(())
}

async fn set_detail(restaurant: &mut Restaurant) {
    match Product::fetch_all(restaurant.get_id()).await {
        Ok(products) => restaurant.set_menu(products),
        Err(_) => {
            if restaurant.get_is_enabled() {
                eprintln!(
                    "\nGet Menu Failed: \n{:#?}",
                    json!({"name": restaurant.get_name(),"store_link": restaurant.get_store_link()})
                )
            }
        }
    }
}

pub async fn fetch_all(chunk_size: usize) -> Result<()> {
    let mut restaurants = Restaurant::fetch_all().await?;
    let total = restaurants.len();
    let mut process = 0;
    for restaurants in restaurants.chunks_mut(chunk_size) {
        process += restaurants.len();
        let futures = restaurants
            .iter_mut()
            .map(|restaurant| set_detail(restaurant))
            .collect::<Vec<_>>();
        join_all(futures).await;

        println!("\tGo Foodie Menu: {process} / {total}");
    }

    output(&restaurants)?;
    println!("Finish Fetch Go Foodie");
    Ok(())
}
