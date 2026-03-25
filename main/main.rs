use tokio::time::{sleep, Duration};
use mongodb::{Client, options::ClientOptions};

mod job;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let uri = std::env::var("MONGO_URL")?;

    let options = ClientOptions::parse(&uri).await?;
    let client = Client::with_options(options)?;

    loop {
        tracing::info!("Running monthly storage charge job...");

        if let Err(e) = job::run(&client).await {
            tracing::error!("Job failed: {:?}", e);
        }

        //pigumnov 24 h todo change to cron for some date in month
        sleep(Duration::from_secs(60 * 60 * 24)).await;
    }
}
