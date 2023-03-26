use crate::glyphs::{Glyph, GlyphType};
use yew::prelude::*;

use yew::classes;

use crate::home::{ContactSection, IntroSection, BlogsSection, ResumeSection};

fn toggle_local_storage() -> Option<()> {
    let key = "theme";
    let click_default = "dark";
    let window = web_sys::window()?;
    let local_storage = window.local_storage().ok()??;
    let (old_value, new_value) = match local_storage.get(key) {
        Ok(Some(val)) if val == "light" => ("light", "dark"),
        Ok(Some(val)) if val == "dark" => ("dark", "light"),
        _ => ("", click_default),
    };
    local_storage.set_item(key, new_value);

    // set document element's 'class' to 'dark'
    window.document()?.document_element()?.set_class_name(new_value);

    Some(())
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <header>
            <div class={classes!("w-full")}>
                <nav class={classes!("bg-accent", "shadow-lg")}>
                    <div class={classes!("md:flex", "items-center", "justify-between", "py-2", "px-8", "md:px-12")}>
                        <div class={classes!("flex", "justify-between", "items-center")}>
                            <div class={classes!("font-bold", "text-white", "md:text-3xl", "flex", "")}>
                               <img class={classes!("relative", "mr-4", "inline-block", "h-9", "w-9", "rounded-md", "object-cover", "object-center")} alt="Icon" src="./icon_tfpk.jpg"/>
                                <a href="#" class={classes!("text-2xl", "text-white", "dark:text-black")}>{"TFPK"}</a>
                            </div>
                        </div>
                        <div class={classes!("flex", "flex-col", "md:flex-row", "xs:hidden", "md:block-mx-2")}>
                            <div class={classes!("md:hidden")}>
                                <button type="button" class={classes!("block", "text-white", "hover:text-gray-500", "focus:text-gray-500", "focus:outline-none")}>
                                    <Glyph glyph={GlyphType::Snackbar} />
                                </button>
                            </div>
                            <a href="#" class={classes!("text-white", "dark:text-black", "rounded", "hover:bg-gray-500", "hover:text-gray-100", "hover:font-medium", "py-2", "px-2", "md:mx-2")}>{"Home"}</a>
                            <a href="#" class={classes!("text-white", "dark:text-black", "rounded", "hover:bg-gray-500", "hover:text-gray-100", "hover:font-medium", "py-2", "px-2", "md:mx-2")}>{"Projects"}</a>
                            <a href="#" class={classes!("text-white", "dark:text-black", "rounded", "hover:bg-gray-500", "hover:text-gray-100", "hover:font-medium", "py-2", "px-2", "md:mx-2")}>{"Blogs"}</a>
                        </div>
                        <div class={classes!("flex", "justify-between", "items-center")}>
                            <div class={classes!("font-bold", "text-gray-800", "md:text-3xl", "flex", "")}>
                                <a href="https://github.com/tfpk/"><Glyph glyph={GlyphType::Github} color="text-white dark:text-black" /></a>
                                <a href="https://linkedin.com/in/tfpk/"><Glyph glyph={GlyphType::Linkedin} color="text-white dark:text-black" /></a>
                                <a href="https://twitter.com/tfpk_/"><Glyph glyph={GlyphType::Twitter} color="text-white dark:text-black" /></a>
                                <button class={classes!("bg-transparent", "border-transparent")} onclick={|_| {toggle_local_storage();}}><Glyph glyph={GlyphType::Brightness} color="text-white dark:text-black" /></button>
                            </div>
                        </div>
                    </div>
                </nav>
            </div>
        </header>
    }
}
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Header />
            <IntroSection />
            <ContactSection />
            <BlogsSection />
            <ResumeSection />
        </main>
    }
}
