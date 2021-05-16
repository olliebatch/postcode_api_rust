use crate::Config;
use tide::{Request, Response, StatusCode};

pub async fn start_api(config: Config) -> tide::Result<()> {
    let port = config.port;

    let mut app = tide::new();
    app.at("/healthz").get(handle_health_check);

    app.listen(format!("127.0.0.1:{}", port)).await?;

    Ok(())
}

pub async fn handle_health_check(_: Request<()>) -> tide::Result<Response> {
    let mut res = Response::new(StatusCode::NoContent);
    res.set_body("");
    Ok(res)
}
