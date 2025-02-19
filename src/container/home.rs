use gloo_console::log;
use graphql_client::Response as GraphqlResponse;
use js_sys::Array;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use super::invoke_without_args;

use crate::api::gitlab::get_projects;
use crate::api::ApiClient;
use crate::atoms::Loader;
use crate::components::code::Editor;
use crate::components::detail::Detail;
use crate::components::home::HomePage;
use crate::components::navbar::Navbar;
use crate::components::projects::Projects;
use crate::components::reports::Reports;
use crate::context::api_context::ApiContext;
use crate::context::pin_projects::{PinProject, PinProjectListContext};
use crate::context::project_context::Project;
use crate::context::project_list_context::ProjectListContext;
use crate::molecules::top_sidebar::TopSidebar;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/reports/:id")]
    Reports { id: String },
    #[at("/:project_path/-/security/vulnerabilities/:vul_id")]
    Detail {
        project_path: String,
        vul_id: String,
    },
    #[at(
        "/:project_path/-/blob/:with_ref/-/file/:path/-/start/:start_line/end/:end_line/-/:title"
    )]
    Editor {
        project_path: String,
        with_ref: String,
        path: String,
        start_line: String,
        end_line: String,
        title: String,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/>},
        Route::Reports { id: _ } => html! {
            <Reports/>
        },
        Route::Detail {
            project_path: _,
            vul_id: _,
        } => html! {
            <Detail />
        },
        Route::Editor {
            project_path: _,
            with_ref: _,
            path: _,
            start_line: _,
            end_line: _,
            title: _,
        } => html! {
            <Editor />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component]
pub fn Home() -> Html {
    let api_ctx = use_context::<ApiContext>().unwrap();
    let pin_project_ctx = use_context::<PinProjectListContext>().unwrap();

    let search = use_state(|| String::new());

    {
        let pin_project_ctx = pin_project_ctx.clone();
        use_effect(|| {
            spawn_local(async move {
                let get_pin_project_resp = &invoke_without_args("get_repos").await;

                match serde_wasm_bindgen::from_value::<Vec<PinProject>>(
                    get_pin_project_resp.clone(),
                ) {
                    Ok(data) => {
                        pin_project_ctx.dispatch(data);
                    }
                    Err(_) => {}
                }
            });
            || {}
        });
    }

    {
        let api_ctx_clone = api_ctx.clone();

        use_effect(|| {
            spawn_local(async move {
                // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
                if api_ctx_clone.client.base_url.is_empty() {

                    let resp = Array::from(&invoke_without_args("get_env").await);

                    let base_url = resp.get(0).as_string().unwrap_or_else(|| String::new());
                    let ci_job_token = resp.get(1).as_string().unwrap_or_else(|| String::new());
                    let group = resp.get(2).as_string().unwrap_or_else(|| String::new());

                    api_ctx_clone.dispatch(ApiClient::new(base_url, ci_job_token, group));
                }
            });
            || {}
        });
    }

    let loader = use_state(|| false);

    {
        let loader_clone = loader.clone();
        let project_list_ctx = use_context::<ProjectListContext>().unwrap();
        log!("Projects");
        let client = api_ctx.client.to_owned();
        let base_url = api_ctx.client.base_url.clone();
        use_effect_with(base_url.clone(), move |_| {
            if !base_url.is_empty() {
                // loader_clone.set(true);
                // let client: ApiClient = client.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // loader_clone.set(false);
                    // return ();
                    let mut all_projects: Vec<Project> = vec![];
                    let mut cursor: Option<String> = None;
                    loop {
                        let resp = client.clone().get_projects(cursor.clone()).await;
                        let resp_body: GraphqlResponse<get_projects::ResponseData> =
                            resp.json().await.unwrap();
                        if let Some(data) = resp_body.data {
                            let data = data.projects.unwrap();
                            let project_data: Vec<Project> = data
                                .nodes
                                .unwrap()
                                .into_iter()
                                .map(|projects| {
                                    if let Some(project) = projects {
                                        return Project {
                                            id: project.id,
                                            name: project.name,
                                            full_path: project.full_path,
                                            avatar_url: project.avatar_url,
                                        };
                                    } else {
                                        Project::default()
                                    }
                                })
                                .collect();

                            all_projects.extend(project_data);

                            if project_list_ctx.data.len() == 0 || *loader_clone {
                                project_list_ctx.dispatch(all_projects.clone());
                                loader_clone.set(false);
                            }

                            let page_info = data.page_info;

                            if !page_info.has_next_page {
                                break;
                            }

                            cursor = page_info.end_cursor;
                        }
                    }
                    project_list_ctx.dispatch(all_projects.clone());
                    loader_clone.set(false);
                    // items_clone.set(all_projects);
                });
                // loader_clone.set(false);
            }
            || ()
        });
    }

    let handle_input = {
        let input_value = search.clone();
        Callback::from(move |value: String| {
            input_value.set(value);
        })
    };

    html! {
        <BrowserRouter>
            <div class="relative px-4 flex flex-col bg-neutral-950 h-screen w-screen overflow-hidden">
                <Navbar/>

                <div class="flex w-full gap-2 h-full">
                    // Sidebar
                    <div class="p-2 bg-neutral-900 rounded-md w-90 bg-gradient-to-b from-neutral-900 to-neutral-950">
                            <TopSidebar oninput={handle_input} update_search={search.clone()} />
                            <div class="overflow-y-auto h-full w-full !overflow-x-hiddden pb-60 custom-scrollbar">
                                {if *loader {
                                    html!{
                                        <div class="flex w-full items-center justify-center">
                                            <Loader class="size-3 mt-3 flex justify-center" />
                                        </div>
                                    }
                                } else {
                                    html!{
                                    <Projects filter={(*search).clone()} icon_class="!min-w-12 !min-h-12" update_search={search} />
                                    }
                                }}
                            </div>
                    </div>
                    
                    // main page
                    <Switch<Route> render={switch} />
                </div>
            </div>
        </BrowserRouter>
    }
}
