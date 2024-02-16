import '../common/eager_offset';
import {Bytes} from '../common/collections';

export namespace fuel {

  export class Block {
    constructor(
        public id: Bytes,
        public height: u32,
        public da_height: u64,
        public msg_receipt_count: u64,
        public tx_root: Bytes,
        public msg_receipt_root: Bytes,
        public prev_id: Bytes,
        public prev_root: Bytes,
        public timestamp: u64,
        public application_hash: Bytes,
        public transactions: Array<Transaction>
    ) {}
  }

  export class Transaction {

    constructor(
        public script: Script,
        public create: Create,
        public mint: Mint,
    ) {}

    kind(): string {
      if (this.script !== null) return 'script';
      if (this.create !== null) return 'create';
      if (this.mint !== null) return 'mint';
      return 'kindCase is not set';
    }

  }

  export class Script {
    constructor(
        public script_gas_limit: u64,
        public script: Bytes,
        public script_data: Bytes,
        public policies: Policies,
        public inputsList: Array<Input>,
        public outputsList: Array<Output>,
        public witnessesList: Array<Bytes>,
        public receipts_root: Bytes
    ) {}
  }

  export class Create {
    constructor(
        public bytecode_length: u64,
        public bytecode_witness_index: u32,
        public policies: Policies,
        public storage_slotsList: Array<StorageSlot>,
        public inputsList: Array<Input>,
        public outputsList: Array<Output>,
        public witnessesList: Bytes,
        public salt: Bytes
    ) {}
  }

  export class Mint {
    constructor(
        public tx_pointer: TxPointer,
        public input_contract: InputContract,
        public output_contract: OutputContract,
        public mint_amount: u64,
        public mint_asset_id: Bytes
    ) {}
  }

  export enum InputKind {
    KIND_NOT_SET = 0,
    COIN_SIGNED = 1,
    COIN_PREDICATE = 2,
    CONTRACT = 3,
    MESSAGE_COIN_SIGNED = 4,
    MESSAGE_COIN_PREDICATE = 5,
    MESSAGE_DATA_SIGNED = 6,
    MESSAGE_DATA_PREDICATE = 7,
  }

  export class Input {
    constructor(
        // public kindCase: InputKind,
        public coin_signed: Coin,
        public coin_predicate: Coin,
        public contract: InputContract,
        public message_coin_signed: Message,
        public message_coin_predicate: Message,
        public message_data_signed: Message,
        public message_data_predicate: Message
    ) {}
  }

  export class Coin {
    constructor(
        public utxo_id: UtxoId,
        public owner: Bytes,
        public amount: u64,
        public asset_id: Bytes,
        public tx_pointer: TxPointer,
        public witness_index: u32,
        public maturity: u32,
        public predicate_gas_used: u64,
        public predicate: Bytes,
        public predicate_data: Bytes
    ) {}
  }

  export class Message {
    constructor(
        public sender: Bytes,
        public recipient: Bytes,
        public amount: u64,
        public nonce: Bytes,
        public witness_index: u32,
        public predicate_gas_used: u64,
        public data: Bytes,
        public predicate: Bytes,
        public predicate_data: Bytes
    ) {}
  }

  export enum OutputKind {
    KIND_NOT_SET = 0,
    COIN = 1,
    CONTRACT = 2,
    CHANGE = 3,
    VARIABLE = 4,
    CONTRACT_CREATED = 5,
  }

  export class Output {
    constructor(
        // public kindCase: OutputKind,
        public coin: OutputCoin,
        public contract: OutputContract,
        public change: OutputCoin,
        public variable: OutputCoin,
        public contract_created: OutputContractCreated
    ) {}
  }

  export class OutputCoin {
    constructor(
        public to: Bytes,
        public amount: u64,
        public asset_id: Bytes
    ) {}
  }

  export class OutputContractCreated {
    constructor(
        public contract_id: Bytes,
        public state_root: Bytes
    ) {}
  }

  export class InputContract {
    constructor(
        public utxo_id: UtxoId,
        public balance_root: Bytes,
        public state_root: Bytes,
        public tx_pointer: TxPointer,
        public contract_id: Bytes
    ) {}
  }

  export class OutputContract {
    constructor(
        public input_index: u32,
        public balance_root: Bytes,
        public state_root: Bytes
    ) {}
  }

  export class StorageSlot {
    constructor(
        public key: Bytes,
        public value: Bytes
    ) {}
  }

  export class UtxoId {
    constructor(
        public tx_id: Bytes,
        public output_index: u32
    ) {}
  }

  export class TxPointer {
    constructor(
        public block_height: u32,
        public tx_index: u32
    ) {}
  }

  export class Policies {
    constructor(
        public valuesList: Bytes
    ) {}
  }

}