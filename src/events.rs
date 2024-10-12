use alloy::{
    dyn_abi::{DynSolType, DynSolValue}, hex, primitives::address, providers::{Provider, ProviderBuilder, WsConnect}, rpc::types::{BlockNumberOrTag, Filter}
};
use futures_util::stream::StreamExt;
use std::error::Error;

pub async fn monitor_events() -> Result<(), Box<dyn Error>> {
    // Create the provider.
    let rpc_url = "wss://base-sepolia.g.alchemy.com/v2/ZXlHMZsPDpR82kjqBEH8KpPvacbKpmsI";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Create a filter to watch for UNI token transfers.
    let uniswap_token_address = address!("9b17032749aa066a2DeA40b746AA6aa09CdE67d9");
    let filter = Filter::new()
        .address(uniswap_token_address)
        // By specifying an `event` or `event_signature` we listen for a specific event of the
        // contract. In this case the `Transfer(address,address,uint256)` event.
        .event("Transfer(address,address,uint256)")
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        let val = &log.data().data;
        println!("Uniswap token logs: {:?}, {:?}, {:?}", log.topics(), log.transaction_hash, log.data());
        // decode(val)?;
    }

    Ok(())
}

fn print_tuple(value: &DynSolValue, field_names: &[&str]) {
    if let DynSolValue::Tuple(values) = value {
        for (value, name) in values.iter().zip(field_names.iter()) {
            println!("  {}: {:?}", name, value);
        }
    }
}

fn decode(encoded_domain: &[u8]) -> Result<(), Box<dyn Error>> {
    println!("{}", hex::encode(encoded_domain));
    let domain_type = DynSolType::Tuple(vec![DynSolType::Address, DynSolType::Address, DynSolType::Uint(256)]);

    let decoded_domain = domain_type.abi_decode(encoded_domain)?;
    println!("\nDecoded domain:");
    print_tuple(&decoded_domain, &["from", "to", "amount"]);
    Ok(())
}