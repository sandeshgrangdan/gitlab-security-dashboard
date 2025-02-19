use gloo_console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{
    api::get_route_full_path,
    components::detail::AddRepoArgs,
    container::{home::Route, invoke, invoke_without_args},
    context::{
        pin_projects::{PinProject, PinProjectListContext},
        project_context::{Project, ProjectContext},
        project_list_context::ProjectListContext,
    },
};

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectProps {
    pub filter: String,
    pub icon_class: String,
    pub update_search: UseStateHandle<String>,
}

#[function_component]
pub fn Projects(props: &ProjectProps) -> Html {
    let ProjectProps {
        filter,
        icon_class,
        update_search: search,
    } = props;
    let other_update_search = search.clone();

    let project_list_context = use_context::<ProjectListContext>().expect("no ctx found");

    let filter_projects: UseStateHandle<Vec<Project>> = use_state(|| vec![]);
    let pin_filter_projects: UseStateHandle<Vec<PinProject>> = use_state(|| vec![]);

    let projects = project_list_context.data.clone();

    let pin_project_ctx = use_context::<PinProjectListContext>().unwrap();
    let pin_projects = pin_project_ctx.data.clone();

    {
        let filter_clone = filter.clone();
        let filter_projects = filter_projects.clone();
        let pin_filter_projects = pin_filter_projects.clone();

        use_effect_with((filter_clone.clone(), projects.len(), pin_projects.len()), move |_| {
            if projects.len() > 0 && !filter_clone.is_empty() {
                let filter_data: Vec<Project> = projects
                    .clone()
                    .into_iter()
                    .filter(|project| {
                        project
                            .name
                            .to_lowercase()
                            .contains(&filter_clone.to_lowercase())
                    })
                    .collect();
                filter_projects.set(filter_data);

                pin_filter_projects.set(
                    pin_projects
                    .clone()
                    .into_iter()
                    .filter(|project| {
                        project
                            .name
                            .to_lowercase()
                            .contains(&filter_clone.to_lowercase())
                    })
                    .collect()
                )

            } else {
                filter_projects.set(projects.clone());
                pin_filter_projects.set(pin_projects.clone());
            }
        });
    }

    let route = use_route::<Route>().expect("Failed to get route");

    let route_full_path = get_route_full_path(route);

    log!(format!("{} haha", route_full_path));

    let project_ctx = use_context::<ProjectContext>().unwrap();

    let navigator = use_navigator().unwrap();

    let onclick = {
        let nav = navigator.clone();
        Callback::from(move |set_project: Project| {
            other_update_search.set(String::new());
            nav.push(&Route::Reports {
                id: set_project.full_path.clone(),
            });
            project_ctx.dispatch(set_project);
        })
    };

    html! {
                <div class="cursor-pointer mt-1">
                        <div onclick={Callback::from({
                            let nav = navigator.clone();
                            move |_| {
                                nav.push(&Route::Home);
                            }
                        })}  class="flex flex-row items-center hover:bg-neutral-800 p-2 rounded-md">
                            <div class={
                                format!("size-12 flex items-center justify-center rounded-lg bg-gradient-to-br from-orange-600 to-gray-400 font-bold text-xl {}",icon_class)
                            }>
                                <Icon
                                    icon_id={IconId::LucideGitlab}
                                    height={"1.5em".to_owned()}
                                    width={"1.5em".to_owned()}
                                />
                            </div>

                            <div class="w-86 flex flex-col ml-3">
                                <div  class="text-md text-gray-200">{ "My Projects" }</div>
                                <div class="flex items-center">
                                    <Icon
                                        class="text-green-600 mr-1"
                                        icon_id={IconId::BootstrapPinAngleFill}
                                        height={"1em".to_owned()}
                                        width={"1em".to_owned()}
                                    />
                                    <div class="text-gray-400 line-clamp-1">{ format!("Project - {} Repo",pin_project_ctx.data.len()) }</div>
                                </div>
                            </div>
                        </div>
                        {
                            pin_filter_projects.iter().map(|project| {
                                let onclick = onclick.clone();

                                let full_path = project.full_path.to_owned();
                                let id = project.id.to_owned();
                                let name = project.name.to_owned();

                                let is_selected = route_full_path == full_path;

                                html!{
                                    <div onclick={Callback::from({
                                        let onclick = onclick.clone();
                                        let p_ctx = Project::new(full_path.clone(),id.clone(), name.clone());
                                        move |event: MouseEvent| {
                                            event.prevent_default();
                                            onclick.emit(p_ctx.clone())
                                        }
                                    })}  class={
                                            if is_selected {
                                                "group flex bg-sky-950 hover:bg-sky-900 flex-row items-center p-2 rounded-md relative"
                                            } else {
                                                "group flex flex-row items-center hover:bg-sky-900 p-2 rounded-md relative"
                                            }
                                        }>
                                        <div class="absolute top-2 right-2 opacity-0 group-hover:!opacity-100 transition-opacity duration-200"
                                            onclick={
                                                Callback::from({
                                                    let pin_project_ctx = pin_project_ctx.clone();
                                                    let other_update_search = search.clone();

                                                    move |event: MouseEvent| {
                                                        event.stop_propagation();
                                                        let pin_project_ctx = pin_project_ctx.clone();
                                                        let full_path = full_path.clone();
                                                        let id = id.clone();
                                                        let name = name.clone();
                                                        let other_update_search = other_update_search.clone();

                                                        spawn_local(async move {
                                                            let args = serde_wasm_bindgen::to_value(&AddRepoArgs { id: &id , name: &name, path : &full_path }).unwrap();

                                                            invoke("delete_repo", args).await;

                                                            let get_pin_project_resp = &invoke_without_args("get_repos").await;

                                                            other_update_search.set(String::new());

                                                            match serde_wasm_bindgen::from_value::<Vec<PinProject>>(get_pin_project_resp.clone()){
                                                                Ok(data) => {
                                                                    pin_project_ctx.dispatch(data);
                                                                },
                                                                Err(_) => {},
                                                            }
                                                        });
                                                    }
                                                })
                                            }
                                        >
                                            <Icon
                                                class="text-gray-300 mr-1 hover:scale-105 transition-transform duration-300 bg-sky-900"
                                                icon_id={
                                                        IconId::BootstrapDashCircleDotted
                                                }
                                                height={"1.3em".to_owned()}
                                                width={"1.3em".to_owned()}
                                            />
                                        </div>
                                        <div class="absolute top-2 right-2 opacity-100 group-hover:!hidden transition-opacity duration-300">
                                            <Icon
                                                class="text-orange-500 mr-1 hover:scale-105 transition-transform duration-100"
                                                icon_id={
                                                        IconId::OcticonsBookmark24
                                                }
                                                height={"1em".to_owned()}
                                                width={"1em".to_owned()}
                                            />
                                        </div>

                                        <div class={
                                            format!("size-12 flex items-center justify-center bg-cyan-700 rounded-full font-bold text-xl {}",icon_class)
                                        }>{ project.name.chars().next().map(|c| c.to_uppercase().to_string()) }</div>

                                        <div class="w-full flex flex-col ml-3 overflow-hidden">
                                            <span  class="text-md text-gray-200 line-clamp-1 pr-4" key={project.id.clone()}>{ project.name.clone() }</span>
                                            <span class="text-gray-400 line-clamp-1" key={project.full_path.clone()}>{ project.full_path.clone() }</span>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    {
                        filter_projects.iter().map(|project| {
                            let onclick = onclick.clone();

                            let full_path = project.full_path.to_owned();
                            let id = project.id.to_owned();
                            let name = project.name.to_owned();

                            let is_pinned = pin_project_ctx.data
                                .iter()
                                .any(|p| p.full_path == full_path);

                            let is_selected = route_full_path == full_path;


                            html!{
                                <div onclick={Callback::from({
                                    let onclick = onclick.clone();
                                    let p_ctx = Project::new(full_path.clone(),id.clone(), name.clone());
                                    move |event: MouseEvent| {
                                        event.prevent_default();
                                        onclick.emit(p_ctx.clone())
                                    }
                                })}  class={
                                        if is_pinned {
                                            "hidden"
                                        } else {
                                            if is_selected {
                                                "group flex bg-sky-950 hover:bg-sky-900 flex-row items-center p-2 rounded-md relative"
                                            } else {
                                                "group flex flex-row items-center hover:bg-sky-900 p-2 rounded-md relative"
                                            }
                                        }
                                    }>
                                    <div class="absolute top-2 right-2 opacity-0 group-hover:!opacity-100 transition-opacity duration-200 bg-sky-900"
                                        onclick={
                                            Callback::from({
                                                let pin_project_ctx = pin_project_ctx.clone();
                                                let other_update_search = search.clone();

                                                move |event: MouseEvent| {
                                                    event.stop_propagation();
                                                    let other_update_search = other_update_search.clone();
                                                    let pin_project_ctx = pin_project_ctx.clone();
                                                    let full_path = full_path.clone();
                                                    let id = id.clone();
                                                    let name = name.clone();

                                                    spawn_local(async move {
                                                        let args = serde_wasm_bindgen::to_value(&AddRepoArgs { id: &id , name: &name, path : &full_path }).unwrap();

                                                        invoke( "add_repo", args).await;

                                                        let get_pin_project_resp = &invoke_without_args("get_repos").await;

                                                        other_update_search.set(String::new());

                                                        match serde_wasm_bindgen::from_value::<Vec<PinProject>>(get_pin_project_resp.clone()){
                                                            Ok(data) => {
                                                                pin_project_ctx.dispatch(data);
                                                            },
                                                            Err(_) => {},
                                                        }
                                                    });
                                                }
                                            })
                                        }
                                    >
                                        <Icon
                                            class="text-gray-300 mr-1 hover:scale-105 transition-transform duration-100"
                                            icon_id={IconId::BootstrapPlusCircleDotted}
                                            height={"1.3em".to_owned()}
                                            width={"1.3em".to_owned()}
                                        />
                                    </div>

                                    <div class={
                                        format!("size-12 flex items-center justify-center bg-cyan-700 rounded-full font-bold text-xl {}",icon_class)
                                    }>{ project.name.chars().next().map(|c| c.to_uppercase().to_string()) }</div>

                                    <div class="w-full flex flex-col ml-3 overflow-hidden">
                                        <span  class="text-md text-gray-200 line-clamp-1" key={project.id.clone()}>{ 
                                                project.name.clone() 
                                        }</span>
                                        <span class="text-gray-400 line-clamp-1" key={project.full_path.clone()}>{ project.full_path.clone() }</span>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
    }
}
