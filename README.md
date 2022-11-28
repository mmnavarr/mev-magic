# MEV Magic

## Quick Start

- Get a node from [QuickNode](https://www.quicknode.com) as they have a free websocket option or you can run your own node.
- Import your EOA priv key for the bot to execute transactions.
- For testing, run `cargo run`
- For production, run `cargo run --release`

## Magic Unlock Backrun Strategy

Back Test Strategy:

1. Watch for `withdraw` events from the Atlas Mine contract `0x1EAb8B6B2f73239B01B20CAB5C2c9B7E80ac7743`
2. Compute $MAGIC price delta from time of event and 5 minutes later
3. Tally positive and negative deltas and the mean difference
    1. With prices generally trending down, how can we best determine the price impact of unlocked magic?
4. Determine if it is probable that large withdrawls have negative price movement
    1. If so, the strategy would be to backrun the withdrawl with a sell swap and buy back in after some set time intetval


### Mempool Monitoring Example

```
---------- MONITORING MEMPOOL ----------
Transaction: Transaction {
    hash: 0xcb3647deb3b7ada364a6643752bf9243b27e84cea78cc0010d26fa3ae52b5e13,
    nonce: 22387,
    block_hash: None,
    block_number: None,
    transaction_index: None,
    from: 0xe88102f2900483c63d0adcdaf4839c2759949de6,
    to: Some(
        0x16327e3fbdaca3bcf7e38f5af2599d2ddc33ae52,
    ),
    value: 9024569904524523678,
    gas_price: Some(
        120000000000,
    ),
    gas: 1000000,
    input: Bytes(
        b"\x7f\xf3j\xb5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x10\xbbdEK\xa0[\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\0\0\0\0\0\0\0\0\0\0\0\0\xe8\x81\x02\xf2\x90\x04\x83\xc6=\n\xdc\xda\xf4\x83\x9c'Y\x94\x9d\xe6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\xf0\\\x10\xa0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\0\0\0\0\0\0\0\0\0\0\0\0!\xbe7\rS\x12\xf4L\xb4,\xe3w\xbc\x9b\x8a\x0c\xef\x1aL\x83\0\0\0\0\0\0\0\0\0\0\0\0\xbeAw%\x87\x87*\x92\x18Hs\xd5[\t\xc6\xbboY\xf8\x95",
    ),
    v: 535,
    r: 44692797049587778392645963656820336298473713424700451186489839760496971858835,
    s: 36641529939556041694165250732768051817286656083457538183752182129357071704354,
    transaction_type: Some(
        0,
    ),
    access_list: None,
    max_priority_fee_per_gas: None,
    max_fee_per_gas: None,
    chain_id: None,
    other: OtherFields {
        inner: {},
    },
}
```

## Inspiration

[DeGatchi](https://twitter.com/DeGatchi) wrote an article [How To Build A MEV Bot](https://www.degatchi.com/articles/how-to-build-a-mev-bot) that explains the overall architecture you need to think of to build a bot - from getting a strategy and fetching data to general tips on design structure (that you most likely wont find anywhere else).