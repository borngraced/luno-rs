# Luno-rs (async)

The Luno API wrapper written in rust of rust in rust prorgram
https://www.luno.com/en/developers/api.

### Dependencies

* reqwest
* serde
* serde_json
* tokio

### Configuration

1. Get your api key and secret from luno.com
2. Create .env file in your project route and configure with:
   ` API_KEY=myapikey`
   `API_SECRET=myapisecret`
3. import Luno client to your project and initialize `use luno_rs::Luno;`

```rust
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
}
```
### Available METHODS regularly (more will be added until completion)

### available endpoints
1. [x] `create_account()`
2. [x] `get_all_balance()`
3. [x] `get_ticker(pair: "XBTNGN")`
4. [x] `get_all_tickers()`
5. [x] `get_full_order_book(pair: "XBTNGN")`
6. [x] `get_top_order_book(pair: "XBTNGN")`
### Todos
7. [ ] update_account_name
8. [ ] list_pending_transactions
9. [ ] list_transactions
10. [ ] list_account_balances
11. [ ] list_recent_trades
12. [ ] get_candles
13. [ ] get_markets_info
14. [ ] list_orders
15. [ ] list_trades
16. [ ] post_market_order
17. [ ] get_order
18. [ ] post_limit_order
19. [ ] cancel_order
20. [ ] get_receiver_address
21. [ ] create_receiver_address
22. [ ] send
23. [ ] estimate_send_fee
24. [ ] list_withdrawal_request
25. [ ] request_a_withdrawal
26. [ ] get_withdrawal_request
27. [ ] cancel_withdrawal_request
28. [ ] list_transfers
29. [ ] list_eneficiaries

# CONTRIBUTING
feel free to work on any of the todos:)

[https://www.luno.com/en/developers/api](https://www.luno.com/en/developers/api).
