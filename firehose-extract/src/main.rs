use clap::Parser;
use cynic::QueryBuilder;
use fuel_core_client::client::{
    pagination::{PageDirection, PaginationRequest},
    FuelClient,
};
use fuel_core_client_ext::FullBlocksQuery;
use fuel_core_types::{blockchain::primitives::BlockId, fuel_types::BlockHeight};
use prost::Message;

mod cli;
mod types;

use types::Block;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = cli::Opt::parse();

    let client = FuelClient::new(opt.fuel_url)?;

    let start_height: BlockHeight = opt.height.into();

    // Get the first prev_id as a special case
    let prev_id: BlockId = match start_height.pred() {
        Some(h) => client
            .block_by_height(h.into())
            .await?
            .map(|b| b.id.into())
            .unwrap_or_default(),
        None => BlockId::default(),
    };
    let mut cursor = Some((*start_height).to_string());

    loop {
        let page_req = PaginationRequest::<String> {
            cursor,
            results: 64,
            direction: PageDirection::Forward,
        };
        let query = FullBlocksQuery::build(page_req.into());
        let resp = client.query(query).await?;
        cursor = resp.blocks.page_info.end_cursor;

        for block in resp.blocks.edges.iter() {
            let block = &block.node;

            let fire_block = Block::from((block, prev_id));
            let out_msg = hex::encode(fire_block.encode_to_vec());
            println!("FIRE PROTO {}", out_msg);
        }

        if !resp.blocks.page_info.has_next_page {
            if opt.stop {
                break;
            } else {
                tokio::time::sleep(opt.poll.into()).await;
            }
        }
    }
    Ok(())
}
