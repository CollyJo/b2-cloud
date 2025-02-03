use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)] // Убрали ручную реализацию Default
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub api_info: ApiInfo,
    pub authorization_token: String,
}

#[derive(Serialize, Deserialize, Debug, Default)] // Добавили Default
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    pub storage_api: StorageApi,
}

#[derive(Serialize, Deserialize, Debug, Default)] // Добавили Default
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
