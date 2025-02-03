use crate::{
    error::B2Error,
    models::{ApiInfo, AuthResponse, BucketRequest, StorageApi, UploadUrlResponse},
    utils::{calculate_sha1, encode_file_name, generate_file_name},
};
use reqwest::{header, Client};

pub struct B2Uploader {
    client: Client,
    auth_response: AuthResponse,
    upload_url_response: UploadUrlResponse,
    key_id: String,
    key_name: String,
    bucket_id: String,
}

impl B2Uploader {
    pub async fn new() -> Result<Self, B2Error> {
        dotenv::dotenv().ok();

        let key_id = std::env::var("B2_KEY_ID")
            .map_err(|_| B2Error::ConfigError("B2_KEY_ID not set".into()))?;
        let key_name = std::env::var("B2_KEY_NAME")
            .map_err(|_| B2Error::ConfigError("B2_KEY_NAME not set".into()))?;
        let bucket_id = std::env::var("B2_BUCKET_ID")
            .map_err(|_| B2Error::ConfigError("B2_BUCKET_ID not set".into()))?;

        let client = Client::new();
        let temp_uploader = Self {
            client: client.clone(),
            auth_response: AuthResponse::default(),
            upload_url_response: UploadUrlResponse {
                upload_url: String::new(),
                authorization_token: String::new(),
            },
            key_id: key_id.clone(),
            key_name: key_name.clone(),
            bucket_id: bucket_id.clone(),
        };

        let auth_response = temp_uploader.authenticate().await?;
        let upload_url_response = temp_uploader.get_upload_url(&auth_response).await?;

        Ok(Self {
            client,
            auth_response,
            upload_url_response,
            key_id,
            key_name,
            bucket_id,
        })
    }

    async fn authenticate(&self) -> Result<AuthResponse, B2Error> {
        Ok(self
            .client
            .get("https://api.backblazeb2.com/b2api/v3/b2_authorize_account")
            .basic_auth(&self.key_id, Some(&self.key_name))
            .send()
            .await?
            .json()
            .await?)
    }

    async fn get_upload_url(
        &self,
        auth_response: &AuthResponse,
    ) -> Result<UploadUrlResponse, B2Error> {
        Ok(self
            .client
            .post(&format!(
                "{}/b2api/v3/b2_get_upload_url",
                auth_response.api_info.storage_api.api_url
            ))
            .header(header::AUTHORIZATION, &auth_response.authorization_token)
            .header(header::CONTENT_TYPE, "application/json")
            .json(&BucketRequest {
                bucket_id: self.bucket_id.clone(),
            })
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn upload_file(&self, file_content: Vec<u8>) -> Result<serde_json::Value, B2Error> {
        let cloud_name = generate_file_name();

        let sha1_hash = calculate_sha1(&file_content);
        let encoded_file_name = encode_file_name(&cloud_name);

        let response = self
            .client
            .post(&self.upload_url_response.upload_url)
            .header(
                header::AUTHORIZATION,
                &self.upload_url_response.authorization_token,
            )
            .header("X-Bz-File-Name", encoded_file_name)
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::CONTENT_LENGTH, file_content.len().to_string())
            .header("X-Bz-Content-Sha1", sha1_hash)
            .body(file_content)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            let status = response.status().as_u16();
            let message = response.text().await?;
            Err(B2Error::UploadError {
                status_code: status,
                message,
            })
        }
    }
}
