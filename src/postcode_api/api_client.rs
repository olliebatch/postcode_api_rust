use crate::postcode::Postcodes;
use surf::StatusCode;
use url::Url;

pub struct PostcodeApiClient {
    base_url: Url,
    http: surf::Client,
}

#[derive(thiserror::Error, Debug)]
pub enum PostcodeApiErrors {
    #[error("Malformed URL: {0}")]
    MalformedUrl(#[from] url::ParseError),
    #[error("Bad Response from Postcode Api client")]
    BadResponse,
    #[error("Http Client Error")]
    HttpClientError,
    #[error("Other Error")]
    OtherError,
}

impl From<surf::Error> for PostcodeApiErrors {
    fn from(_: surf::Error) -> Self {
        Self::HttpClientError
    }
}

impl PostcodeApiClient {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self, PostcodeApiErrors> {
        Ok(PostcodeApiClient {
            base_url: Url::parse(base_url.as_ref())?,
            http: surf::Client::new(),
        })
    }

    pub async fn get_post_code_info(
        self,
        postcode: String,
    ) -> Result<PostcodeClientResponse, PostcodeApiErrors> {
        let request = self
            .http
            .get(format!("{}/postcodes/{}", self.base_url, postcode));

        let mut response = request.await?;
        if response.status() != StatusCode::Ok {
            println!("{:?}", response.status());
            return Err(PostcodeApiErrors::BadResponse);
        }
        let postcode_details = response.body_json().await?;
        Ok(postcode_details)
    }

    pub(crate) async fn get_many_post_codes_info(
        &self,
        postcodes: Postcodes,
    ) -> Result<MultiplePostcodesClientResponse, PostcodeApiErrors> {
        let body = serde_json::to_string(&PostcodesInput {
            postcodes: postcodes.get_postcodes_vec(),
        })
        .map_err(|_| PostcodeApiErrors::OtherError)?;
        let request = self
            .http
            .post(
                self.base_url
                    .join(&*format!("./postcodes"))
                    .expect("Malformed URL"),
            )
            .body(body)
            .header("Content-Type", "application/json");

        let mut response = request.await?;

        if response.status() != StatusCode::Ok {
            return Err(PostcodeApiErrors::BadResponse);
        }

        Ok(response
            .body_json::<MultiplePostcodesClientResponse>()
            .await?)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostcodeClientResponse {
    pub status: i64,
    pub result: PostCodeDetails,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCodeDetails {
    pub postcode: String,
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostcodesInput {
    postcodes: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiplePostcodesClientResponse {
    pub status: i64,
    pub result: Vec<MultiplePostCodeResult>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MultiplePostCodeResult {
    pub query: String,
    pub result: PostCodeDetails,
}

#[cfg(test)]
mod tests {
    use crate::postcode_api::api_client::PostcodeApiClient;
    use httpmock::MockServer;
    use serde_json::json;

    #[async_std::test]
    async fn get_postcode_details() {
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
        let result = client
            .get_post_code_info(postcode.to_owned())
            .await
            .unwrap();

        insta::assert_json_snapshot!(result)
    }
}
