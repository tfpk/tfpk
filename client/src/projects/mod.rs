use crate::{Content, UseStateHandle};
use crate::protocols::projects_list_toml::ProjectGroupMetadata;

use std::rc::Rc;
use yew::prelude::*;

#[hook]
pub fn use_project_list() -> Vec<ProjectGroupMetadata> {
    let projects = use_context::<Rc<UseStateHandle<Content>>>().expect("No ctx found");
    let project_groups = &(*projects).project_metadata.project_groups;
    return project_groups.clone();
}
