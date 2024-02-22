use clap::Parser;
use fuel_core_client::client::FuelClient;
use fuel_core_types::{
    blockchain::primitives::BlockId, fuel_tx::Transaction, fuel_types::BlockHeight,
};
use prost::Message;

mod cli;
mod types;

use types::{Block, TxExtra};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = cli::Opt::parse();

    let client = FuelClient::new(opt.fuel_url)?;

    let mut height: BlockHeight = opt.height.into();

    loop {
        if let Some(block) = client.block_by_height(height.into()).await? {
            let prev_id: BlockId = match height.pred() {
                Some(h) => client
                    .block_by_height(h.into())
                    .await?
                    .map(|b| b.id.into())
                    .unwrap_or_default(),
                None => BlockId::default(),
            };

            let mut tx_data: Vec<Transaction> = vec![];
            let mut tx_extra: Vec<TxExtra> = vec![];
            for id in &block.transactions {
                let tx = client.transaction(id).await?.unwrap();
                tx_data.push(tx.transaction);
                let receipts = client.receipts(id).await?;
                tx_extra.push(TxExtra {
                    id: (*id).into(),
                    receipts: receipts.unwrap_or_default().to_vec(),
                });
            }

            let fire_block =
                Block::from((&block, prev_id, tx_data.as_slice(), tx_extra.as_slice()));
            let out_msg = hex::encode(fire_block.encode_to_vec());
            println!("FIRE PROTO {}", out_msg);

            height = height.succ().expect("Max height reached.");
        } else if opt.stop {
            break;
        } else {
            tokio::time::sleep(opt.poll.into()).await;
        }
    }
    Ok(())
}
