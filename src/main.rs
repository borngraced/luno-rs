mod lib;
use futures::executor::block_on;
use lib::error;
use reqwest::{Method, Url};

#[tokio::main]
async fn main() {
    //     let url = Url::parse("https://api.luno.com/api/1/accounts").unwrap();
    //     let client = Client2 {
    //         method: Method::GET,
    //         base: url,
    //     };
    //   get_req(client, true, None, "music".to_string()).await;
}
