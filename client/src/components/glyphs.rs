use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GlyphProps {
    pub glyph: GlyphType,
    pub color: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum GlyphType {
    Github,
    Twitter,
    Linkedin,
    Mail,
    Snackbar,
    Brightness,
    Blog,
    Experience,
    Resume,
}

#[function_component(Glyph)]
pub fn glyph(props: &GlyphProps) -> Html {
    let color = match &props.color {
        Some(color) => color.clone(),
        None => "fg-accent".to_string(),
    };
    match props.glyph {
        GlyphType::Github => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22" />
            </svg>
        },
        GlyphType::Linkedin => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z" />  <rect x="2" y="9" width="4" height="12" />  <circle cx="4" cy="4" r="2" />
            </svg>
        },
        GlyphType::Twitter => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                <path d="M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z" />
            </svg>
        },
        GlyphType::Mail => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z"/>  <rect x="3" y="5" width="18" height="14" rx="2" />  <polyline points="3 7 12 13 21 7" />
            </svg>
        },
        GlyphType::Snackbar => html! {
            <svg class={classes!("h-6", "w-6", color)} viewBox="0 0 24 24">
                <path class={classes!("hidden")} d="M16.24 14.83a1 1 0 0 1-1.41 1.41L12 13.41l-2.83 2.83a1 1 0 0 1-1.41-1.41L10.59 12 7.76 9.17a1 1 0 0 1 1.41-1.41L12 10.59l2.83-2.83a1 1 0 0 1 1.41 1.41L13.41 12l2.83 2.83z"/>
                <path d="M4 5h16a1 1 0 0 1 0 2H4a1 1 0 1 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2zm0 6h16a1 1 0 0 1 0 2H4a1 1 0 0 1 0-2z"/>
            </svg>
        },
        GlyphType::Brightness => html! {
            <svg class={classes!("h-6", "w-6", color)}  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round">  <circle cx="12" cy="12" r="5" />  <line x1="12" y1="1" x2="12" y2="3" />  <line x1="12" y1="21" x2="12" y2="23" />  <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />  <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />  <line x1="1" y1="12" x2="3" y2="12" />  <line x1="21" y1="12" x2="23" y2="12" />  <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />  <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" /></svg>
        },
        GlyphType::Blog => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                 <path stroke="none" d="M0 0h24v24H0z"/>  <path d="M6 4h11a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-11a1 1 0 0 1 -1 -1v-14a1 1 0 0 1 1 -1m3 0v18" />  <line x1="13" y1="8" x2="15" y2="8" />  <line x1="13" y1="12" x2="15" y2="12" />
            </svg>
        },
        GlyphType::Experience => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                <path stroke="none" d="M0 0h24v24H0z"/>  <path d="M12 17.75l-6.172 3.245 1.179-6.873-4.993-4.867 6.9-1.002L12 2l3.086 6.253 6.9 1.002-4.993 4.867 1.179 6.873z" />
            </svg>
        },
        GlyphType::Resume => html! {
            <svg
                class={classes!("pt-2", "h-10", "w-8", color)}
                viewBox="0 0 24 32"  fill="none"  stroke="currentColor"
                stroke-width="1.5"  stroke-linecap="round"  stroke-linejoin="round">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
        },
    }
}
