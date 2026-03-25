use mongodb::{bson::doc,Client};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    #[serde(rename = "ChannelId")]
    channel_id: String,
    #[serde(rename = "Owner")]
    owner: String,
    #[serde(rename = "StorageBlocked")]
    storage_blocked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wallet {
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "Address")]
    address: String,
}

pub async fn run(client: &Client) -> anyhow::Result<()> {
    let db = client.database("vooly");
    let channels = db.collection::<Channel>("Channels");
    let wallets = db.collection::<Wallet>("wallets");
    let mut cursor = channels.find(None, None).await?;
    let mut tasks = vec![];

    while let Some(channel) = cursor.next().await {
        let channel = channel?;
        let wallets = wallets.clone();
        tasks.push(tokio::spawn(async move {
            process_channel(channel, wallets).await
        }));
    }
    //pigumnov set only 20 parralel chunks
    for chunk in tasks.chunks(20) {
        for task in chunk {
            let _ = task.await;
        }
    }
    Ok(())
}
