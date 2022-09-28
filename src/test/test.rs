/* use luno_rust_api::Luno;
use serde_json::json;
use std::env;

#[tokio::test] // any other async runtime can be used, not limited to tokio
async fn test_luno_async() {
	dotenv::dotenv().ok();
	let key = env::var("API_KEY").expect("Api Key doesn't exist yet, please add");
	let secret = env::var("API_SECRET").expect("Api Key Secret doesn't exist yet, please add");
	let luno = luno_rs::Luno::init(key, secret).await;
	let balance = luno.get_all_balance().await;
	assert!(tickers.is_ok());
    let all_balance: Vec<Balance> = balance.unwrap();
    print!("{balance:#?}")
} assert_eq!(true, tickers.is_ok());
}
 */
