use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatchlogs::{Client, Error};
use aws_types::region::Region;
use tokio::sync::{mpsc, RwLock};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let log_group_name = "/aws/lambda/Test-Authorizer-Kris";

    let start_time = 1626735600000; // Example timestamp in milliseconds
    let end_time = 1626822000000;

    let response = client
        .filter_log_events()
        .log_group_name(log_group_name)
        .start_time(start_time)
        .end_time(end_time)
        .send()
        .await?;

    for event in response.events.unwrap_or_default() {
        println!("Log: {}", event.message.unwrap_or_default());
    }

    println!("Done!");

    Ok(())
}
