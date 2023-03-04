mod abis;
mod helpers;
mod pb;

use pb::sinkfiles::Lines;
use pb::token_stream::{Erc1155TransferBatch, Erc1155TransferSingle, Transfer, Transfers};
use serde_json::json;
use substreams::errors::Error;
use substreams::{log, Hex};
use substreams_ethereum::{pb::eth::v2 as eth, Event as EventTrait};

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    let mut transfers = vec![];
    let mut erc1155_transfer_batchs = vec![];
    let mut erc1155_transfer_singles = vec![];
    let block_timestamp = blk
        .clone()
        .header
        .as_ref()
        .unwrap()
        .timestamp
        .as_ref()
        .unwrap()
        .seconds as u64;
    for log in blk.logs() {
        let tx_hash = Hex(log.receipt.transaction.hash.clone()).to_string();
        log::info!("Tx Hash {}", tx_hash);

        // ERC20 Transfer
        if let Some(event) = abis::erc20::events::Transfer::match_and_decode(log) {
            let (from_balance, to_balance) = helpers::utils::get_balances(
                event.value.clone(),
                log.receipt.transaction.calls.clone(),
            );
            let erc20_transfer: Transfer = Transfer {
                amount: event.value.to_string(),
                token_id: event.value.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.clone().address()).to_string(),
                ),
                chain_id: 1.to_string(),
                log_index: log.block_index(),
                source: 1,
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                operator: helpers::utils::format_with_0x(
                    Hex(log.receipt.transaction.from.clone()).to_string(),
                ),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: Some(1),
                block_number: blk.number,
                block_timestamp: block_timestamp,
                // log_ordinal: log.ordinal(),
                balance_from: Some(from_balance.to_string()),
                balance_to: Some(to_balance.to_string()),
                // name: token.clone().unwrap().name,
                // symbol: token.clone().unwrap().symbol,
                // decimals: token.clone().unwrap().decimals,
                ..Default::default()
            };
            transfers.push(erc20_transfer);
            // }
        }
        // ERC721 Transfer
        else if let Some(event) = abis::erc721::events::Transfer::match_and_decode(log) {
            let erc721_transfer: Transfer = Transfer {
                amount: 1.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.clone().address()).to_string(),
                ),
                chain_id: 1.to_string(),
                log_index: log.block_index(),
                source: 1,
                token_type: Some(2),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                token_id: event.token_id.to_string(),
                operator: helpers::utils::format_with_0x(
                    Hex(log.receipt.transaction.from.clone()).to_string(),
                ),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                block_number: blk.number,
                block_timestamp: block_timestamp,
                // log_ordinal: log.ordinal(),
                // balance_from: from_balance.to_string(),
                // balance_to: to_balance.to_string(),
                // name: token.clone().unwrap().name,
                // symbol: token.clone().unwrap().symbol,
                // decimals: token.clone().unwrap().decimals,
                ..Default::default()
            };
            transfers.push(erc721_transfer);
            // }
        }
        // ERC1155 Single
        else if let Some(event) = abis::erc1155::events::TransferBatch::match_and_decode(log) {
            let erc1155_transfer_batch: Erc1155TransferBatch = Erc1155TransferBatch {
                amounts: event.values.iter().map(|c| c.clone().to_string()).collect(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index(),
                source: 1,
                chain_id: 1.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.log.clone().address).to_string(),
                ),
                operator: helpers::utils::format_with_0x(Hex(event.operator).to_string()),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: 3,
                block_number: blk.number,
                block_timestamp: block_timestamp,
                token_ids: event
                    .ids
                    .iter()
                    .map(|id| Hex(id.clone()).0.to_string())
                    .collect(),
                // log_ordinal: log.ordinal(),
                ..Default::default()
            };
            erc1155_transfer_batchs.push(erc1155_transfer_batch);
        }
        // ERC1155 Batch
        else if let Some(event) = abis::erc1155::events::TransferSingle::match_and_decode(log) {
            let (from_balance, to_balance) = helpers::utils::get_balances(
                event.value.clone(),
                log.receipt.transaction.calls.clone(),
            );
            let erc1155_transfer_single: Erc1155TransferSingle = Erc1155TransferSingle {
                amount: event.value.to_string(),
                transaction_hash: helpers::utils::format_with_0x(tx_hash.clone()),
                log_index: log.block_index(),
                source: 1,
                chain_id: 1.to_string(),
                token_address: helpers::utils::format_with_0x(
                    Hex(log.log.clone().address).to_string(),
                ),
                operator: helpers::utils::format_with_0x(Hex(event.operator).to_string()),
                from: helpers::utils::format_with_0x(Hex(event.from).to_string()),
                to: helpers::utils::format_with_0x(Hex(event.to).to_string()),
                token_type: 3,
                block_number: blk.number,
                block_timestamp: block_timestamp,
                token_id: event.id.to_string(),
                balance_from: Some(from_balance.to_string()),
                balance_to: Some(to_balance.to_string()),
                // log_ordinal: log.ordinal(),
                ..Default::default()
            };
            erc1155_transfer_singles.push(erc1155_transfer_single);
        }
    }
    Ok(Transfers {
        transfers,
        erc1155_transfer_batchs,
        erc1155_transfer_singles,
    })
}

#[substreams::handlers::map]
fn jsonout(transfers: Transfers) -> Result<Lines, substreams::errors::Error> {
    Ok(pb::sinkfiles::Lines {
        lines: transfers
            .transfers
            .iter()
            .flat_map(|transfer| {
                [
                    json!({
                        "address": transfer.from,
                        "updated_at": transfer.block_timestamp,
                        "contract_decimals": transfer.decimals,
                        "contract_name": transfer.name,
                        "contract_symbol": transfer.symbol,
                        "contract_address": transfer.token_address,
                        "support_erc": transfer.token_type,
                        "logo_url": "",
                        "last_transferred_at": transfer.block_timestamp,
                        "token_type": transfer.token_type,
                        "balance": transfer.balance_from,
                        "balance_24h": "",
                        "quote_rate": "",
                        "quote_rate_24h": "",
                        "quote": "",
                        "nft_data": "",
                        "chain_id": transfer.chain_id,
                    })
                    .to_string(),
                    json!({
                        "address": transfer.from,
                        "updated_at": transfer.block_timestamp,
                        "contract_decimals": transfer.decimals,
                        "contract_name": transfer.name,
                        "contract_symbol": transfer.symbol,
                        "contract_address": transfer.token_address,
                        "support_erc": transfer.token_type,
                        "logo_url": "",
                        "last_transferred_at": transfer.block_timestamp,
                        "token_type": transfer.token_type,
                        "balance": transfer.balance_to,
                        "balance_24h": "",
                        "quote_rate": "",
                        "quote_rate_24h": "",
                        "quote": "",
                        "nft_data": "",
                        "chain_id": transfer.chain_id,
                    })
                    .to_string(),
                ]
            })
            .collect(),
    })
}
