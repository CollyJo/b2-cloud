use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub api_info: ApiInfo,
    pub authorization_token: String,
}

impl Default for AuthResponse {
    fn default() -> Self {
        Self {
            api_info: ApiInfo::default(),
            authorization_token: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    pub storage_api: StorageApi,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StorageApi {
    pub api_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadUrlResponse {
    pub authorization_token: String,
    pub upload_url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BucketRequest {
    pub bucket_id: String,
}
