mod pb;

use pb::sf::fuel::r#type::v1::{ Block };
use crate::pb::example::{ Receipts, Transactions };
use hex::encode;

#[substreams::handlers::map]
fn map_blocks(block: Block) -> Result<Block, substreams::errors::Error> {
    Ok(block)
}



#[substreams::handlers::map]
fn map_transactions_from_blocks(block: Block) -> Result<Transactions, substreams::errors::Error> {
    Ok(Transactions { block_id: encode(&block.id), transaction: block.transactions.into_iter().collect() })
}

// todo: Implement the decode_receipts_for_contract function
#[substreams::handlers::map]
pub fn decode_receipts_for_contract(contract_id: String, transactions: Transactions) -> Result<Option<Receipts>, substreams::errors::Error> {

    // Contract_id "id": "ZtjPHAcXvw8DDoTuDC/A8JZb9HM3xSb9Tas+zW6K0PA=",
    // Log_id/Type_id"rb": "13",

    let receipts = transactions.transaction.into_iter().find_map(|tx|
        if encode(&tx.id) == contract_id {

            // decoder.decode_as_debug_str(
            //     &param_type,
            //     &[0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 10]
            // )?

            // let json_abi_file =
            //     "/Users/salka1988/Documents/Git/substreams-fuel/contracts_abis/b81f1c4b9c408cb7d573cfcfe78878f671124322e17da233f047a4cc2372f9d7.json";
            //
            // let abi_file_contents = std::env::current_dir().expect("Failed to get current directory").file_name()
            //     .expect("Failed to get file name").to_str().expect("Failed to convert to string").to_string();


            // let abi_file_contents = std::fs::read_to_string(json_abi_file).expect("Failed to read ABI file");

            // let abi_file_contents = match std::fs::read_to_string(&json_abi_file) {
            //     Ok(contents) => contents,
            //     Err(err) => {
            //         "ars".to_string()
            //     }
            // };

            // let parsed_abi: ProgramABI = serde_json::from_str(&abi_file_contents).expect("Failed to parse ABI");
            //
            // let type_lookup = parsed_abi
            //     .types
            //     .into_iter()
            //     .map(|decl| (decl.type_id, decl))
            //     .collect::<HashMap<_, _>>();

            // let logged_type = parsed_abi.logged_types;
            // dbg!(logged_type);

            Some(Receipts { transaction_id: encode(&tx.id), receipts: tx.receipts})
            // Some(Receipts { transaction_id: b, receipts: tx.receipts})
        } else { None }
    );

    Ok(receipts)
}
