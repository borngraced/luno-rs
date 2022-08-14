use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub currency: String,
    pub id: String,
    pub name: String,
    pub pending: Vec<Pending>,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub account_id: String,
    pub available: String,
    pub available_delta: String,
    pub balance: String,
    pub balance_delta: String,
    pub currency: String,
    pub description: String,
    pub details_field: DetailsField,
    pub details: String,
    pub kind: Kind,
    pub row_index: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DetailsField {
    pub crypto_details: CryptoDetails,
    pub trade_details: TradeDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CryptoDetails {
    pub address: String,
    pub txid: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TradeDetails {
    pub pair: String,
    pub price: String,
    pub sequence: i64,
    pub volume: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Kind {
    Fee,
    Transfer,
    Exchange,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Balance {
    pub account_id: String,
    pub asset: String,
    pub balance: String,
    pub reserved: String,
    pub unconfirmed: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tickers {
    pub pair: String,
    pub timestamp: i64,
    pub bid: String,
    pub ask: String,
    pub last_trade: String,
    pub rolling_24_hour_volume: String,
    pub status: Status,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Status {
    Active,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ticker {
    pub pair: String,
    pub timestamp: i64,
    pub bid: String,
    pub ask: String,
    pub last_trade: String,
    pub rolling_24_hour_volume: String,
    pub status: Status,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OrderBook {
    pub timestamp: usize,
    pub asks: Vec<Asks>,
    pub bids: Vec<Bids>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Asks {
    pub price: String,
    pub volume: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Bids {
    pub price: String,
    pub volume: String,
}
