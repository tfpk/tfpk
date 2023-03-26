use yew::prelude::*;

use yew::classes;
use crate::glyphs::{Glyph, GlyphType};

#[derive(Properties, Clone, Debug, PartialEq)]
struct SocialLinkProps {
    link: String,
    glyph: GlyphType,
    name: String,
    code: String,
}

fn toggle_local_storage() -> Option<()> {
    let key = "theme";
    let click_default = "dark";
    let local_storage = web_sys::window()?.local_storage().ok()??;
    let new_value = match local_storage.get(key) {
        Ok(Some(val)) if val == "light" => "dark",
        Ok(Some(val)) if val == "dark" => "light",
        _ => click_default,
    };
    local_storage.set_item(key, new_value).ok()?;
    Some(())
}

#[function_component(SocialLink)]
fn social_link(props: &SocialLinkProps) -> Html {
    html! {
        <div class="space-y-4">
            <a href={props.link.clone()} class="flex items-center px-6 py-2 shadow-xl ring-1 ring-gray-900/5 dark:ring-gray-100/5 sm:mx-auto sm:max-w-lg sm:rounded-lg sm:px-10 hover:bg-gray-200 dark:hover:bg-gray-900">
                <div class="flex-none">
                    <Glyph glyph={props.glyph.clone()} />
                </div>
                <p class="ml-4">
                    {props.name.clone()}{": "}
                    <code class="text-sm font-bold text-gray-900 dark:text-gray-50">{props.code.clone()}</code>
                </p>
            </a>
        </div>
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
struct HomeSectionProps {
    side: SectionSide,
    image: String,
    contents: Html
}


#[function_component(HomeSection)]
fn home_section(props: &HomeSectionProps) -> Html {
    let polygon_style = match props.side {
        SectionSide::Left => "clip-path: polygon(100% 0, 80% 50%, 100% 100%, 0 100%, 0 0);",
        SectionSide::Right => "clip-path: polygon(0 0, 100% 0, 100% 100%, 0 100%, 20% 50%);",
    };
    let image_style = format!("background-image: url(\"{}\"); background-size: cover;", props.image);

    let hero_image = html!{
        <div class={classes!("hidden", "lg:block", "lg:w-1/2")} style={polygon_style}>
            <div class={classes!("h-full", "object-cover")} style={image_style}>
                <div class={classes!("h-full", "bg-black", "opacity-25")}></div>
            </div>
        </div>
    };
    let text_content = html! {
        <div class={classes!("flex", "items-center", "text-center", "lg:text-left", "px-8", "md:px-12", "lg:w-1/2")}>
            {props.contents.clone()}
        </div>
    };
    let contents = match props.side {
        SectionSide::Left => html!{<>{hero_image}{text_content}</>},
        SectionSide::Right => html!{<>{text_content}{hero_image}</>},
    };

    html! {
        <div class={classes!("flex")} style="height:600px;">
            {contents}
        </div>
    }
}

#[function_component(IntroSection)]
pub fn intro() -> Html {
    let text_content = html!{
        <div>
            <h2 class={classes!("text-3xl", "font-semibold", "text-gray-800", "dark:text-gray-200", "md:text-4xl")}>{
                "Hi, I'm Tom Kunc"
            }</h2>
            <p class={classes!("mt-2", "text-sm", "text-gray-500", "md:text-base")}>{

                r#"I'm currently working as a Software Engineer for Optiver
                   Australia, after five years studying and teaching Computer
                   Science at the University of New South Wales. My experience
                   is mostly in designing tools and content for teaching, though I have web
                   design and data-wrangling experience too. "#

            }</p>
            <p class={classes!("mt-2", "text-sm", "text-gray-500", "md:text-base")}>{

                r#"In my spare time, I like hiking, skiing and photography; and
                   you can see the results of those passions in
                   some of the photos below! If you're wondering, the letters "tfpk" are
                   my initials, and they've become my online handle over the last few years."#

            }</p>
        </div>
    };
    html! {
        <HomeSection side={SectionSide::Right} image="./hero_profile.jpg" contents={text_content} />
    }
}

#[function_component(ContactSection)]
pub fn contact_section() -> Html {
    let text_content = html!{
        <div>
            <h2 class={classes!("text-3xl", "font-semibold", "text-gray-800", "dark:text-gray-200", "md:text-4xl")}>{"Contact Me"}</h2>
            <p class={classes!("my-4", "text-sm", "text-gray-500", "md:text-base")}>{
                "Please reach out!"
            }</p>
            <div class={classes!("space-y-4")}>
                <div class="relative">
                    <div class="mx-auto max-w-md">
                        <div class="divide-y divide-gray-300/50">
                            <div class="space-y-6 py-8 text-base leading-7 text-gray-500">
                                <SocialLink link="https://twitter.com/tfpk" glyph={GlyphType::Twitter} name="Twitter" code="@tfpk"/>
                                <SocialLink link="https://linkedin.com/in/tfpk" glyph={GlyphType::Linkedin} name="Linkedin" code="/in/tfpk"/>
                                <SocialLink link="https://github.com/tfpk" glyph={GlyphType::Github} name="Github" code="@tfpk"/>
                                <SocialLink link="#" glyph={GlyphType::Mail} name="Email" code="tom <at> tfpk <dot> dev"/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    };
    html! {
        <HomeSection side={SectionSide::Left} image="./hero_zion.jpg" contents={text_content} />
    }
}

#[function_component(BlogsSection)]
pub fn blogs_section() -> Html {
    let text_content = html!{
        <div>
            <h2 class={classes!("text-3xl", "font-semibold", "text-gray-800", "dark:text-gray-200", "md:text-4xl")}>{"Recent Blogs"}</h2>
            <p class={classes!("mt-2", "text-sm", "text-gray-500", "md:text-base")}>{
                "You can read my recent blogs by pressing the link below (or see the most recent one!)"
            }</p>
            <div class={classes!("flex", "justify-center", "lg:justify-start", "mt-6")}>
            </div>
        </div>
    };
    html! {
        <HomeSection side={SectionSide::Right} image="./hero_bc.jpg" contents={text_content} />
    }
}

#[function_component(ResumeSection)]
pub fn resume_section() -> Html {
    let text_content = html!{
        <div>
            <h2 class={classes!("text-3xl", "font-semibold", "text-gray-800", "dark:text-gray-200", "md:text-4xl")}>{"Resume & Experience"}</h2>
            <p class={classes!("mt-2", "text-sm", "text-gray-500", "md:text-base")}>{
                "If you'd like to read more about my experience, or download my resume, you can do so by pressing the links below."
            }</p>
            <div class={classes!("flex", "justify-center", "lg:justify-start", "mt-6")}>
            </div>
        </div>
    };
    html! {
        <HomeSection side={SectionSide::Left} image="./hero_aspen.jpg" contents={text_content} />
    }
}

#[derive(Clone, Debug, PartialEq)]
enum SectionSide {
    Left,
    Right,
}
