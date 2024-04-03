import {arweave, BigInt, Bytes, fuel, Value} from "@graphprotocol/graph-ts"

import {
  Block,
  Create,
  Mint,
  Receipt,
  OutputContract,
  Policies,
  Script,
  Transaction,
  TxPointer,
  LogReceipt, LogDataReceipt, ScriptResultReceipt
} from "../generated/schema"


function saveReceipts(transaction_id: Bytes, receipts: fuel.Receipt[]): string[] {
  let recs: Array<string> = []

  for (let i  = 0; i < receipts.length; i++) {
    const receipt = receipts[i];
    let receipt_sc = new Receipt(transaction_id.toHexString());

    receipt_sc.kindCase = receipt.kind().toString()
    if (receipt.kind() == "log") {
      receipt_sc.id = receipt.log.id.toHexString()
      let receipt_log = new LogReceipt(transaction_id)
      receipt_log.id = receipt.log.id
      receipt_log.ra = BigInt.fromU64(receipt.log.ra)
      receipt_log.rb = BigInt.fromU64(receipt.log.rb)
      receipt_log.rc = BigInt.fromU64(receipt.log.rc)
      receipt_log.rd = BigInt.fromU64(receipt.log.rd)
      receipt_log.pc = BigInt.fromU64(receipt.log.pc)
      receipt_log.is = BigInt.fromU64(receipt.log.is)
      receipt_log.save()
      receipt_sc.log = receipt_log.id
    }

    if (receipt.kind() == "log_data") {
        receipt_sc.id = receipt.log_data.id.toHexString()
        let receipt_log_data = new LogDataReceipt(transaction_id)
        receipt_log_data.id = receipt.log_data.id
        receipt_log_data.ra = BigInt.fromU64(receipt.log_data.ra)
        receipt_log_data.rb = BigInt.fromU64(receipt.log_data.rb)
        receipt_log_data.ptr = BigInt.fromU64(receipt.log_data.ptr)
        receipt_log_data.len = BigInt.fromU64(receipt.log_data.len)
        receipt_log_data.digest = receipt.log_data.digest
        receipt_log_data.pc = BigInt.fromU64(receipt.log_data.pc)
        receipt_log_data.is = BigInt.fromU64(receipt.log_data.is)
        receipt_log_data.data = receipt.log_data.data
        receipt_log_data.save()
        receipt_sc.log_data = receipt_log_data.id
    }

    if (receipt.kind() == "script_result") {
      let receipt_script = new ScriptResultReceipt(transaction_id.toHexString())
      receipt_script.gas_used = BigInt.fromU64(receipt.script_result.gas_used)
      receipt_script.result = BigInt.fromU64(receipt.script_result.result)
      receipt_script.save()
      receipt_sc.script_result = receipt_script.id
    }

    receipt_sc.save();
    recs.push(receipt_sc.id)
  }

  return recs

}

function saveTransactions(id: Bytes, transactions: fuel.Transaction[]): string[] {

  let txs: Array<string> = []

  for (let i  = 0; i < transactions.length; i++) {

    const transaction = transactions[i];
    const transaction_sc = new Transaction(transaction.id.toHexString());

    transaction_sc.kindCase = transaction.kind().toString()
    transaction_sc.receipts = saveReceipts(transaction.id, transaction.receipts)

    if (transaction_sc.kindCase == "mint") {
      let mint = new Mint(transaction.id.toHexString())
      // mint.mint_asset_id = transaction.mint.mint_asset_id
      // mint.mint_amount = BigInt.fromU64(transaction.mint.mint_amount)
      //
      // let tx_pointer = new TxPointer(transaction.id.toHexString())
      // tx_pointer.block_height = BigInt.fromU32(transaction.mint.tx_pointer.block_height)
      // tx_pointer.tx_index = BigInt.fromU32(transaction.mint.tx_pointer.tx_index)
      //
      // let output_contract = new OutputContract(transaction.id.toHexString())
      // output_contract.input_index = BigInt.fromU32(transaction.mint.output_contract.input_index)
      // output_contract.balance_root = transaction.mint.output_contract.balance_root
      // output_contract.state_root = transaction.mint.output_contract.state_root
      //
      // mint.tx_pointer = tx_pointer.id
      // mint.output_contract = output_contract.id
      // transaction_sc.mint = mint.id
      //
      // let policies = new Policies(transaction.id.toHexString())
      // policies.values = "example_value".toString()
      //
      // mint.policies = policies.id
      //
      // policies.save()
      // output_contract.save()
      // tx_pointer.save()
      mint.save()
      transaction_sc.mint = mint.id
    }

    if (transaction_sc.kindCase == "script") {
      let script = new Script(transaction.id.toHexString());

      script.save()
      transaction_sc.script = script.id
    }

    if (transaction_sc.kindCase == "create") {
      let create = new Create(transaction.id.toHexString());


      create.save()
      transaction_sc.create = create.id
    }

    transaction_sc.save();
    txs.push(transaction_sc.id)
  }


  return txs
}


export function handleBlock(block: fuel.Block): void {

  let entity = new Block(block.id)
  entity.id = block.id
  entity.height = BigInt.fromU64(block.height as u64)
  entity.da_height = BigInt.fromU64(block.timestamp)
  entity.msg_receipt_count = BigInt.fromU64(block.timestamp)
  entity.tx_root = block.tx_root
  entity.msg_receipt_root = block.msg_receipt_root
  entity.prev_id = block.prev_id
  entity.prev_root = block.prev_root
  entity.timestamp = BigInt.fromU64(block.timestamp)
  entity.application_hash = block.application_hash
  entity.transactions = saveTransactions(block.id, block.transactions)

  entity.save()
}