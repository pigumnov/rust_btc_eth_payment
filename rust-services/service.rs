async fn process_channel(
    channel: Channel,
    wallets: mongodb::Collection<Wallet>,
) -> anyhow::Result<()> {

    let owner = channel.owner.to_lowercase();
    let wallet = wallets
        .find_one(doc! { "UserId": &owner }, None)
        .await?;

    let wallet = match wallet {
        Some(w) => w,
        None => return Ok(()),
    };

    let usage = calculate_user_storage(&channel.channel_id).await?;
    let balance = get_wallet_balance(&wallet.address).await?;

    if usage.exceeded_gb > 0 {
        let charge = usage.exceeded_gb * 500;

        let mut new_blocked = false;

        if balance >= charge {
            charge_dbl(charge, &wallet.address).await?;
        } else {
            new_blocked = true;
        }
        //pigumnov update only one item in row
        wallets.database().collection::<Channel>("Channels")
            .update_one(
                doc! { "ChannelId": &channel.channel_id },
                doc! { "$set": { "StorageBlocked": new_blocked } },
                None
            ).await?;
    }
    Ok(())
}
//pigumnov for tests
struct Usage {
    exceeded_gb: i32,
}

async fn calculate_user_storage(_channel_id: &str) -> anyhow::Result<Usage> {
    Ok(Usage { exceeded_gb: 1 })
}

async fn get_wallet_balance(_address: &str) -> anyhow::Result<i32> {
    Ok(10000)
}

async fn charge_dbl(_amount: i32, _address: &str) -> anyhow::Result<()> {
    Ok(())
}
