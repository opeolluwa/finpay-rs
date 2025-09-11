use std::{fs, path::Path};

use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderValue},
    multipart,
};

use serde::{Deserialize, Serialize};

use crate::ImagekitError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagekitUploadResponse {
    pub file_id: String,
    pub name: String,
    pub size: u64,
    pub version_info: VersionInfo,
    pub file_path: String,
    pub url: String,
    pub file_type: String,
    pub ai_tags: Option<serde_json::Value>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfo {
    pub id: String,
    pub name: String,
}

pub struct ImagekitClient {
    client: Client,
    upload_url: String,
    public_key: String,
    private_key: String,
}

impl ImagekitClient {
    pub fn new(public_key: &str, private_key: &str) -> Result<Self, ImagekitError> {
        Ok(Self {
            client: Client::builder().build()?,
            upload_url: "https://upload.imagekit.io/api/v1/files/upload".to_string(),
            public_key: public_key.to_string(),
            private_key: private_key.to_string(),
        })
    }

    pub async fn upload_file<P: AsRef<Path>>(
        &self,
        path: P,
        fine_name: &str,
    ) -> Result<ImagekitUploadResponse, ImagekitError> {
        let file_bytes = fs::read(&path)?;
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Basic {}", self.private_key))?,
        );

        let form = multipart::Form::new()
            .part(
                "file",
                multipart::Part::bytes(file_bytes).file_name(fine_name.to_string()),
            )
            .text("fileName", fine_name.to_string())
            .text("publicKey", self.public_key.clone());

        let response = self
            .client
            .request(Method::POST, &self.upload_url)
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ImagekitError::UploadFailed(format!(
                "Upload failed: {}",
                response.status()
            )));
        }

        let parsed = response.json::<ImagekitUploadResponse>().await?;
        Ok(parsed)
    }
}
