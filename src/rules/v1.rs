use crate::models::api_token::V1Rule;

pub async fn check(upstream_url: &str, rules: &V1Rule) -> anyhow::Result<bool> {
    let url_components = upstream_url.split("/").collect::<Vec<&str>>();
    Ok(true)
}
