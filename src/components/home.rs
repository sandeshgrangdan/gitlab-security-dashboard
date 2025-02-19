use chrono::{Datelike, Local, NaiveDate};
use graphql_client::Response as GraphqlResponse;
use gloo_console::log;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yew_router::prelude::*;

use crate::{
    api::{
        gitlab::group_vulnerability_history, ApiClient
    }, 
    components::detail::AddRepoArgs, 
    container::
    {
        home::Route, 
        invoke, 
        invoke_without_args
    }, 
    context::{
        api_context::ApiContext, 
        pin_projects::{PinProject, PinProjectListContext}, 
        project_context::Project
    }
};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct VulHistory {
    pub date: String,
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}
#[wasm_bindgen(module = "/src/js/charts.js")]
unsafe extern "C" {
    fn initCharts(
        id: &str, 
        x_values : Vec<String>,
        critical : Vec<usize>,
        high : Vec<usize>,
        medium : Vec<usize>,
        low : Vec<usize>,
        info : Vec<usize>,
        unknown : Vec<usize>,
    );
}

fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
    NaiveDate::from_ymd_opt(year, month, day).is_some()
}

fn subtract_one_month(date: NaiveDate) -> NaiveDate {
    let mut year = date.year();
    let mut month = date.month() - 1;
    let mut day = date.day();

    if month < 1 {
        month = 12;
        year -= 1;
    }

    // Handle cases where the day is not valid for the new month
    // For example, 31st January will be adjusted to 30th or 31st December
    while !is_valid_date(year, month, day) {
        day -= 1;
    }

    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}


