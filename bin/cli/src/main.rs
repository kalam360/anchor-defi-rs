use ethers::contract::{Contract, ContractFactory};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use std::convert::TryFrom;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the Ethereum network
    let provider = Arc::new(Provider::<Http>::try_from("http://localhost:8545")?);

    // Load the ABI and bytecode of the contract
    let abi: ethers::contract::Abi = serde_json::from_str(include_str!("../abi/NewContract.json"))?;
    let bytecode: Bytes = include_bytes!("../bytecode/NewContract.bin").to_vec().into();

    // Create a factory for deploying the contract
    let wallet: LocalWallet = "your_private_key".parse()?;
    let client = SignerMiddleware::new(provider, wallet);
    let factory = ContractFactory::new(abi, bytecode, Arc::new(client));

    // Deploy the contract
    let contract = factory.deploy("NewContract")?.send().await?.into();

    // Interact with the contract
    let value: U256 = contract.call("getValue", ()).await?;
    println!("Current value: {}", value);

    // Set a new value
    let tx = contract.call("setValue", (42u32,)).send().await?;
    tx.await?;

    // Get the updated value
    let value: U256 = contract.call("getValue", ()).await?;
    println!("Updated value: {}", value);

    Ok(())
}
