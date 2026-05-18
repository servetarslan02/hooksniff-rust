// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct CustomDomainIn {
    pub domain: String,
}

impl CustomDomainIn {
    pub fn new(domain: String) -> Self {
        Self { domain }
    }
}
