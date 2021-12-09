use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{config::Config, utils};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rule {
    pub v1: Option<V1Rule>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct V1Rule {
    pub pipeline: bool,
    pub models: HashMap<String, bool>,
    pub notebooks: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCore {
    pub name: String,
    pub rules: Rule,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub id: i32,
    #[serde(flatten)]
    pub core: TokenCore,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIToken {
    #[serde(flatten)]
    pub data: TokenData,
    pub last_used: String,
}

#[derive(Serialize, Debug)]
pub struct TokenDTO {
    pub tokens: Vec<APIToken>,
    pub template: TokenCore,
}

impl Default for TokenCore {
    fn default() -> Self {
        Self {
            name: String::default(),
            rules: Rule::default(),
            start_date: String::from("1970-01-01"),
            end_date: String::from("1970-01-01"),
        }
    }
}

impl TokenCore {
    pub async fn better_default(&mut self, config: &Config) -> anyhow::Result<Self> {
        self.rules = self.rules.clone().better_default(config).await?;
        Ok(self.clone())
    }
}

impl Default for TokenData {
    fn default() -> Self {
        Self {
            id: -1,
            core: TokenCore::default(),
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            v1: Some(V1Rule::new()),
        }
    }
}

impl Rule {
    pub async fn better_default(&mut self, config: &Config) -> anyhow::Result<Self> {
        self.v1 = Some(self.v1.clone().unwrap().better_default(config).await?);
        Ok(self.clone())
    }
}

impl V1Rule {
    pub fn new() -> Self {
        Self {
            pipeline: true,
            ..Self::default()
        }
    }

    pub async fn better_default(&mut self, config: &Config) -> anyhow::Result<Self> {
        self.notebooks = HashMap::from_iter(
            utils::get_resource(config, utils::Resource::Notebook)
                .await?
                .iter()
                .map(|r| (r.name.clone(), true)),
        );

        self.models = HashMap::from_iter(
            utils::get_resource(config, utils::Resource::Model)
                .await?
                .iter()
                .map(|r| (r.name.clone(), true)),
        );

        Ok(self.clone())
    }
}
