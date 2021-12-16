use crate::models::api_token::V1Rule;

pub async fn check(upstream_url: &str, rules: &V1Rule) -> anyhow::Result<bool> {
    let url_components = upstream_url.split("/").collect::<Vec<&str>>();
    if url_components.len() > 0 {
        if url_components[0] == "pipeline" {
            return Ok(rules.pipeline);
        } else if url_components[0] == "notebook" {
            if url_components.len() >= 3 {
                if rules.notebooks.contains_key(url_components[2]) {
                    return Ok(rules.notebooks[url_components[2]]);
                }
            }
        } else if url_components[0] == "v2" {
            if url_components.len() >= 3 && url_components[1] == "models" {
                if rules.models.contains_key(url_components[2]) {
                    return Ok(rules.notebooks[url_components[2]]);
                }
            }
        }
    }
    Ok(false)
}
