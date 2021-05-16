use postcode_api_rust::Config;

fn main() {
    dotenv::dotenv().ok();
    let config = Config::new().expect("Couldn't load config");
}
