use super::error::ErrorResponse;
use crate::api::state;
use crate::postcode::{Postcode, Postcodes};
use tide::{Request, Response, StatusCode};

pub async fn handle_health_check(_: Request<state::State>) -> tide::Result<Response> {
    let mut res = Response::new(StatusCode::NoContent);
    res.set_body("");
    Ok(res)
}

pub async fn get_location_info(req: Request<state::State>) -> tide::Result<Response> {
    let postcode = req.param("postcode");

    let postcode_api = req.state().clone().postcode_api()?;

    if let Ok(input_postcode) = postcode {
        let space_correction = input_postcode.replace("%20", "");
        let postcode = Postcode::new(space_correction, None);
        let postcode_with_loc = postcode
            .with_location(postcode_api)
            .await
            .map_err(ErrorResponse::from);

        if postcode_with_loc.is_err() {
            let error = postcode_with_loc.unwrap_err();
            return Ok(error.into());
        }

        let mut res = Response::new(StatusCode::Ok);
        res.set_body(serde_json::to_string(&postcode_with_loc.unwrap())?);
        res.append_header("Content-Type", "application/json");
        return Ok(res);
    }
    Err(tide::Error::from_str(
        StatusCode::BadRequest,
        "Invalid PostCode Provided",
    ))
}

pub async fn multiple_get_location_info(mut req: Request<state::State>) -> tide::Result<Response> {
    let post_code_request: Vec<String> = req.body_json().await?;
    let postcodes: Vec<Postcode> = post_code_request
        .iter()
        .map(|postcode| Postcode::new(postcode.replace("%20", ""), None))
        .collect();

    let postcode_wrapper = Postcodes::new(postcodes);

    let postcode_api = req.state().clone().postcode_api()?;

    let postcodes_with_loc = postcode_wrapper.with_locations(postcode_api).await?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(serde_json::to_string(&postcodes_with_loc)?);
    res.append_header("Content-Type", "application/json");
    return Ok(res);
}
