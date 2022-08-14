#[allow(unused)]
use crate::luno::error::LunoErr;
use crate::luno::LunoResult;
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Clone)]
pub(crate) struct LunoClientBuilder {
    headers: HeaderMap,
    pub(crate) client: Client,
}

impl LunoClientBuilder {
    pub async fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::USER_AGENT,
            "rust-luno-api v0.1".parse().unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
        headers.insert(reqwest::header::ACCEPT_CHARSET, "utf-8".parse().unwrap());

        Self {
            headers,
            client: Client::new(),
        }
    }

    pub async fn get_request(
        &self,
        path: Url,
        key: &String,
        secret: Option<&String>,
    ) -> LunoResult<Response> {
        self.client
            .get(path)
            .basic_auth(key, secret.as_ref())
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(|err| LunoErr::from(err.to_string()))
    }

    pub async fn get_request_with_params(
        &self,
        path: Url,
        param: (&String, &String),
        key: &String,
        secret: Option<&String>,
    ) -> LunoResult<Response> {
        let mut params = HashMap::new();
        params.insert(param.0, param.1);
        self.client
            .get(path)
            .basic_auth(key, secret.as_ref())
            .headers(self.headers.clone())
            .query(&[(param.0, param.1)])
            .send()
            .await
            .map_err(|err| LunoErr::from(err.to_string()))
    }

    pub async fn create_account_post(
        &self,
        path: Url,
        currency_name: String,
        account_name: String,
        key: &String,
        secret: Option<&String>,
    ) -> LunoResult<Response> {
        self.client
            .post(path)
            .basic_auth(key, secret.as_ref())
            .headers(self.headers.clone())
            .query(&[("currency", currency_name), ("name", account_name)])
            .send()
            .await
            .map_err(|err| LunoErr::from(err.to_string()))
    }
}
