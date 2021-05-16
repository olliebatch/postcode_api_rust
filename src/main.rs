use postcode_api_rust::api::start_api;
use postcode_api_rust::{Config, State};

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = Config::new().expect("Couldn't load config");
    let state = State::new(config.postcode_api_base_url.clone());
    println!("Starting with config: \n{}", config);
    start_api(config, state).await.expect("Couldn't start api");
}
