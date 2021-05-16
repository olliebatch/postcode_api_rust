use crate::postcode_api::api_client::{PostCodeDetails, PostcodeApiClient, PostcodeApiErrors};
use serde::{Deserialize, Serialize};

// Make Location Optional as assumption that this may not come back from api with all postcodes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Postcode {
    pub postcode: String,
    pub location: Option<Location>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Location {
    latitude: f64,
    longitude: f64,
}

impl Postcode {
    pub fn new(postcode: String, location: Option<Location>) -> Self {
        Postcode { postcode, location }
    }

    pub async fn with_location(
        self,
        postcode_api: PostcodeApiClient,
    ) -> Result<Self, PostcodeApiErrors> {
        let postcode_info = postcode_api.get_post_code_info(self.postcode).await?;

        Ok(postcode_info.result.into())
    }
}

impl From<PostCodeDetails> for Postcode {
    fn from(client_response: PostCodeDetails) -> Self {
        Postcode::new(
            client_response.postcode,
            Some(Location {
                latitude: client_response.latitude,
                longitude: client_response.longitude,
            }),
        )
    }
}
