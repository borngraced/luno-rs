use futures::FutureExt;
use reqwest::{header::HeaderMap, Error, Method, Response, Url};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
#[allow(unused_variables)]
use std::collections::HashMap;
use std::env;

const DEFAULT_BASE_URL: &str = "https://api.luno.com";

#[derive(Debug)]
pub struct Luno {
    pub base_url: String,
    pub path: Option<Url>,
    pub method: Method,
    username: String,
    password: String,
}

impl Luno {
    pub fn init(api_key_id: String, api_key_secret: String) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            username: api_key_id,     // api_key_id
            password: api_key_secret, // api_key_secret
            path: None,
            method: Method::GET,
        }
    }
    pub async fn create_account(
        &self,
        account_name: String,
        currency_name: String,
    ) -> Result<Account, Error> {
        let path = Url::parse(format! {"{}{}", self.base_url, "/api/1/accounts"}.as_str()).unwrap();
        let result = create_account_post(
            path,
            currency_name,
            account_name,
            &self.username,
            Some(&self.password),
        )
        .await
        .expect("An error occured")
        .text()
        .await
        .unwrap();
        println!("{}, {:?}", result, self);
        let result: Account = from_str(&result).unwrap();
        Ok(result)
    }

    pub async fn get_all_balance(&self) -> Result<Vec<Balance>, Error> {
        let path = Url::parse(format! {"{}{}", self.base_url, "/api/1/balance/"}.as_str()).unwrap();
        let result = get_request(path, &self.username, Some(&self.password))
            .await
            .expect("An error occured")
            .text()
            .await
            .unwrap();
        let result: HashMap<String, Vec<Balance>> = from_str(&format!("{}", result)).unwrap();
        Ok(result.get("balance").unwrap().to_vec())
    }
    pub async fn get_ticker(&self, pair: &String) -> Result<Tickers, Error> {
        let path = Url::parse(format! {"{}{}", self.base_url, "/api/1/ticker/"}.as_str()).unwrap();
        let result = get_request_with_params(
            path,
            (&"pair".to_string(), pair),
            &self.username,
            Some(&self.password),
        )
        .await
        .expect("Unable to get ticker")
        .text()
        .await
        .unwrap();
        let result: Tickers = from_str(&result).unwrap();
        Ok(result)
    }
    pub async fn get_all_tickers(&self) -> Result<Vec<Tickers>, Error> {
        let path = Url::parse(format! {"{}{}", self.base_url, "/api/1/tickers/"}.as_str()).unwrap();
        let result = get_request(path, &self.username, Some(&self.password))
            .await
            .expect("An error occured")
            .text()
            .await
            .unwrap();
        let result: HashMap<String, Vec<Tickers>> = from_str(&format!("{}", result)).unwrap();
        println!("{:?}", result.get("ticker"));
        Ok(result.get("tickers").unwrap().to_vec())
    }
    pub async fn get_full_order_book(&self, pair: &String) -> Result<OrderBook, Error> {
        let path =
            Url::parse(format! {"{}{}", self.base_url, "/api/1/orderbook"}.as_str()).unwrap();
        let result = get_request_with_params(
            path,
            (&"pair".to_string(), pair),
            &self.username,
            Some(&self.password),
        )
        .await
        .expect("Unable to get ticker")
        .text()
        .await
        .unwrap();
        let result: OrderBook = from_str(&result).unwrap();
        Ok(result)
    }
    pub async fn get_top_order_book(&self, pair: &String) -> Result<OrderBook, Error> {
        let path =
            Url::parse(format! {"{}{}", self.base_url, "/api/1/orderbook_top"}.as_str()).unwrap();
        let result = get_request_with_params(
            path,
            (&"pair".to_string(), pair),
            &self.username,
            Some(&self.password),
        )
        .await
        .expect("Unable to get ticker")
        .text()
        .await
        .unwrap();
        let result: OrderBook = from_str(&result).unwrap();
        Ok(result)
    }
    pub async fn get_recent_trades(&self, timestamp: String, pair: String) {}
}
#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    currency: String,
    id: String,
    name: String,
    pending: Vec<Pending>,
    transactions: Vec<Transaction>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Pending {
    pub account_id: String,
    pub available: String,
    pub available_delta: String,
    pub balance: String,
    pub balance_delta: String,
    pub currency: String,
    pub description: String,
    pub details_field: DetailsField,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    pub account_id: String,
    pub available: String,
    pub available_delta: String,
    pub balance: String,
    pub balance_delta: String,
    pub currency: String,
    pub description: String,
    pub details_field: DetailsField,
    pub details: Name,
    pub kind: Kind,
    pub row_index: String,
    pub timestamp: String,
}
type Name = String;
#[derive(Debug, Deserialize, Clone)]
pub struct DetailsField {
    pub crypto_details: CryptoDetails,
    pub trade_details: TradeDetails,
}
#[derive(Debug, Deserialize, Clone)]
pub struct CryptoDetails {
    pub address: String,
    pub txid: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct TradeDetails {
    pub pair: String,
    pub price: String,
    pub sequence: i64,
    pub volume: String,
}
#[derive(Debug, Deserialize, Clone)]
pub enum Kind {
    FEE,
    TRANSFER,
    EXCHANGE,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Balance {
    pub account_id: String,
    pub asset: String,
    pub balance: String,
    pub reserved: String,
    pub unconfirmed: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Tickers {
    pub pair: String,
    pub timestamp: i64,
    pub bid: String,
    pub ask: String,
    pub last_trade: String,
    pub rolling_24_hour_volume: String,
    pub status: Status,
}
#[derive(Debug, Deserialize, Clone)]
pub enum Status {
    ACTIVE,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Ticker {
    pub pair: String,
    pub timestamp: i64,
    pub bid: String,
    pub ask: String,
    pub last_trade: String,
    pub rolling_24_hour_volume: String,
    pub status: Status,
}
#[derive(Debug, Deserialize, Clone)]
pub struct OrderBook {
    pub timestamp: usize,
    pub asks: Vec<Asks>,
    pub bids: Vec<Bids>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Asks {
    pub price: String,
    pub volume: String,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Bids {
    pub price: String,
    pub volume: String,
}

async fn get_request(path: Url, key: &String, secret: Option<&String>) -> Result<Response, Error> {
    //let mut request = Client::builder();
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        "rust-luno v 0.2".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(reqwest::header::ACCEPT_CHARSET, "utf-8".parse().unwrap());
    Ok(reqwest::Client::new()
        .get(path)
        .basic_auth(key, secret.as_ref())
        .headers(headers)
        .send()
        .await
        .expect("An error occured"))
}
async fn get_request_with_params(
    path: Url,
    param: (&String, &String),
    key: &String,
    secret: Option<&String>,
) -> Result<Response, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        "rust-luno v 0.2".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(reqwest::header::ACCEPT_CHARSET, "utf-8".parse().unwrap());
    let mut params = HashMap::new();
    params.insert(param.0, param.1);
    Ok(reqwest::Client::new()
        .get(path)
        .basic_auth(key, secret.as_ref())
        .headers(headers)
        .query(&[(param.0, param.1)])
        .send()
        .await
        .expect("An error occured"))
}
async fn create_account_post(
    path: Url,
    currency_name: String,
    account_name: String,
    key: &String,
    secret: Option<&String>,
) -> Result<Response, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        "rust-luno v 0.2".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(reqwest::header::ACCEPT_CHARSET, "utf-8".parse().unwrap());
    Ok(reqwest::Client::new()
        .post(path)
        .basic_auth(key, secret.as_ref())
        .headers(headers)
        .query(&[("currency", currency_name), ("name", account_name)])
        .send()
        .await
        .expect("An error occured while creating account"))
}
#[tokio::test]
async fn test_something_async() {
    dotenv::dotenv().ok();
    // for some reason, test cannot be async
    let key = env::vars().enumerate();
    // let init = Luno::init(key[0], secret[1]);
    // let tickers = init
    //     .create_account("Trading ACC".to_string(), "XRPBTC".to_string())
    //     .await;
    println!("{:?}", key)
}