#[function_component]
pub fn HomePage() -> Html {
    
    let pin_project_ctx = use_context::<PinProjectListContext>().unwrap();
    
    let navigator = use_navigator().unwrap();

    let onclick = {
        let nav = navigator.clone();
            Callback::from(move |set_project: Project| {
                nav.push(&Route::Reports{ id: set_project.full_path.clone() });
            })
    };

    let api_ctx = use_context::<ApiContext>().unwrap();

    {
        log!("History");
        let client = api_ctx.client.to_owned();
        let base_url = api_ctx.client.base_url.clone();
        let full_path = api_ctx.client.group.clone();
        let now = Local::now().date_naive();
        let end_date = now.to_string();
        let start_date = subtract_one_month(now).to_string();

        use_effect_with(base_url.clone(), move |_| {
            if !base_url.is_empty() {
                let client: ApiClient = client.clone();
                wasm_bindgen_futures::spawn_local(async move {

                    let resp = client.clone().get_vul_history(full_path,start_date,end_date).await;
                    let resp_body: GraphqlResponse<group_vulnerability_history::ResponseData> =
                    resp.json().await.unwrap();

                    let mut x_value: Vec<String> = vec![];

                    let mut critical: Vec<usize> = vec![];
                    let mut high: Vec<usize> = vec![];
                    let mut medium: Vec<usize> = vec![];
                    let mut low: Vec<usize> = vec![];
                    let mut info: Vec<usize> = vec![];
                    let mut unknown: Vec<usize> = vec![];

                    let _: Vec<VulHistory> = resp_body
                        .data.unwrap()
                        .group.unwrap()
                        .vulnerabilities_count_by_day.unwrap()
                        .nodes.unwrap()
                        .into_iter()
                        .map(|h|{
                            let his = h.unwrap();
                            log!(his.date.clone());
                            x_value.push(his.date);
                            critical.push(his.critical as usize);
                            high.push(his.high as usize);
                            medium.push(his.medium as usize);
                            low.push(his.low as usize);
                            info.push(his.info as usize);
                            unknown.push(his.unknown as usize);

                            VulHistory::default()
                        }).collect();

                        log!(x_value.clone());

                        initCharts(
                            "myChart",
                            x_value,
                            critical,
                            high,
                            medium,
                            low,
                            info,
                            unknown
                        );
                        
                });
                // loader_clone.set(false);
            }
            || ()
        });
    }

    html! {
        <div class="flex flex-col bg-neutral-900 w-full h-full rounded-lg relative overflow-hidden bg-gradient-to-b from-teal-950 to-neutral-900 overflow-hidden py-8">
            {
                html!{
                    <div class="justify-start flex flex px-4 rounded-lg py-2">
                        <img src="/public/gitlab_security.svg" class="h-fit" />
                        <div class="ml-4 mx-2 mb-4 flex flex-col flex-end">
                            <p class="text-6xl font-black text-gray-300">{ "Monitor vulnerabilities" }</p>
                            <p class="text-zinc-400">{ "Manage and track vulnerabilities identified in your project. Historical view of open vulnerabilities in the default branch. Excludes vulnerabilities that were resolved or dismissed." }</p>

                            <div class="flex gap-5 flex-wrap">
                                <a class="mt-4 px-3 py-1 rounded-lg hover:bg-sky-950 bg-sky-900 decoration-none !text-stone-400" target="_blank" href="https://docs.gitlab.com/ee/user/application_security/configuration/index.html">
                                    {"Configure security testing"}
                                </a>
                                <a class="mt-4 px-3 py-1 outline rounded-lg decoration-none !text-stone-400" target="_blank" href="https://docs.gitlab.com/ee/user/application_security/configuration/index.html">
                                    {"Learn More"}
                                </a>
                            </div>
                        </div>
                    </div>
                }
            }
            <div class="flex flex-col relative bg-neutral-900/20 h-full pt-3 gap-3 w-full">
                
                <div class="px-4">
                    <canvas id="myChart" class="w-full h-fit" height="250"></canvas>
                </div>

                <div class="pb-60 overflow-y-auto h-full">

                    <div onclick={Callback::from({
                        move |_| {}
                    })}  class={
                                "flex flex-row items-center px-4 py-2 rounded-md"
                        }>
                        <div class={
                            format!("size-12 flex items-center justify-center rounded-lg bg-gradient-to-br from-orange-600 to-gray-400 font-bold text-2xl {}","icon_class")
                        }>
                            <span>{ pin_project_ctx.data.len() }</span>
                        </div>

                        <div class="w-86 flex flex-col ml-3">
                            <div  class="text-md text-gray-200">{ "Your Projects" }</div>
                            <div class="flex items-center">
                                // <Icon
                                //     class="text-green-600 mr-1"
                                //     icon_id={IconId::BootstrapPinAngleFill}
                                //     height={"1em".to_owned()}
                                //     width={"1em".to_owned()}
                                // />
                                <div class="text-gray-400 line-clamp-1">{ format!("Project - {} Repo",pin_project_ctx.data.len()) }</div>
                            </div>
                        </div>
                    </div>

                    {
                        pin_project_ctx.data.clone().into_iter().map(|p| {
                            let full_path = p.full_path.to_owned();
                            let id = p.id.to_owned();
                            let name = p.name.to_owned();
                            html!{
                                <div onclick={Callback::from({
                                    let onclick = onclick.clone();
                                    let full_path = full_path.clone();
                                    let id = id.clone();
                                    let name = name.clone();
                                    
                                    let p_ctx = Project::new(full_path,id, name);
                                    move |_| onclick.emit(p_ctx.clone())
                                })} class={ "group cursor-pointer flex flex-row items-center hover:bg-cyan-900/40 px-4 py-2 rounded-md"
                                    }>
                                    <div class={
                                        format!("size-12 flex items-center justify-center rounded-lg bg-gradient-to-br from-pink-600 to-gray-400 font-bold text-xl {}","icon_class")
                                    }>
                                        <Icon
                                            icon_id={IconId::LucideGitlab}
                                            height={"1.5em".to_owned()}
                                            width={"1.5em".to_owned()}
                                        />
                                    </div>
                
                                    <div class="w-full justify-between flex flex-row ml-3">
                                        <div>
                                            <div  class="text-md text-gray-200">{ p.name }</div>
                                            <div class="flex items-center">
                                                // <Icon
                                                //     class="text-green-600 mr-1"
                                                //     icon_id={IconId::BootstrapPinAngleFill}
                                                //     height={"1em".to_owned()}
                                                //     width={"1em".to_owned()}
                                                // />
                                                <div class="text-gray-400 line-clamp-1">{ p.full_path }</div>
                                            </div>
                                        </div>
                                        <div class=""
                                            onclick={
                                                Callback::from({
                                                    let pin_project_ctx = pin_project_ctx.clone();
                                                    let full_path = full_path.clone();
                                                    let id = id.clone();
                                                    let name = name.clone();
                                                        
                                                    move |event: MouseEvent| {
                                                        event.stop_propagation();
                                                        let pin_project_ctx = pin_project_ctx.clone();
                                                        let full_path = full_path.clone();
                                                        let id = id.clone();
                                                        let name = name.clone();
                                                        
                                                        spawn_local(async move {
                                                            let args = serde_wasm_bindgen::to_value(&AddRepoArgs { id: &id , name: &name, path : &full_path }).unwrap();
                                            
                                                            invoke( "delete_repo", args).await;

                                                            let get_pin_project_resp = &invoke_without_args("get_repos").await;

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
                                                icon_id={
                                                        IconId::BootstrapDashCircleDotted
                                                }
                                                height={"1.3em".to_owned()}
                                                width={"1.3em".to_owned()}
                                            />
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            </div>

        </div>
    }
}
