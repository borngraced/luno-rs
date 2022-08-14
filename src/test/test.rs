use luno_rust_api::Luno;
use serde_json::json;
use std::env;

#[tokio::test] // any async runtime can be used, not limited to tokio
async fn test_luno_async() {
    dotenv::dotenv().ok();
    let key = env::var("API_KEY").expect("Api Key doesn't exist yet, please add");
    let secret = env::var("API_SECRET").expect("Api Key Secret doesn't exist yet, please add");
    let luno = Luno::init(key, secret).await;
    let tickers = luno.get_all_balance().await;
    println!("{:#?}", json!(tickers.as_ref().unwrap())); // data can be serialized to json)
    assert_eq!(true, tickers.is_ok());
}
