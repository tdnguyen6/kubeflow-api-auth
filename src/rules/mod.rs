use crate::models::api_token::Rule;

mod v1;

pub async fn check(upstream_url: &str, rules: &Rule) -> anyhow::Result<bool> {
  // newer versions of rule should appear before
  if let Some(rule) = &rules.v1 {
    return v1::check(upstream_url, &rule).await
  }

  Ok(false)
}
