use gloo_net::http::Request;
use yew::prelude::*;

use yew::classes;

use markdown::{to_html_with_options, Options};
use crate::{Content, UseStateHandle};

use std::rc::Rc;

use crate::protocols::blog_list_toml::BlogMetadata;

#[derive(Properties, PartialEq, Clone)]
pub struct BlogPageProps {
    pub slug: String,
}

#[function_component(BlogPage)]
pub fn blog_page(props: &BlogPageProps) -> Html {
    let blog_metadata = crate::blogs::use_blog(&props.slug);
    let state = use_context::<Rc<UseStateHandle<Content>>>().expect("No ctx found");

    let blog_content: Rc<UseStateHandle<Option<String>>> = Rc::new(use_state(|| None));

    {
        let blog_content = blog_content.clone();
        let blog_metadata_closure = blog_metadata.clone();
        use_effect_with_deps(
            |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(found_blog) = blog_metadata_closure {
                        let url = &state.url;
                        let req_path = &format!("{url}/static/content/{}", found_blog.md_path);
                        let blogs_content = Request::new(req_path)
                            .send()
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                        blog_content.set(Some(blogs_content));
                    } else {
                        blog_content.set(Some("No blog found.".to_string()));
                    }
                });
            },
            blog_metadata.clone(),
        );
    }

    let blog_text = &(**blog_content)
        .clone()
        .unwrap_or("# Loading...".to_string());
    let blog_img = if let Some(BlogMetadata { ref img_path, ..}) = blog_metadata {html! {
        <img src={img_path.clone()} />
    }} else { html!{<></>} };
    let parsed = yew::Html::from_html_unchecked(yew::AttrValue::from(to_html_with_options(blog_text, markdown::Options::gfm())));
    html! {
        <div class={classes!("container", "blog-content" ,"mb-4")}>
            {blog_img}
            {parsed}
        </div>
    }
}
