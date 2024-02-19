import {arweave, BigInt, Bytes, fuel, Value} from "@graphprotocol/graph-ts"

import {Block, Create, Mint, OutputContract, Policies, Script, Transaction, TxPointer} from "../generated/schema"

//
function saveTransactions(id: Bytes, transactions: fuel.Transaction[]): string[] {

  for (let i  = 0; i < transactions.length; i++) {

    const transaction = transactions[i];
    const transaction_sc = new Transaction(id.toHexString());

    transaction_sc.kindCase = transaction.kind().toString()

    if (transaction.kind() == "mint") {

      let mint = new Mint(id.toHexString())
      mint.mint_asset_id = transaction.mint.mint_asset_id
      mint.mint_amount = BigInt.fromU64(transaction.mint.mint_amount)

      let tx_pointer = new TxPointer(id.toHexString())
      tx_pointer.block_height = BigInt.fromU32(transaction.mint.tx_pointer.block_height)
      tx_pointer.tx_index =  BigInt.fromU32(transaction.mint.tx_pointer.tx_index)

      let output_contract = new OutputContract(id.toHexString())
      output_contract.input_index = BigInt.fromU32(transaction.mint.output_contract.input_index)
      output_contract.balance_root = transaction.mint.output_contract.balance_root
      output_contract.state_root = transaction.mint.output_contract.state_root

      mint.tx_pointer = tx_pointer.id
      mint.output_contract = output_contract.id
      transaction_sc.mint = mint.id

      let policies = new Policies(id.toHexString())
      policies.values = "arsars".toString()

      mint.policies = policies.id

      policies.save()


      output_contract.save()
      tx_pointer.save()
      mint.save()

    }
    // Todo: Simplify
    // if (transaction.kind() == "script") {
    //
    //   let script = new Script(id.toHexString())
    //   script.script_gas_limit = BigInt.fromU64(transaction.script.script_gas_limit)
    //   script.script = transaction.script.script
    //   script.script_data = transaction.script.script_data
    //   script.receipts_root = transaction.script.receipts_root
    //
    //   let policies = new Policies(id.toHexString())
    //   policies.values = transaction.script.policies.valuesList.toString()
    //
    //   script.policies = policies.id
    //
    //   policies.save()
    //   transaction_sc.script = script.id
    //   script.save()
    //
    // }
    //
    // if (transaction.kind() == "create") {
    //
    //   let create = new Create(id.toHexString())
    //   create.bytecode_length = BigInt.fromU64(transaction.create.bytecode_length)
    //   create.bytecode_witness_index =  BigInt.fromU64(transaction.create.bytecode_witness_index)
    //   create.witnessesList = transaction.create.witnessesList
    //   create.salt = transaction.create.salt
    //
    //   let policies = new Policies(id.toHexString())
    //   policies.values = transaction.script.policies.valuesList.toString()
    //
    //   create.policies = policies.id
    //
    //   policies.save()
    //   transaction_sc.create = create.id
    //   create.save()
    //
    // }

    transaction_sc.save();
  }

  return new Array<string>(transactions.length).fill(id.toHexString());
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