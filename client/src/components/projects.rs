use yew::prelude::*;

use yew::classes;
use yew_router::prelude::*;

use crate::Route;
use crate::components::glyphs::{Glyph, GlyphType};

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectSummaryProps {
    title: String,
    upper_time: String,
    lower_time: String,
    summary: String,
    link: Option<String>,
}

#[function_component(ProjectSummary)]
pub fn project_summary(props: &ProjectSummaryProps) -> Html {
    html! {
        <>
            <div class="flex w-full">
                <div class="flex flex-col text-md text-center m-2 w-16">
                    <span>{props.upper_time.clone()}</span>
                    <span>{"-"}</span>
                    <span>{props.lower_time.clone()}</span>
                </div>
                <div class="flex-grow-1 m-2">
                    <h2 class="text-lg font-extrabold">{props.title.clone()}</h2>
                    <p>{props.summary.clone()}</p>
                </div>
                <div class="flex flex-col justify-center text-lg text-center w-8">
                    <a href={props.link.clone()}>
                        <Glyph glyph={GlyphType::Github} />
                    </a>
                </div>
            </div>
            <hr class="m-2 mx-12"/>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectGroupProps {
    pub title: String,
    pub short_text: String,
    pub image: String,
    pub projects: Vec<ProjectSummaryProps>,
}

#[function_component(ProjectGroup)]
pub fn project_group(props: &ProjectGroupProps) -> Html {
    html! {
        <div class={classes!("mb-4", "h-full")}>
            <div class={classes!("project-group", "col-12", "flex", "justify-center")}>
            <div class="bg-secondary w-full lg:max-w-full lg:flex shadow-xl ring-1 ring-gray-900/5 dark:ring-gray-50/10">
                <div class="project-img hidden lg:block lg:h-auto bg-cover flex overflow-hidden" style="background-image: url('./static/project_featured.jpg'); background-size: 100% 100%;">
                    <img class="project-img d-none lg:h-auto bg-cover flex-none text-center overflow-hidden" src={props.image.clone()} title="Mountain" style="backdrop-filter: blur(20px); object-fit: scale-down; height: 100%"/>
                </div>
                <div class="w-full border-r border-b border-l lg:border-l-0 lg:border-t lg:border-gray-50/10 rounded-b lg:rounded-b-none lg:rounded-r p-4 flex flex-col justify-between leading-normal">
                    <div class="mb-8">
                        <div class="text-title font-bold text-xl mb-2">{ props.title.clone() }</div>
                        <p class="text-primary text-base">{ props.short_text.clone() } </p>
                        <hr class="m-4 mt-8"/>
                        { props.projects.iter().map(|project| {
                            html! {
                                <ProjectSummary
                                    title={project.title.clone()}
                                    upper_time={project.upper_time.clone()}
                                    lower_time={project.lower_time.clone()}
                                    summary={project.summary.clone()}
                                    link={project.link.clone()} />
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    let project_list = crate::projects::use_project_list().iter().map(|project| {
        let projects = project.projects.iter().map(|project| {
            ProjectSummaryProps {
                title: project.title.clone(),
                upper_time: project.upper_time.clone(),
                lower_time: project.lower_time.clone(),
                summary: project.summary.clone(),
                link: project.link.clone(),
            }
        }).collect::<Vec<ProjectSummaryProps>>();
        html! {
            <ProjectGroup
                title={project.title.clone()}
                short_text={project.short_text.clone()}
                image={project.img_path.clone()}
                projects={projects} />
        }
    }).collect::<Html>();

    html! {
        <main class="my-4 container flex flex-col justify-stretch">
            {project_list}
        </main>
    }
}
