use crate::api::handlers;
use crate::Config;

pub async fn start_api(config: Config) -> tide::Result<()> {
    let port = config.port;

    let mut app = tide::new();
    app.at("/healthz").get(handlers::handle_health_check);

    app.at("/postcode/:postcode")
        .get(handlers::get_location_info);

    app.listen(format!("127.0.0.1:{}", port)).await?;

    Ok(())
}
