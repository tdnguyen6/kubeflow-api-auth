use crate::models::api_token::V1Rule;

pub async fn check(upstream_url: &str, _rules: &V1Rule) -> anyhow::Result<bool> {
// "https://kubeflow.k8s.tidu.giize.com/notebook/tidu-nguyen-2000/scipy-1/lab?"
    let _url_components = upstream_url.split("/").collect::<Vec<&str>>();
    Ok(true)
}
