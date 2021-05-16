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

#[cfg(test)]
mod tests {
    use crate::postcode::Postcode;
    use crate::postcode_api::api_client::PostcodeApiClient;
    use httpmock::MockServer;
    use serde_json::json;

    #[async_std::test]
    async fn with_location() {
        // This would be nicer to mock the trait of the profile api client.
        // For the essence of time i've left this as the same code for profile client can work here.

        let postcode = "WC2N 5DU";
        // Arrange
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method("GET").path_contains("/postcodes");
            then.status(200)
                .json_body(json!({ "status": 200, "result": {
                    "postcode": "WC2N 5DU",
                    "longitude": -0.128294,
                    "latitude": 51.507209,
                } }));
        });
        let client = PostcodeApiClient::new(&server.base_url()).unwrap();

        // Act
        let postcode_struct = Postcode::new(postcode.to_owned(), None);

        let postcode_with_loc = postcode_struct.with_location(client).await.unwrap();

        insta::assert_json_snapshot!(postcode_with_loc)
    }
}
