use dydx_v4_rust_sdk::clients::indexer_client::IndexerClient;
use dydx_v4_rust_sdk::clients::utility::Utility;

#[tokio::main]
async fn main() {
    let indexer_client = IndexerClient::new("https://indexer.dydx.trade");
    let utility = Utility{};    
    let response = utility.get_time(&indexer_client).await;

    match response {
        Ok(value) => {
            let pretty_json = serde_json::to_string_pretty(&value).unwrap();
            println!("{}", pretty_json);
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}