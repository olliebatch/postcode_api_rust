use tide::{Request, Response, StatusCode};

pub async fn handle_health_check(_: Request<()>) -> tide::Result<Response> {
    let mut res = Response::new(StatusCode::NoContent);
    res.set_body("");
    Ok(res)
}

pub async fn get_location_info(_: Request<()>) -> tide::Result<Response> {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body("51.5074, 0.1278");
    Ok(res)
}
