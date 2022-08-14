///! Rust Luno Api client (async rust)
///
/// INSTALLATION
/// Add to Cargo.toml
/// -> luno-rust-api = "0.0.1"
///
///
/// USAGE
///  requires a key and secret that can be found in your luno account settings
///```rust
/// use env;
/// use luno_rust_api::Luno;
///
/// #[tokio::test]
/// async fn main(){
/// dotenv::dotenv().ok();
///	let key = env::var("API_KEY").expect("Api Key doesn't exist yet, please add");
///	let secret = env::var("API_SECRET").expect("Api Key Secret doesn't exist yet, please add");
///	let luno = Luno::init(key, secret).await;
///	let tickers = luno.get_all_balance().await;
///	println!("{:#?}", json!(tickers.as_ref().unwrap())); // data can be serialized to json)
///	assert_eq!(true, tickers.is_ok());
/// }
/// ```
///
///
mod luno;
pub use luno::Luno;
