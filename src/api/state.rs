use crate::postcode_api::api_client::{PostcodeApiClient, PostcodeApiErrors};

#[derive(Clone)]
pub struct State {
    postcode_api_base_url: String,
}

impl State {
    pub fn new(postcode_api_base_url: String) -> Self {
        State {
            postcode_api_base_url,
        }
    }

    pub fn postcode_api(self) -> Result<PostcodeApiClient, PostcodeApiErrors> {
        PostcodeApiClient::new(self.postcode_api_base_url)
    }
}
