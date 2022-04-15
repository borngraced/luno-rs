use luno_rust_api::Luno;
use std::env;

#[tokio::test]
async fn test_something_async() {
    dotenv::dotenv().ok();
    // for some reason, test cannot be async
    let key = env::var("API_KEY").expect("Api Key doesn't exist yet, please add");
    let secret = env::var("API_SECRET").expect("Api Key Secret doesn't exist yet, please add");
    let init = Luno::init(key, secret);
    let tickers = init.get_all_balance().await;
    println!("hi {:#?}", tickers)
}
