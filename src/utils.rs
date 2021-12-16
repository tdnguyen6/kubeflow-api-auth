use std::{fs::File, io::Read};
use reqwest::header::CONTENT_LENGTH;
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize)]
struct Req {
    secret: String,
    response: String,
}

#[derive(Debug, Deserialize)]
struct Res {
    success: bool,
    #[serde(rename = "challenge_ts")]
    _challenge_ts: String,
    #[serde(rename = "hostname")]
    _hostname: String,
    #[serde(rename = "error-codes")]
    _error_codes: Option<Vec<u32>>,
}

pub async fn verify_recaptcha(config: &Config, token: &String) -> anyhow::Result<bool> {
    let client = reqwest::Client::new();
    let res: Res = client
        .post(format!(
            "https://www.google.com/recaptcha/api/siteverify?secret={}&response={}",
            config.recaptcha.secret.clone(),
            token.clone(),
        ))
        .header(CONTENT_LENGTH, 0)
        .send()
        .await
        .expect("recaptcha verify request error")
        .json()
        .await
        .expect("recaptcha verify response decode error");

    Ok(res.success)
}

#[derive(Deserialize)]
pub struct ResourceModel {
    pub name: String,
    pub uid: String,
}

pub enum Resource {
    Notebook,
    Model,
    Profile,
}

pub async fn get_resource(
    config: &Config,
    resource: Resource,
) -> anyhow::Result<Vec<ResourceModel>> {
    let mut buf = Vec::new();
    File::open(format!("{}/ca.crt", config.service_account))?.read_to_end(&mut buf)?;
    let client = reqwest::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(&buf)?)
        .build()?;

    let token = std::fs::read_to_string(format!("{}/token", config.service_account))?;

    let url = format!(
        "{}/apis/{}",
        &config.apiserver,
        match resource {
            Resource::Notebook => &config.kubeflow.notebook_resource,
            Resource::Model => &config.kubeflow.model_resource,
            Resource::Profile => &config.kubeflow.profile_resource,
        }
    );

    let res: serde_json::Value = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?
        .json()
        .await?;

    Ok(res
        .get("items")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|i| ResourceModel {
            name: i["metadata"]["name"].as_str().unwrap().to_string(),
            uid: i["metadata"]["uid"].as_str().unwrap().to_string(),
        })
        .collect())
}
