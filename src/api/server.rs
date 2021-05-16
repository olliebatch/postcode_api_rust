use crate::api::handlers;
use crate::api::state::State;
use crate::Config;

pub async fn start_api(config: Config, state: State) -> tide::Result<()> {
    let port = config.port;

    let mut app = tide::with_state(state);
    app.at("/healthz").get(handlers::handle_health_check);

    app.at("/postcode/:postcode")
        .get(handlers::get_location_info);

    app.at("/postcodes")
        .post(handlers::multiple_get_location_info);

    app.listen(format!("127.0.0.1:{}", port)).await?;

    Ok(())
}
