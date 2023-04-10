use crate::protocols::blog_list_toml::BlogMetadata;
use crate::{Content, UseStateHandle};

use std::rc::Rc;
use yew::prelude::*;

#[hook]
pub fn use_blog_list() -> Vec<BlogMetadata> {
    let blogs = use_context::<Rc<UseStateHandle<Content>>>().expect("No ctx found");
    let blog_list = &blogs.blog_metadata.blogs;
    blog_list.clone()
}

#[hook]
pub fn use_recent_blog() -> Option<BlogMetadata> {
    let blogs = use_context::<Rc<UseStateHandle<Content>>>().expect("No ctx found");
    let blog_list = &mut blogs.blog_metadata.blogs.clone();
    blog_list.sort_by_key(|blog| blog.last_updated);
    return blog_list.get(0).cloned();
}

#[hook]
pub fn use_blog(slug: &str) -> Option<BlogMetadata> {
    let blog_list = use_blog_list();
    let found_blog = blog_list.iter().find(|blog| blog.slug == slug);

    found_blog.cloned()
}
