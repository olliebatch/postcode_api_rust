use url::Url;

pub struct PostcodeApiClient {
    base_url: Url,
    http: surf::Client,
}

impl PostcodeApiClient {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self, PostcodeApiError> {
        Ok(PostcodeApiClient {
            base_url: Url::parse(base_url.as_ref())?,
            http: surf::Client::new(),
        })
    }
}
