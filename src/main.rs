mod company;
mod models;
mod utils;

use crate::utils::args::Args;
use anyhow::Result;
use futures::future::join;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::new();
    let batch_size = args.get_batch_size();
    println!("Batch Size: {:#?}", batch_size);
    println!("Fetch Start...");

    let (go_foodie, iding) = join(
        company::go_foodie::fetch_all(batch_size),
        company::iding::fetch_all(batch_size),
    )
    .await;
    go_foodie?;
    iding?;

    println!("All Question Finish...");

    Ok(())
}
