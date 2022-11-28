pub mod address_book;
pub mod alert;
pub mod block_scanner;
pub mod helpers;
pub mod mempool;

use address_book::*;
use ethers::{abi::AbiDecode, prelude::k256::ecdsa::SigningKey, prelude::*};
use helpers::address;
use std::error::Error;
use std::sync::Arc;
use std::time::Instant;

use crate::helpers::setup_signer;

pub struct Config {
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub wss: Arc<Provider<Ws>>,
}

impl Config {
    pub async fn new() -> Self {
        let network = std::env::var("NETWORK_RPC").expect("missing NETWORK_RPC env var");
        let provider: Provider<Http> = Provider::<Http>::try_from(network).unwrap();
        let middleware = Arc::new(setup_signer(provider.clone()).await);

        let ws_network = std::env::var("NETWORK_WSS").expect("missing NETWORK_WSS env var");
        let ws_provider: Provider<Ws> = Provider::<Ws>::connect(ws_network).await.unwrap();
        Self {
            http: middleware,
            wss: Arc::new(ws_provider),
        }
    }
}

const PAGE_SIZE: u64 = 25000;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new().await;
    let client = Arc::clone(&config.wss);

    let last_block = client
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();
    let mut current_block: U64 = U64::from(ATLAS_MINE_CONTRACT_START_BLOCK);
    let block_delta: U64 = last_block - current_block;
    println!(
        "last_block={}, start_block={}, block_difference={}",
        last_block, current_block, block_delta
    );

    let start = Instant::now();
    let mut event_count = 0;

    // Iterate from contract deployment block to latest block
    while last_block >= current_block {
        let withdraw_event_filter = Filter::new()
            .from_block(current_block)
            .to_block(current_block + PAGE_SIZE)
            .address(address(ATLAS_MINE_CONTRACT))
            .event(ATLAS_MINE_EVENT_WITHDRAW);

        let mut stream = client.get_logs_paginated(&withdraw_event_filter, PAGE_SIZE);

        while let Some(res) = stream.next().await {
            let log = res?;
            println!(
                "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
                log.block_number,
                log.transaction_hash,
                log.address,
                log.topics.get(1),
                log.topics.get(2),
                U256::decode(log.data)
            );
            event_count += 1;
        }

        // Increment current block to get to the latest block
        current_block = current_block + 1000;
        println!("current_block after increment={current_block}");
    }

    println!(
        "Processed event_count={} in {:?}",
        event_count,
        start.elapsed()
    );
    Ok(())
}
