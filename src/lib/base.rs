use std::{collections::HashMap, fmt::format};

use futures::FutureExt;
use reqwest::{header::HeaderMap, Error, Method, Response, Url};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

const DEFAULT_BASE_URL: &str = "https://api.luno.com";

#[derive(Debug)]
pub struct BaseClient {
    pub base_url: String,
    pub path: Option<Url>,
    pub method: Method,
    username: String,
    password: String,
}

impl BaseClient {
    fn init(api_key_id: String, api_key_secret: String) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            username: api_key_id,     // api_key_id
            password: api_key_secret, // api_key_secret
            path: None,
            method: Method::GET,
        }
    }
    async fn get_all_balance(&self, path: String) -> Result<HashMap<String, Vec<Balance>>, Error> {
        let path = Url::parse(format! {"{}{}", self.base_url, path}.as_str()).unwrap();
        let result = get_req(
            path,
            self.method.to_owned(),
            &self.username,
            Some(&self.password),
        )
        .await
        .expect("An error occured")
        .text()
        .await
        .unwrap();
        let result: HashMap<String, Vec<Balance>> = from_str(&format!("{}", result)).unwrap();
        Ok(result)
    }
    async fn get_all_tickers(&self, path: String) -> Result<Response, Error> {
        let path = Url::parse(format! {"{}{}", self.base_url, path}.as_str()).unwrap();
        let result = get_req(
            path,
            self.method.to_owned(),
            &self.username,
            Some(&self.password),
        )
        .await
        .expect("An error occured")
        .text()
        .await
        .unwrap();
        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Balance {
    account_id: String,
    asset: String,
    balance: String,
    reserved: String,
    unconfirmed: String,
}

async fn get_req(
    path: Url,
    method: Method,
    key: &String,
    secret: Option<&String>,
) -> Result<Response, Error> {
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

    //request.default_headers(headers)
    let get = reqwest::Client::new()
        .get(path)
        .basic_auth(key, secret.as_ref())
        .headers(headers)
        .send()
        .await
        .expect("An error occured");

    Ok(get)
}

#[tokio::test]
async fn test_something_async() {
    // for some reason, test cannot be async
    let key = String::from("gmrt4mpc487ma");
    let secret = String::from("GWcvK82lCu9O79x29GOPZ67oMK1VSMZYrVrgibGJveA");
    let init = BaseClient::init(key, secret);
    let balance = init
        .get_all_balance("/api/1/balance".to_string())
        .await
        .unwrap();
    println!("{:?}", balance.get("balance"))
}
