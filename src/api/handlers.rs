use crate::postcode::Postcode;
use tide::{Request, Response, StatusCode};

pub async fn handle_health_check(_: Request<()>) -> tide::Result<Response> {
    let mut res = Response::new(StatusCode::NoContent);
    res.set_body("");
    Ok(res)
}

pub async fn get_location_info(req: Request<()>) -> tide::Result<Response> {
    let postcode = req.param("postcode");

    if let Ok(input_postcode) = postcode {
        let postcode = Postcode::new(input_postcode.to_owned(), None);
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(serde_json::to_string(&postcode)?);
        res.append_header("Content-Type", "application/json");
        return Ok(res);
    }
    Err(tide::Error::from_str(
        StatusCode::BadRequest,
        "Invalid PostCode Provided",
    ))
}
