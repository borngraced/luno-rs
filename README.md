# Luno-API-Client-Rust (async)

# V.0.0.1 beta

https://www.luno.com/en/developers/api

The Luno API provides developers with a wealth of financial information provided through the Luno platform. Through this secure system developers can:

    Create accounts for trading in cryptocurrencies
    Access current and historic cryptocurrency market data
    Submit trade orders and view order status
    Buy and sell Bitcoin and Ethereum
    Send and receive Bitcoin and Ethereum
    Generate Bitcoin and Ethereum wallet addresses

The Async Rust Luno API brings the world of Bitcoin and Ethereum to your doorstep.

This API is still in beta phase and not ready to be used in a live environment

# Authentication

Some API calls require your application to authenticate itself. This is done using an API key associated with your account. You can create an API key by visiting the API Keys section on the settings page.

An API key consists of a key id and a key secret. For example, cnz2yjswbv3jd (key id) and 0hydMZDb9HRR3Qq-iqALwZtXLkbLR4fWxtDZvkB9h4I (key secret).

API requests are authenticated using HTTP basic authentication with the key id as the username and the key secret as the password. A missing, incorrect or revoked key causes error 401 to be returned.

Each API key is granted a set of permissions when it is created. The key can only be used to call the permitted API functions.

# USAGE

# Dependencies

1. reqwest
2. serde
3. serde_json
4. futures
5. tokio
6. dotenv

## Configuration

1. Get your api key from luno
2. configure your env with:
   ##### API_KEY=myapikey
   ##### API_SECRET=myapisecret

## INITIALIZING LUNO API FROM RUST APP

```rust
#[tokio::test]
async fn test_something_async() {
    dotenv::dotenv().ok();
    // for some reason, test cannot be async
    let key = env::var("API_KEY").expect("Api Key doesn't exist yet, please add");
    let secret = env::var("API_SECRET").expect("Api Key Secret doesn't exist yet, please add");
    let luno = Luno::init(key, secret);
    let balance = luno.get_all_balance().await;
    println!("Balances {:?}", balance)
}
```

### Available METHODS regularly (more will be added until completion)

1. create_account()
2. get_ticker(pair: `"XBTNGN"`)
3. get_all_tickers()
4. get_all_balance()
5. get_all_tickers()
6. get_full_order_book(pair: `"XBTNGN"`)
7. get_top_order_book(pair: `"XBTNGN"`)

# CONTRIBUTING

make us better :)
