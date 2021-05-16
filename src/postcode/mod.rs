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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Postcodes(Vec<Postcode>);

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

impl Postcodes {
    pub fn new(postcodes: Vec<Postcode>) -> Self {
        Postcodes(postcodes)
    }
    pub fn get_postcodes_vec(self) -> Vec<String> {
        self.0
            .iter()
            .map(|postcode| postcode.postcode.clone())
            .collect()
    }

    pub async fn with_locations(
        self,
        postcode_api: PostcodeApiClient,
    ) -> Result<Self, PostcodeApiErrors> {
        let result = postcode_api.get_many_post_codes_info(self).await;
        match result {
            Ok(postcodes) => {
                let postcode = postcodes
                    .result
                    .into_iter()
                    .map(|postcode_response| postcode_response.result.into())
                    .collect::<Vec<Postcode>>();
                Ok(Postcodes(postcode))
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::postcode::{Postcode, Postcodes};
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
        server.mock(|when, then| {
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

    #[async_std::test]
    async fn multiple_with_locations() {
        // This would be nicer to mock the trait of the profile api client.
        // For the essence of time i've left this as the same code for profile client can work here.
        let postcodes = vec![
            Postcode::new("OX49 5NU".to_owned(), None),
            Postcode::new("M32 0JG".to_owned(), None),
            Postcode::new("NE30 1DP".to_owned(), None),
        ];
        let postcode_wrapper = Postcodes::new(postcodes);
        // Arrange
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method("POST").path_contains("/postcodes");
            then.status(200)
                .json_body(json!({ "status": 200, "result":[{
                        "query": "OX49 5NU",
                         "result": {
                        "postcode": "OX49 5NU",
                        "longitude": -1.069752,
                        "latitude": 51.655929}
                },
                    {
                        "query": "M32 0JG",
                         "result": {
                        "postcode": "M32 0JG",
                        "longitude": -2.302836,
                        "latitude": 53.455654
                    }},
                        {
                        "query": "NE30 1DP",
                         "result": {
                        "postcode": "NE30 1DP",
                        "longitude": -1.439269,
                        "latitude": 55.011303
                        }
                            }
                ]}
                ));
        });

        let client = PostcodeApiClient::new(&server.base_url()).unwrap();

        let postcode_with_loc = postcode_wrapper.with_locations(client).await.unwrap();

        insta::assert_json_snapshot!(postcode_with_loc)
    }
}
