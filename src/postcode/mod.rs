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
}
