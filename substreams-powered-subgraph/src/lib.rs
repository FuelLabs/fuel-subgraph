mod pb;

use pb::example::Block;
use pb::sf::fuel::r#type::v1::{Block as FuelBlock};

use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
fn map_blocks(block: FuelBlock) -> Result<Block, substreams::errors::Error> {
    Ok(Block {
        id: block.id,
        height: block.height,
        da_height: block.da_height,
        msg_receipt_count: block.msg_receipt_count,
        tx_root: block.tx_root,
        msg_receipt_root: block.msg_receipt_root,
        prev_id: block.prev_id,
        prev_root: block.prev_root,
        timestamp: block.timestamp,
        application_hash: block.application_hash,
    })
}

#[substreams::handlers::map]
pub fn graph_out(block: Block) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    tables
        .create_row("Block", format!("0x{}", Hex(&block.id)))
        .set("timestamp", block.timestamp)
        .set("blockNumber", block.height);


    Ok(tables.to_entity_changes())
}
