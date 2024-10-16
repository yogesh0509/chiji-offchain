use alloy::{
    dyn_abi::{DynSolType, DynSolValue},
    hex,
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
};
use futures_util::stream::StreamExt;
use std::error::Error;

pub async fn monitor_events() -> Result<(), Box<dyn Error>> {
    println!("Looking for events 🔍");
    let rpc_url = "wss://base-sepolia.g.alchemy.com/v2/dsfDS_Je3D5uE96msKwAppfkvsnys2a2";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    let factory_address = address!("03F7f254cC7442045cbBbC16b268cbF87608659D");

    let filter = Filter::new()
        .address(factory_address)
        .event("ContractDeployed(index_topic_1 address contractAddress, index_topic_2 address timelockContract)")
        .from_block(BlockNumberOrTag::Earliest);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    while let Some(log) = stream.next().await {
        println!("Factory logs: ${:?}", log);
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
    let domain_type = DynSolType::Tuple(vec![
        DynSolType::Address,
        DynSolType::Address,
        DynSolType::Uint(256),
    ]);

    let decoded_domain = domain_type.abi_decode(encoded_domain)?;
    println!("\nDecoded domain:");
    print_tuple(&decoded_domain, &["from", "to", "amount"]);
    Ok(())
}
