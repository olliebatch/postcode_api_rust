use postcode_api_rust::api::start_api;
use postcode_api_rust::Config;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = Config::new().expect("Couldn't load config");

    start_api(config).await.expect("Couldn't start api");
}
