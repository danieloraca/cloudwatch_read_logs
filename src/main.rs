use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatchlogs::{Client, Error};
use aws_types::region::Region;
use std::fs::File;
use std::io::Write;

struct LogQuery {
    logs: Vec<String>,
}

impl LogQuery {
    fn new(logs: Vec<String>) -> Self {
        LogQuery { logs }
    }

    fn search(&self, keyword: &str) -> Vec<&String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        let matching_logs = self.search(keyword);

        let mut file = File::create(path)?;

        for log in matching_logs {
            writeln!(file, "{}", log)?;
        }

        Ok(())
    }
}

#[::tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else(Region::new("us-east-1"));
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let log_group_name = "/aws/lambda/Test-Authorizer-Kris";

    let start_time = 1625835600000; // Example timestamp in milliseconds
    let end_time = 1626922000000;

    let response = client
        .filter_log_events()
        .log_group_name(log_group_name)
        .start_time(start_time)
        .end_time(end_time)
        .send()
        .await?;

    let events = response.events.unwrap_or_default();

    let logs: Vec<String> = events
        .iter()
        .map(|event| event.message.clone().unwrap_or_default())
        .collect();

    let log_query = LogQuery::new(logs);

    log_query.export_to_file("ERROR", "error_logs.txt").unwrap();

    println!("Done!");

    Ok(())
}
