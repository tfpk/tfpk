use yew::prelude::*;

use yew::classes;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectsProps {
    pub title: String,
    pub short_text: String,
    pub image: String,
}

#[function_component(ProjectGroup)]
pub fn project_group(props: &ProjectsProps) -> Html {
    html! {
        <div class={classes!("container" ,"mb-4")}>
            <div class={classes!("project-group", "col-12", "flex", "justify-center")}>
            <div class="bg-secondary w-full lg:max-w-full lg:flex shadow-xl ring-1 ring-gray-900/5 dark:ring-gray-50/10">
                <div class="project-img hidden lg:block lg:h-auto bg-cover flex flex-none overflow-hidden" style="background-image: url('./static/project_featured.jpg'); background-size: cover;">
                    <img class="project-img d-none lg:h-auto bg-cover flex-none rounded-t lg:rounded-t-none lg:rounded-l text-center overflow-hidden" src={props.image.clone()} title="Mountain" style="backdrop-filter: blur(20px); object-fit: scale-down; "/>
                </div>
                <div class="w-full border-r border-b border-l lg:border-l-0 lg:border-t lg:border-gray-50/10 rounded-b lg:rounded-b-none lg:rounded-r p-4 flex flex-col justify-between leading-normal">
                    <div class="mb-8">
                        <div class="text-title font-bold text-xl mb-2">{ props.title.clone() }</div>
                        <p class="text-primary text-base">{ props.short_text.clone() } </p>
                    </div>
                </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! {
        <main>
            <ProjectGroup title="Featured Projects" short_text="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl nec lacinia ultricies, nunc nisl tincidunt nisl, eget aliquam nisl justo eget nisl. Donec euismod, nisl nec lacinia ultricies, nunc nisl tincidunt nisl, eget aliquam nisl justo eget nisl." image="/static/project_featured.jpg" />
            <ProjectGroup title="Casual Academic @ UNSW" short_text="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl nec lacinia ultricies, nunc nisl tincidunt nisl, eget aliquam nisl justo eget nisl. Donec euismod, nisl nec lacinia ultricies, nunc nisl tincidunt nisl, eget aliquam nisl justo eget nisl." image="/static/project_UNSW_tutor.jpg" />
            <ProjectGroup title="Society Exec @ CSESoc" short_text="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl nec lacinia ultricies, nunc nisl tincidunt nisl, eget aliquam nisl justo eget nisl. Donec euismod, nisl nec lacinia ultricies, nunc nisl tincidunt nisl, eget aliquam nisl justo eget nisl." image="/static/project_UNSW_student.jpg" />
        </main>
    }
}
