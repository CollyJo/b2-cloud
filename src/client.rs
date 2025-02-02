use crate::{
    error::B2Error,
    models::{AuthResponse, BucketRequest, UploadUrlResponse},
    utils::{calculate_sha1, encode_file_name, generate_file_name},
};
use reqwest::{header, Client};

pub struct B2Uploader {
    key_id: String,
    key_name: String,
    bucket_id: String,
    client: Client,
}

impl B2Uploader {
    pub fn new(key_id: String, key_name: String, bucket_id: String) -> Self {
        let client = Client::new();
        Self {
            key_id,
            key_name,
            bucket_id,
            client,
        }
    }

    pub async fn authenticate(&self) -> Result<AuthResponse, B2Error> {
        Ok(self
            .client
            .get("https://api.backblazeb2.com/b2api/v3/b2_authorize_account")
            .basic_auth(&self.key_id, Some(&self.key_name))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_upload_url(
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

    pub async fn upload_file(
        &self,
        file_content: Vec<u8>,
        upload_url_response: &UploadUrlResponse,
    ) -> Result<serde_json::Value, B2Error> {
        let cloud_name = generate_file_name();

        let sha1_hash = calculate_sha1(&file_content);
        let encoded_file_name = encode_file_name(&cloud_name);

        let response = self
            .client
            .post(&upload_url_response.upload_url)
            .header(
                header::AUTHORIZATION,
                &upload_url_response.authorization_token,
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
