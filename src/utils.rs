use crate::config::Config;
use cmd_lib::spawn_with_output;
use reqwest::header::CONTENT_LENGTH;
use serde::{Deserialize, Serialize};

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

pub fn kf_notebooks(namespace: Option<&str>, config: &Config) -> anyhow::Result<Vec<String>> {
    let resource = &config.kubeflow.notebook_resource;
    let mut proc = match namespace {
        Some(ns) => {
            spawn_with_output!(kubectl get $resource -n $ns -o jsonpath="{.items[*].metadata.name}")
        }
        None => {
            spawn_with_output!(kubectl get $resource -A -o jsonpath="{.items[*].metadata.name}")
        }
    }?;

    Ok(proc
        .wait_with_output()?
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.to_string())
        .collect::<Vec<String>>())
}

pub fn kf_models(namespace: Option<&str>, config: &Config) -> anyhow::Result<Vec<String>> {
    let resource = &config.kubeflow.model_resource;

    let mut proc = match namespace {
        Some(ns) => {
            spawn_with_output!(kubectl get $resource -n $ns -o jsonpath="{.items[*].metadata.name}")
        }
        None => {
            spawn_with_output!(kubectl get $resource -A -o jsonpath="{.items[*].metadata.name}")
        }
    }?;

    Ok(proc
        .wait_with_output()?
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.to_string())
        .collect::<Vec<String>>())
}

pub fn all_kf_users(config: &Config) -> anyhow::Result<Vec<String>> {
    let resource = &config.kubeflow.profile_resource;

    let mut proc = spawn_with_output!(kubectl get $resource -A -o jsonpath="{.items[*].spec.owner.name}")?;

    Ok(proc
        .wait_with_output()?
        .split(" ")
        .filter(|s| *s != "")
        .map(|s| s.to_string())
        .collect::<Vec<String>>())
}

pub fn kf_user_namespace(user: &str, config: &Config) -> anyhow::Result<String> {
    let resource = &config.kubeflow.profile_resource;

    let mut proc = spawn_with_output!(kubectl get $resource -A -o jsonpath="{.items}")?;

    let res: serde_json::Value = serde_json::from_str(proc.wait_with_output()?.as_str())?;
    let fres = res
        .as_array()
        .unwrap()
        .iter()
        .filter(|e| e["spec"]["owner"]["name"] == user)
        .map(|v| v["metadata"]["name"].as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    match fres.first() {
        Some(s) => Ok(s.to_owned()),
        None => Ok(String::default()),
    }
}
