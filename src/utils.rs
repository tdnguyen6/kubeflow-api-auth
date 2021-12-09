use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize)]
struct Req {
    secret: String,
    response: String,
}

#[derive(Deserialize)]
struct Res {
    success: bool,
    challenge_ts: String,
    hostname: String,
    #[serde(rename = "error-codes")]
    error_codes: Vec<u32>,
}

pub async fn verify_recaptcha(config: &Config, token: &String) -> anyhow::Result<bool> {
    return Ok(true);
    let client = reqwest::Client::new();
    let res: Res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .json(&Req {
            secret: config.recaptcha.secret.clone(),
            response: token.clone(),
        })
        .send()
        .await?
        .json()
        .await?;

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
    Profile
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
            Notebook => &config.kubeflow.notebook_resource,
            Model => &config.kubeflow.model_resource,
            Profile => &config.kubeflow.profile_resource,
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
