pub use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Default, Clone)]
pub struct Config {
    pub server: ServerCfg,
    pub kubeflow: KubeflowCfg,
    pub database_url: String,
    pub database_maxcon: u32,
    pub jwt_secret: String,
    pub recaptcha: RecaptchaCfg,
    pub apiserver: String,
    pub service_account: String,
}

#[derive(Deserialize, Default, Clone)]
pub struct ServerCfg {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Default, Clone)]
pub struct KubeflowCfg {
    pub userid_header: String,
    pub interactive_endpoint: String,
    pub model_resource: String,
    pub notebook_resource: String,
}

#[derive(Deserialize, Default, Clone)]
pub struct RecaptchaCfg {
    pub secret: String,
    pub site_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
