use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct LangYaml(pub HashMap<String, LocalizedText>);

impl Deref for LangYaml {
    type Target = HashMap<String, LocalizedText>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct LocalizedText {
    #[serde(flatten)]
    pub elem: HashMap<String, String>,
}

impl Deref for LocalizedText {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}
