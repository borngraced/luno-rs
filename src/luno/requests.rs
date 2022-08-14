use super::error::LunoErr;
use crate::luno::client_builder::LunoClientBuilder;
use crate::luno::error::ErrKind;
use crate::luno::structs::{Account, Balance, OrderBook, Tickers};
use reqwest::{Method, Url};
use serde_json::from_str;
use std::collections::HashMap;

const DEFAULT_BASE_URL: &str = "https://api.luno.com";
const GET_ACCOUNTS: &str = "/api/1/accounts";
const GET_BALANCE: &str = "/api/1/balance";
const GET_TICKER: &str = "/api/1/ticker";
const GET_TICKERS: &str = "/api/1/tickers";
const GET_ORDERBOOK: &str = "/api/1/orderbook";
const GET_ORDERBOOK_TOP: &str = "/api/1/orderbook_top";

pub type LunoResult<T> = Result<T, LunoErr>;

#[derive(Debug, Clone)]
pub struct Luno {
    client: LunoClientBuilder,
    pub base_url: String,
    pub path: Option<Url>,
    pub method: Method,
    username: String,
    password: String,
}

#[allow(unused)]
impl Luno {
    pub async fn init(api_key_id: String, api_key_secret: String) -> Self {
        Self {
            client: LunoClientBuilder::new().await,
            base_url: DEFAULT_BASE_URL.to_string(), //base_url https://api.luno.com
            username: api_key_id,                   // api_key_id
            password: api_key_secret,               // api_key_secret
            path: None,
            method: Method::GET,
        }
    }

    pub async fn create_account(
        &self,
        account_name: String,
        currency_name: String,
    ) -> LunoResult<Account> {
        let path = Url::parse(format! {"{}{}", self.base_url, GET_ACCOUNTS}.as_str())?;
        let result = self
            .client
            .create_account_post(
                path,
                currency_name,
                account_name,
                &self.username,
                Some(&self.password),
            )
            .await;

        match result {
            Ok(res) => {
                let res = res.text().await;
                match res {
                    Ok(e) => {
                        let e: Account = from_str(&e).unwrap();
                        Ok(e)
                    }
                    Err(err) => Err(LunoErr {
                        kind: ErrKind::ErrInvalidArguments,
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(LunoErr {
                kind: ErrKind::ErrInvalidArguments,
                message: err.message,
            }),
        }
    }

    pub async fn get_all_balance(&self) -> LunoResult<Vec<Balance>> {
        let path = Url::parse(format! {"{}{}", self.base_url, GET_BALANCE}.as_str()).unwrap();
        let result = self
            .client
            .get_request(path, &self.username, Some(&self.password))
            .await;
        match result {
            Ok(res) => {
                let res = res.text().await;
                match res {
                    Ok(e) => {
                        let e: HashMap<String, Vec<_>> = from_str(&e).unwrap();
                        Ok(e.get("balance").unwrap().to_vec())
                    }
                    Err(err) => Err(LunoErr {
                        kind: ErrKind::ErrInvalidArguments,
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(LunoErr {
                kind: ErrKind::ErrInvalidArguments,
                message: err.message,
            }),
        }
    }

    pub async fn get_ticker(&self, pair: &String) -> LunoResult<Tickers> {
        let path = Url::parse(format! {"{}{}", self.base_url, GET_TICKER}.as_str())?;
        let result = self
            .client
            .get_request_with_params(
                path,
                (&"pair".to_string(), pair),
                &self.username,
                Some(&self.password),
            )
            .await;

        match result {
            Ok(res) => {
                let res = res.text().await;
                match res {
                    Ok(e) => {
                        let e: Tickers = from_str(&e).unwrap();
                        Ok(e)
                    }
                    Err(err) => Err(LunoErr {
                        kind: ErrKind::ErrInvalidArguments,
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_all_tickers(&self) -> LunoResult<Vec<Tickers>> {
        let path = Url::parse(format! {"{}{}", self.base_url, GET_TICKERS}.as_str())?;
        let result = self
            .client
            .get_request(path, &self.username, Some(&self.password))
            .await;

        match result {
            Ok(res) => {
                let res = res.text().await;
                match res {
                    Ok(res) => {
                        let res: HashMap<String, Vec<Tickers>> = from_str(&res)?;
                        Ok(res.get("tickers").unwrap().to_vec())
                    }
                    Err(err) => Err(LunoErr {
                        kind: ErrKind::ErrInvalidArguments,
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_full_order_book(&self, pair: &String) -> LunoResult<OrderBook> {
        let path = Url::parse(format! {"{}{}", self.base_url, GET_ORDERBOOK}.as_str())?;
        let result = self
            .client
            .get_request_with_params(
                path,
                (&"pair".to_string(), pair),
                &self.username,
                Some(&self.password),
            )
            .await;

        match result {
            Ok(res) => {
                let res = res.text().await;
                match res {
                    Ok(e) => {
                        let e: OrderBook = from_str(&e).unwrap();
                        Ok(e)
                    }
                    Err(err) => Err(LunoErr {
                        kind: ErrKind::ErrInvalidArguments,
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_top_order_book(&self, pair: &String) -> LunoResult<OrderBook> {
        let path = Url::parse(format! {"{}{}", self.base_url, GET_ORDERBOOK_TOP}.as_str())?;
        let result = self
            .client
            .get_request_with_params(
                path,
                (&"pair".to_string(), pair),
                &self.username,
                Some(&self.password),
            )
            .await;

        match result {
            Ok(res) => {
                let res = res.text().await;
                match res {
                    Ok(e) => {
                        let e: OrderBook = from_str(&e).unwrap();
                        Ok(e)
                    }
                    Err(err) => Err(LunoErr {
                        kind: ErrKind::ErrInvalidArguments,
                        message: err.to_string(),
                    }),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub async fn get_recent_trades(&self, timestamp: String, pair: String) {
        todo!()
    }
}
