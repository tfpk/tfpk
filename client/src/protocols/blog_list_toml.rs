use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct BlogMetadata {
    pub title: String,
    pub slug: String,
    pub lede: String,
    pub tags: Vec<String>,
    pub md_path: String,
    pub img_path: String,
    pub published: toml::value::Datetime,
    pub last_updated: toml::value::Datetime,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct BlogListMetadata {
    pub blogs: Vec<BlogMetadata>,
}

impl BlogListMetadata {
    pub fn from_text(text: &str) -> Self {
        toml::from_str(text).unwrap()
    }
}
