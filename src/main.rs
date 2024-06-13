use std::error::Error;
use solana_sdk::{pubkey, pubkey::Pubkey};

mod lib_gen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let kapital: u64 = 100000000; // 0,1 SOL
    let bps: u16 = 1000;
    pub const WALLET: Pubkey = pubkey!("3AugJd4PLZPgXFdhj4gb52U57FGUr3FjifHSuLejRaxM");
    let private_key_str: &str = "1";
    let addresses: Vec<&str> = ["DriFtupJYLTosbwoN8koMbEYSx54aFAVLddWsbksjwg7", "MEW1gQWJ3nEXg2qgERiKu7FAFj79PHvQVREQUzScPP5"].to_vec();
    

    let mut handles = vec![];

    for address in addresses {
        let handle = tokio::spawn(async move {
            loop {
                let _result = lib_gen::start(kapital, bps, private_key_str, WALLET, address).await;
            }
        });
        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    Ok(())
}
