import { BigInt, Bytes, fuel, log } from "@graphprotocol/graph-ts"
import { sha256Hash } from "./sha256/crypto";

import { Asset, AssetIssuer } from "../generated/schema"

function getAssetId(contractId: Bytes, subId: Bytes): Bytes {
  let id = Bytes.fromUint8Array(sha256Hash(contractId.concat(subId)));
  return id;
}

//

function handleTransactions(transactions: fuel.Transaction[]): void {
  for (let i  = 0; i < transactions.length; i++) {
    const transaction = transactions[i];

    if (transaction.kind() == "script") {
      for (let i = 0; i < transaction.receipts.length; i += 1) {
        let receipt = transaction.receipts[i];
        if (receipt.kind() == "mint") {
          let id = getAssetId(receipt.mint.contract_id, receipt.mint.sub_id);
          log.info("MINT - ID {}", [id.toHexString()]);

          if (Asset.load(id.toHexString()) == null) {
            let asset = new Asset(id.toHexString());
            asset.sub_id = receipt.mint.sub_id;
            asset.issuer = receipt.mint.contract_id.toHexString();

            let issuer = AssetIssuer.load(receipt.mint.contract_id.toHexString());
            if (issuer == null) {
              issuer = new AssetIssuer(receipt.mint.contract_id.toHexString());
              issuer.num_assets = 1;
            } else {
              issuer.num_assets += 1;
            }
            issuer.save();
            asset.save();
          }
        }
      }
    }
  }
}

export function handleBlock(block: fuel.Block): void {
  handleTransactions(block.transactions)
}
