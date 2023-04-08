use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Project {
    pub title: String,
    pub upper_time: String,
    pub lower_time: String,
    pub summary: String,
    #[serde(default)]
    pub link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct ProjectGroupMetadata {
    pub title: String,
    pub short_text: String,
    pub img_path: String,
    #[serde(rename="project", default)]
    pub projects: Vec<Project>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Default)]
pub struct ProjectsMetadata {
    #[serde(rename="project_group", default)]
    pub project_groups: Vec<ProjectGroupMetadata>,
}

impl ProjectsMetadata {
    pub fn from_text(text: &str) -> Self {
        toml::from_str(text).unwrap()
    }
}
