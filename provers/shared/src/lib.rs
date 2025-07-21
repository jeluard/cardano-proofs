use pallas_network::{facades::PeerClient, miniprotocols::Point};

pub async fn get_block() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let hash = std::env::var("BLOCK_HASH")?;
    let hash = hex::decode(hash)?;
    let mut peer_client = PeerClient::connect("preprod-node.play.dev.cardano.org:3001", 1).await?;
    let client = peer_client.blockfetch();
    let slot = std::env::var("BLOCK_SLOT")?;
    client.fetch_single(Point::Specific(slot.parse()?, hash)).await.map_err(Into::into)
}