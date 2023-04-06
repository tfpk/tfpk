use yew::prelude::*;

use yew::classes;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct BlogSummaryProps {
    pub title: String,
    pub date: String,
    pub image: String,
    pub short_text: String,
}

#[function_component(BlogSummary)]
pub fn blog_summary(props: &BlogSummaryProps) -> Html {
    let background_image = format!("background-image: url({})", props.image);
    html! {
        <div class={classes!("row" ,"mb-4")}>
            <div class={classes!("col-12", "flex", "justify-center")}>
            <div class="bg-secondary-with-hover w-full lg:max-w-full lg:flex shadow-xl ring-1 ring-gray-900/5 dark:ring-gray-50/10">
                <div class="h-48 lg:h-auto lg:w-48 flex-none bg-cover  rounded-t lg:rounded-t-none lg:rounded-l text-center overflow-hidden" style={background_image} title="Mountain">
                </div>
                <div class="w-full border-r border-b border-l lg:border-l-0 lg:border-t lg:border-gray-50/10 rounded-b lg:rounded-b-none lg:rounded-r p-4 flex flex-col justify-between leading-normal">
                    <div class="mb-8">
                    <div class="text-title font-bold text-xl mb-2">{ props.title.clone() }</div>
                    <p class="text-primary text-base">{ props.short_text.clone() } </p>
                    </div>
                    <div class="flex items-center">
                    <img class="w-10 h-10 rounded-full mr-4" src="./static/icon_tfpk.jpg" alt="Tom" />
                    <div class="text-sm">
                        <p class="text-primary leading-none">{"Tom Kunc"}</p>
                        <p class="text-secondary">{ props.date.clone()}</p>
                    </div>
                    </div>
                </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(BlogListPage)]
pub fn blog_list_page() -> Html {
    let blog_list = crate::blogs::use_blog_list();
    html! {
        <main>
            <div class={classes!("container", "my-4")}>
                <div class={classes!("row")}>
                    <div class={classes!("col-12", "flex", "my-4", "justify-center")}>
                        <h1 class={classes!("m-auto", "text-3xl", "font-semibold", "text-gray-800", "dark:text-gray-200", "md:text-4xl")}>{ "Tom's Blog" }</h1>
                    </div>
                </div>
                <div class={classes!("row")}>
                    <div class={classes!("col-12", "flex", "my-4", "justify-center")}>
                        <p class={classes!("text-primary", "font-semibold")}>{ "This blog is provided AS-IS and WITHOUT WARRANTY, to the maximum extent permissible by law. Batteries sold seperately." }</p>
                    </div>
                </div>
        {blog_list.iter().map(|b| html!{
            <>
                <Link<Route> to={Route::Blog { slug: b.slug.clone() }}>
                    <BlogSummary title={b.title.clone()} date={format!("{}", b.last_updated)} image={b.img_path.clone()} short_text={b.lede.clone()}/>
                </Link<Route>>
            </>
        }).collect::<Html>()}
            </div>
        </main>
    }
}
