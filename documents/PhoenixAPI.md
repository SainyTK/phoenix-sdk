# Example code to interact with Phoenix in Rust

## Get all markets
You can get all markets by calling the getProgramAccounts RPC, filtering for accounts that match the market discriminant. Sample code available here.
let client = EllipsisClient::from_rpc(
        RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed()),
        &payer,
    )?;

    let market_discriminant = get_discriminant("phoenix::program::accounts::MarketHeader");

    // Fetch all markets
    // Memcmp encoding field is deprecated
    #[allow(deprecated)]
    let program_accounts = client
        .get_program_accounts_with_config(
            &phoenix::ID,
            RpcProgramAccountsConfig {
                filters: Some(vec![RpcFilterType::Memcmp(Memcmp {
                    offset: 0,
                    bytes: MemcmpEncodedBytes::Bytes(market_discriminant.to_le_bytes().to_vec()),
                    encoding: None,
                })]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(CommitmentConfig::confirmed()),
                    ..RpcAccountInfoConfig::default()
                },

                ..RpcProgramAccountsConfig::default()
            },
        )
        .await?;

    println!("Found {} markets", program_accounts.len());

## Viewing the state of the order book
To get the state of a market's order book, call SDKClient.get_market_orderbook(&market_pubkey). The order book can be pretty-printed with orderbook.print_ladder().

let client = EllipsisClient::from_rpc(
    RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    ),
    &payer,
)?;

let sdk_client = SDKClient::new_from_ellipsis_client(client).await?;
let sol_market = Pubkey::from_str("4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg")?;
let orderbook = sdk_client.get_market_orderbook(&sol_market).await?;
orderbook.print_ladder(5, 4);
// Example output
            19.9240   214.3670  
            19.9090   53.5920   
   46.8330  19.8860             
   37.4840  19.8710     

## Fetching market events
All market events are logged for easy data indexing and trader convenience. Events are recorded in instruction data via an authorized self-CPI to ensure no logs are dropped.

By listening to all transactions that touch a given market, one can create a stream of all market events as they are confirmed by the blockchain with a polling loop. For latency sensitive operations, look into running a Geyser Plugin.

let payer = Keypair::new();
let client = EllipsisClient::from_rpc(
    RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    ),
    &payer,
)?;

let sdk_client = Arc::new(SDKClient::new_from_ellipsis_client(client).await?);
let sol_market = Pubkey::from_str("4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg")?;

let mut until = None;
loop {
    let config = match until {
        None => GetConfirmedSignaturesForAddress2Config {
            before: None,
            until: None,
            limit: Some(1),
            commitment: Some(CommitmentConfig::confirmed()),
        },
        Some(until) => GetConfirmedSignaturesForAddress2Config {
            before: None,
            until: Some(until),
            limit: None,
            commitment: Some(CommitmentConfig::confirmed()),
        },
    };

    let signatures = sdk_client
        .client
        .get_signatures_for_address_with_config(&sol_market, config)
        .await
        .unwrap_or_default()
        .iter()
        .map(|tx| Signature::from_str(&tx.signature).unwrap())
        .rev()
        .collect::<Vec<_>>();

    if !signatures.is_empty() {
        until = Some(signatures[0]);
    }

    let mut handles = vec![];

    for signature in signatures {
        let sdk = sdk_client.clone();
        let handle =
            tokio::spawn(
                async move { sdk.parse_events_from_transaction(&signature).await }
            );
        handles.push(handle);
    }

    for handle in handles {
        let events = handle.await?;
        events.map(|events| {
            events.iter().for_each(|e| {
                // Here we only print the event, but in practice, you can do
                // a lot more
                println!("{:#?}", e);
            });
        });
    }

    // Note: this is a basic polling loop, if there are >1000 signatures in 200ms
    // events will get dropped
    tokio::time::sleep(Duration::from_millis(200)).await;
}
Sample output:

Copy
PhoenixEvent {
    market: 4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg,
    sequence_number: 10048932,
    slot: 185021845,
    timestamp: 1679941569,
    signature: 2AH2Ywtk9fiR1PLmcQE1SrPRWpwMyfiB2Rr9QSwapw4DUBpNExst6haPEkjwaSCgAFt4jMYGqhVkUVfDd6D6zkTg,
    signer: 3HBWHuyxWv4uN8U8SeukocrWPfLZJqrtj9DgDHsGo2HR,
    event_index: 1,
    details: Place(
        Place {
            order_sequence_number: 8079036,
            client_order_id: 3,
            maker: 3HBWHuyxWv4uN8U8SeukocrWPfLZJqrtj9DgDHsGo2HR,
            price_in_ticks: 19909,
            base_lots_placed: 214567,
        },
    ),
}

For more information: https://ellipsis-labs.gitbook.io/phoenix-dex/tRIkEFlLUzWK9uKO3W2V