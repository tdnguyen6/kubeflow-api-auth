use std::{collections::HashMap, iter::FromIterator};

use serde::{Deserialize, Serialize};

use crate::{utils};

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
        let today = chrono::Utc::now();
        Self {
            name: String::default(),
            rules: Rule::default(),
            start_date: today.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }
}

impl TokenCore {
    pub fn better_default(&mut self, kf_user: &str) -> anyhow::Result<Self> {
        self.rules = self.rules.clone().better_default(kf_user)?;
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
    pub fn better_default(&mut self, kf_user: &str) -> anyhow::Result<Self> {
        self.v1 = Some(self.v1.clone().unwrap().better_default(kf_user)?);
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

    pub fn better_default(&mut self, kf_user: &str) -> anyhow::Result<Self> {
        let kf_user_ns = utils::kf_user_namespace(kf_user)?;

        self.notebooks = HashMap::from_iter(
            utils::kf_notebooks(Some(kf_user_ns.as_str()))?
                .iter()
                .map(|r| (r.to_owned(), false)),
        );

        self.models = HashMap::from_iter(
            utils::kf_models(Some(kf_user_ns.as_str()))?
                .iter()
                .map(|r| (r.to_owned(), false)),
        );

        Ok(self.clone())
    }
}
