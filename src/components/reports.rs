use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use graphql_client::Response as GraphqlResponse;

use crate::api::get_route_full_path;
use crate::container::home::Route;
use crate::context::pin_projects::PinProjectListContext;
use crate::context::project_context::Project;
use crate::context::project_list_context::ProjectListContext;
use crate::{
    api::gitlab::vulnerability_severities_count,
    context::api_context::ApiContext
};

use crate::molecules::{report_table::ReportTable, BugSeverityIndicator};

#[derive(Debug, Default)]
struct VulCount {
    critical: Option<i64>,
    high: Option<i64>,
    info: Option<i64>,
    low: Option<i64>,
    medium: Option<i64>,
    unknown: Option<i64>,
}

#[function_component]
pub fn Reports() -> Html {
    let vul_count  = use_state(|| VulCount::default());

    let api = use_context::<ApiContext>().expect("no ctx found");
    
    let project_list_ctx = use_context::<ProjectListContext>().unwrap();
    
    let route = use_route::<Route>().expect("Failed to get route");
    let route_full_path = get_route_full_path(route);
    let pin_project_ctx = use_context::<PinProjectListContext>().unwrap();

    let get_project = project_list_ctx.data
        .iter()
        .find(|p| p.full_path == route_full_path)
        .unwrap_or({
            let pin_project =  pin_project_ctx.data
                .iter()
                .find(|p| p.full_path == route_full_path)
                .unwrap_or(&&crate::context::pin_projects::PinProject::default())
                .clone();

            &Project{
                full_path: pin_project.full_path,
                id: pin_project.id,
                name: pin_project.name,
                avatar_url: None
            }
        })
        .clone();
    
    {

        log!("Fetching Vuln..");
        let client = api.client.to_owned();
        let base_url = api.client.base_url.to_owned();
        let vul_count_clone = vul_count.clone();
        let full_path = get_project.full_path.clone();
        
        use_effect_with((base_url.clone(), full_path.clone()), move |_| {
            vul_count_clone.set(VulCount::default());
            wasm_bindgen_futures::spawn_local(async move {
                if !base_url.is_empty() {
                    log!("Fetching Vuln..2");
                    let resp = client.get_vul_count(full_path).await;
                    let resp_body: GraphqlResponse<vulnerability_severities_count::ResponseData> =
                        resp.json().await.unwrap();

                    let count_data= resp_body
                    .data
                    .unwrap()
                    .project
                    .unwrap()
                    .vulnerability_severities_count
                    .unwrap();

                    let vul_count_data = VulCount {
                        critical: count_data.vulnerability_severities_count.critical,
                        high: count_data.vulnerability_severities_count.high,
                        info: count_data.vulnerability_severities_count.info,
                        low: count_data.vulnerability_severities_count.low,
                        medium: count_data.vulnerability_severities_count.medium,
                        unknown: count_data.vulnerability_severities_count.unknown,
                    };
                    vul_count_clone.set(vul_count_data);

                }
            });
            || ()
        });
    }

    html! {
        <div class="flex flex-col bg-gradient-to-b from-neutral-900 to-neutral-950 items-start text-neutral-400 px-5 py-2 bg-neutral-900 rounded-md w-full h-full overflow-hidden">
            <div class="flex mb-5 items-center w-full border-b border-neutral-800">
                // <span class="mx-3 text-lg font-bold text-neutral-300 font-bold">{">>"}</span>

                {
                    get_project.full_path.clone().split("/").into_iter().map( |name| {
                        html!{
                            <>
                                <span class="mx-1 text-lg">{"/"}</span>
                                <span class={
                                    if get_project.name.clone() == name {
                                        "text-neutral-300 font-bold"
                                    } else {
                                        ""
                                    }
                                }>{name}</span>
                            </>
                        }
                    }).collect::<Html>()
                }
            </div>
            <h1 class="font-extrabold text-2xl text-neutral-300 underline">{ 
                if get_project.name.is_empty() {
                    String::from("Vulnerability report")
                }else {
                    format!("{} for {}", "Vulnerability report", get_project.name.clone()) 
                }
            }</h1>
            <span class="mt-1 mb-5">{"The Vulnerability Report shows results of successful scans on your project's default branch, manually added vulnerability records, and vulnerabilities found from scanning operational environments. Learn more."}</span>
            <div class="flex gap-5 mb-4">
                <BugSeverityIndicator
                    level={"Critical".to_string()}
                    number={vul_count.critical.unwrap_or(0) as usize}
                />
                <BugSeverityIndicator
                    level={"High".to_string()}
                    number={vul_count.high.unwrap_or(0) as usize}
                />
                <BugSeverityIndicator
                    level={"Medium".to_string()}
                    number={vul_count.medium.unwrap_or(0) as usize}
                />
                <BugSeverityIndicator
                    level={"Low".to_string()}
                    number={vul_count.low.unwrap_or(0) as usize}
                />
                <BugSeverityIndicator
                    level={"Info".to_string()}
                    number={vul_count.info.unwrap_or(0) as usize}
                />
                <BugSeverityIndicator
                    level={"Unknown".to_string()}
                    number={vul_count.unknown.unwrap_or(0) as usize}
                />
            </div>
            <div class="max-w-full overflow-x-auto relative mb-15 w-full h-full custom-scrollbar">
                    <ReportTable/>
            </div>
        </div>
    }
}
