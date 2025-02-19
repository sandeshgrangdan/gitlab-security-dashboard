use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::*;
use graphql_client::Response as GraphqlResponse;
use yew_icons::{Icon, IconId};

use crate::{
    api::{
        get_route_full_path, 
        gitlab::get_vuls::{
            self, 
            GetVulsProjectVulnerabilitiesNodesIdentifiers, 
            VulnerabilitySeverity,
            GetVulsProjectVulnerabilitiesPageInfo
        }
    }, 
    atoms::Loader, 
    container::home::Route, 
    context::{
        api_context::ApiContext, 
        detail_context::{
            ProjectDetail, 
            ProjectDetailContext
        }, 
        project_context::Project, 
        project_list_context::ProjectListContext
    }
};

use crate::atoms::{BugSeverityLevel, CheckBox};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Location {
    pub blob_path: String,
    pub file: String,
    pub start_line: String,
    pub end_line: String,
}

impl Default for get_vuls::VulnerabilitySeverity {
    fn default() -> Self {
        get_vuls::VulnerabilitySeverity::INFO
    }
}

impl PartialEq for VulnerabilitySeverity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Other(l0), Self::Other(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Clone for VulnerabilitySeverity {
    fn clone(&self) -> Self {
        match self {
            Self::INFO => Self::INFO,
            Self::UNKNOWN => Self::UNKNOWN,
            Self::LOW => Self::LOW,
            Self::MEDIUM => Self::MEDIUM,
            Self::HIGH => Self::HIGH,
            Self::CRITICAL => Self::CRITICAL,
            Self::Other(arg0) => Self::Other(arg0.clone()),
        }
    }
}

impl PartialEq for GetVulsProjectVulnerabilitiesNodesIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.external_type == other.external_type && self.name == other.name
    }
}

impl Clone for GetVulsProjectVulnerabilitiesNodesIdentifiers {
    fn clone(&self) -> Self {
        Self { external_type: self.external_type.clone(), name: self.name.clone() }
    }
}

impl Default for GetVulsProjectVulnerabilitiesPageInfo {
    fn default() -> Self {
        Self { has_next_page: Default::default(), has_previous_page: Default::default(), start_cursor: Default::default(), end_cursor: Default::default() }
    }
}

// impl Clone for get_vuls::GetVulsProject {
//     fn clone(&self) -> Self {
//         Self { name_with_namespace: self.name_with_namespace.clone(), vulnerabilities: self.vulnerabilities.clone() }
//     }
// }

// impl Clone for get_vuls::GetVulsProjectVulnerabilities {
//     fn clone(&self) -> Self {
//         Self { nodes: self.nodes.clone(), page_info: self.page_info.clone() }
//     }
// }

// impl Clone for get_vuls::GetVulsProjectVulnerabilitiesNodes {
//     fn clone(&self) -> Self {
//         Self { id: self.id.clone(), title: self.title.clone(), description: self.description.clone(), severity: self.severity.clone(), state: self.state.clone(), detected_at: self.detected_at.clone(), report_type: self.report_type.clone(), scanner: self.scanner.clone(), identifiers: self.identifiers.clone(), location: self.location.clone() }
//     }
// }

// impl Clone for get_vuls::GetVulsProjectVulnerabilitiesPageInfo {
//     fn clone(&self) -> Self {
//         Self { has_next_page: self.has_next_page.clone(), has_previous_page: self.has_previous_page.clone(), start_cursor: self.start_cursor.clone(), end_cursor: self.end_cursor.clone() }
//     }
// }


#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vulnarable {
    pub id: String,
    pub detected: String,
    pub severity: get_vuls::VulnerabilitySeverity,
    pub title: String,
    pub description: String,
    pub location: Location,
    pub report_type: String,
    pub scanner_name: Option<String>,
    // pub scanner: get_vuls::GetVulProjectVulnerabilitiesNodesScanner,
    pub identifiers: Vec<GetVulsProjectVulnerabilitiesNodesIdentifiers>,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CurrentPagination {
    pub after: Option<String>,
    pub before: Option<String>,
    pub first: Option<i64>,
}


#[function_component]
pub fn ReportTable() -> Html {

    let reports: UseStateHandle<Vec<Vulnarable>>  = use_state(|| vec![]);
    let project_name_with_namespaces = use_state(|| String::new());

    let api = use_context::<ApiContext>().expect("no ctx found");

    let loading = use_state(|| true);
    let route = use_route::<Route>().expect("Failed to get route");
    
    let full_path = get_route_full_path(route);

    let page_info: UseStateHandle<GetVulsProjectVulnerabilitiesPageInfo> = use_state(|| GetVulsProjectVulnerabilitiesPageInfo::default());
    let cur_page: UseStateHandle<CurrentPagination> = use_state(|| CurrentPagination{
        after: None,
        before: None,
        first : Some(100)
    });

    {
        let cur_page = cur_page.clone();
        use_effect_with(full_path.clone(),move |_|{
            cur_page.set(CurrentPagination{
                after: None,
                before: None,
                first : Some(100)
            });
            || {}
        });
    }

    {
        log!("Fetching Vuln..");
        let client = api.client.to_owned();
        let base_url = api.client.base_url.to_owned();
        let reports_clone = reports.clone();
        let project_name_with_namespaces_clone = project_name_with_namespaces.clone();
        let full_path = full_path.clone();
        let loading_clone = loading.clone();
        let page_info =  page_info.clone();
        let cur_page = cur_page.clone();

        use_effect_with((base_url.clone(),full_path.clone(), cur_page.after.clone(), cur_page.before.clone(), cur_page.first), 
            move |_| {
                reports_clone.set(vec![]);
                loading_clone.set(true);

                // let cur_page_clone = cur_page.clone();
                // let reports_clone_cleapup = reports_clone.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if !base_url.is_empty() {
                        log!("Fetching Vuln..2");
                        let resp = client.get_vuls(full_path,cur_page.after.clone(), cur_page.before.clone() , cur_page.first.clone()).await;
                        
                        let resp_body: GraphqlResponse<get_vuls::ResponseData> =
                            resp.json().await.unwrap();

                        let report_data: get_vuls::GetVulsProject = resp_body
                        .data
                        .unwrap()
                        .project
                        .unwrap();

                        project_name_with_namespaces_clone.set(report_data.name_with_namespace);

                        let report_data = report_data
                            .vulnerabilities
                            .unwrap();

                        page_info.set(report_data.page_info);

                        let report_data: Vec<Vulnarable> = report_data
                            .nodes
                            .unwrap()
                            .into_iter()
                            .map(|data| {
                                if let Some(vul) = data {
                                    let location = vul.location.unwrap();
                                    let identifiers = vul.identifiers;

                                    let location : Location = match location {
                                        // get_vuls::GetVulProjectVulnerabilitiesNodesLocation::VulnerabilityLocationClusterImageScanning(data) => {
                                        //     Location {blob_path:data.blob_path, file: todo!(), start_line: todo!() }
                                            
                                        // },
                                        // get_vuls::GetVulProjectVulnerabilitiesNodesLocation::VulnerabilityLocationContainerScanning(data) => todo!(),
                                        // get_vuls::GetVulProjectVulnerabilitiesNodesLocation::VulnerabilityLocationCoverageFuzzing => todo!(),
                                        // get_vuls::GetVulProjectVulnerabilitiesNodesLocation::VulnerabilityLocationDast(data) => todo!(),
                                        get_vuls::GetVulsProjectVulnerabilitiesNodesLocation::VulnerabilityLocationDependencyScanning(data) => {
                                            Location {
                                                blob_path: data.blob_path.unwrap_or(String::new()),
                                                file: data.file.unwrap_or(String::new()),
                                                start_line: String::from("0"),
                                                end_line: String::from("0"),
                                            }
                                        },
                                        // get_vuls::GetVulProjectVulnerabilitiesNodesLocation::VulnerabilityLocationGeneric => todo!(),
                                        get_vuls::GetVulsProjectVulnerabilitiesNodesLocation::VulnerabilityLocationSast(data) => {
                                            Location {
                                                blob_path: data.blob_path.unwrap_or(String::new()),
                                                file: data.file.unwrap_or(String::new()),
                                                start_line: data.start_line.unwrap_or(String::from("0")),
                                                end_line: data.end_line.unwrap_or(String::from("0"))
                                            }
                                        },
                                        get_vuls::GetVulsProjectVulnerabilitiesNodesLocation::VulnerabilityLocationSecretDetection(data) => {
                                            Location {
                                                blob_path: data.blob_path.unwrap_or(String::new()),
                                                file: data.file.unwrap_or(String::new()),
                                                start_line: data.start_line.unwrap_or(String::from("0")),
                                                end_line: data.end_line.unwrap_or(String::from("0"))
                                            }
                                        },
                                        _ => Location::default()
                                    };

                                    // if let VulnerabilityLocationSast(data) = location {};
                                    // let me = location.0;
                                    //

                                    let report_type = match vul.report_type.unwrap() {
                                        get_vuls::VulnerabilityReportType::SAST => "SAST",
                                        get_vuls::VulnerabilityReportType::DEPENDENCY_SCANNING => "DEPENDENCY SCANNING",
                                        get_vuls::VulnerabilityReportType::CONTAINER_SCANNING => "CONTAINER SCANNING",
                                        get_vuls::VulnerabilityReportType::DAST => "DAST",
                                        get_vuls::VulnerabilityReportType::SECRET_DETECTION => "SECRET DETECTION",
                                        get_vuls::VulnerabilityReportType::COVERAGE_FUZZING => "COVERAGE FUZZING",
                                        get_vuls::VulnerabilityReportType::API_FUZZING => "API FUZZING",
                                        get_vuls::VulnerabilityReportType::CLUSTER_IMAGE_SCANNING => "CLUSTER IMAGE SCANNING",
                                        get_vuls::VulnerabilityReportType::CONTAINER_SCANNING_FOR_REGISTRY => "CONTAINER SCANNING FOR REGISTRY",
                                        get_vuls::VulnerabilityReportType::GENERIC => "GENERIC",
                                        get_vuls::VulnerabilityReportType::Other(_) => "OTHERS",
                                    };

                                    return Vulnarable {
                                        id: vul.id,
                                        detected: vul.detected_at,
                                        title: vul.title.unwrap_or(String::new()),
                                        description: vul.description.unwrap_or(String::new()),
                                        scanner_name: vul.scanner.unwrap().name,
                                        location,
                                        identifiers,
                                        report_type: report_type.to_string(),
                                        severity: vul.severity.unwrap(),
                                    };                            
                                } else {
                                    Vulnarable::default()
                                }
                            })
                            .collect();

                        reports_clone.set(report_data);
                        loading_clone.set(false);
                    }
                });

                move || {
                    // cur_page_clone.set(CurrentPagination{
                    //     after: None,
                    //     before: None,
                    //     first : Some(100)
                    // });
                    // reports_clone_cleapup.set(vec![]);
                }
            });
    }

    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone();

    let project_list_ctx = use_context::<ProjectListContext>().unwrap();

    let get_project = project_list_ctx.data
        .iter()
        .find(|p| p.full_path == full_path)
        .unwrap_or(&Project::default())
        .clone();

    let detail_ctx = use_context::<ProjectDetailContext>().unwrap();
    let onclick = Callback::from(move |set_project: ProjectDetail| {
        let vul_id = set_project.data.id.clone();
        detail_ctx.dispatch(set_project);
        navigator.push(&Route::Detail{ project_path: get_project.full_path.clone(), vul_id })
    });

    if *loading {
        return html!{
            <div class="items-center flex pt-10 flex-col h-full w-full">
                <Loader class="size-3"/>
            </div>
        };
    };

    if reports.clone().len() == 0 {
        return html!{
            <div class="items-center flex justify-center h-full flex-col">
                <img src="/public/gitlab_security.svg" />
                <p class="text-3xl font-black text-zinc-400 my-2">{ "No vulnerabilities to report " }</p>
                <p class="leading-none">{ "If you were expecting vulnerabilities to be shown here, check that you've" }</p>
                    <span>
                    { "completed the" }{" "}
                        {
                            html!{
                                <a class="hover:!underline cursor-pointer" href="https://docs.gitlab.com/ee/user/application_security/configuration/index.html">{ " security scanning prerequisites" }</a>
                            }
                        }
                        { ", or check the other vulnerability" }
                    </span>
                <p class="leading-none">{ "types in the tabs above." }</p>

                <a class="mt-4 px-3 py-1 rounded-lg hover:bg-sky-950 bg-sky-900 decoration-none !text-stone-400" target="_blank" href="https://docs.gitlab.com/ee/user/application_security/configuration/index.html">
                    {"Learn more about security configuration"}
                </a>
            </div>
        };

    };


    let td_class = "p-2 border-t border-neutral-800";
    let th_class = "p-2 self-start";
    // let navigator = use_navigator().unwrap();

    let handle_pagination_click = Callback::from({
        let cur_page = cur_page.clone();
        move |page_info: CurrentPagination| {
            cur_page.set(page_info);
        }
    });

    if reports.clone().len() > 0 {

        return html! {
            <div class="pb-3">
            <table class="table-fixed text-sm text-left w-full">
                <thead class="sticky top-0 bg-neutral-900 font-bold text-neutral-300">
                    <tr>
                        <th class={format!("{} {}",th_class,"w-8")}><CheckBox /></th>
                        <th class={format!("{} {}",th_class,"w-25")}>{ "Detected" }</th>
                        <th class={th_class}>{ "Status" }</th>
                        <th class={th_class}>{ "Severity" }</th>
                        <th class={format!("{} {}",th_class,"w-115")}>{ "Description" }</th>
                        <th class={format!("{} {}",th_class,"w-40")}>{ "Identifier" }</th>
                        <th class={th_class}>{ "Tool" }</th>
                        <th class={format!("{} {}",th_class,"")}>{ "Activity" }</th>
                    </tr>
                </thead>
                <tbody class="h-full overflow-y-auto">
                    {
                        reports.iter().map(|report| {

                            html!{
                                <tr class="border-t">
                                <td class={td_class}><CheckBox /></td>
                                <td class={td_class}>{ report.detected.clone().split('T').next().unwrap_or("") }</td>
                                <td class={td_class}>{ "Needs Triage" }</td>
                                <td class={td_class}>
                                    <BugSeverityLevel
                                        level={
                                            match report.severity {
                                                VulnerabilitySeverity::INFO => "Info",
                                                VulnerabilitySeverity::UNKNOWN => "Unknown",
                                                VulnerabilitySeverity::LOW => "Low",
                                                VulnerabilitySeverity::MEDIUM => "Medium",
                                                VulnerabilitySeverity::HIGH => "High",
                                                VulnerabilitySeverity::CRITICAL => "Critical",
                                                VulnerabilitySeverity::Other(_) => "Other",
                                            }
                                        }
                                    />
                                </td>
                                <td class={td_class}>
                                    <p onclick={Callback::from({
                                        let data = report.clone();
                                        let project_name_with_namespaces_clone = project_name_with_namespaces.clone();

                                        let project_detail = ProjectDetail::new(data.clone(), project_name_with_namespaces_clone.to_string());
                                        let onclick_clone = onclick.clone();

                                        move |_| onclick_clone.emit(project_detail.clone())
                                    })} class="cursor-pointer hover:underline text-neutral-400 text-base leading-none">{ report.title.clone() }</p>
                                    <p class="text-neutral-500">{ project_name_with_namespaces.to_string() }</p>
                                    <a class="cursor-pointer hover:!underline text-xs text-sky-400 line-clamp-1" onclick={
                                        
                                        Callback::from({

                                            let project_path = full_path.clone();

                                            let path: String = report.location.file.clone();

                                            let blob_path = report.location.blob_path.clone();

                                            let with_ref = if !blob_path.is_empty(){
                                                blob_path.split("/blob/").nth(1)
                                                .and_then(|s| s.split('/').next())
                                                .unwrap_or("")
                                            } else {
                                                ""
                                            }.to_string();

                                            let navigator_clone = navigator_clone.clone();

                                            let start_line = report.location.start_line.clone();
                                            let end_line = report.location.end_line.clone();

                                            let blob_path = Route::Editor { project_path, with_ref, path, start_line, end_line, title: report.title.clone() };
                                                        
                                            move |_| {
                                                navigator_clone.push(
                                                    &blob_path
                                                )
                                            }
                                        })
                                    }>{ 
                                        if report.location.start_line != "0" {
                                            format!("{}:{}", report.location.file.clone(), report.location.start_line.clone())
                                        } else {
                                            report.location.file.clone()
                                        }
                                    }</a>
                                </td>
                                <td class={td_class}>{ 
                                    if report.identifiers.len() > 0{
                                        html!{
                                            <>
                                                <p class="line-clamp-3">{ report.identifiers[0].name.clone()}</p>
                                                <p>{ 
                                                    if report.identifiers.len() > 1 {
                                                        format!("+ {} more",report.identifiers.len() - 1 )
                                                    }else {
                                                        String::new()
                                                    }
                                                }
                                                </p>
                                            </>
                                        }

                                    }else {
                                        html!{
                                            <p></p>
                                        }
                                    }
                                }</td>
                                <td class={td_class}>{ report.report_type.clone() }</td>
                                <td class={td_class}>{ "" }</td>
                                </tr>
                            } 
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
            <div class="w-full w-full flex items-center justify-center gap-2 mt-2">
                        
                        <div 
                            onclick={
                                Callback::from({
                                    let set_cur_page = CurrentPagination {
                                        before: page_info.start_cursor.clone(),
                                        after: None,
                                        first: Some(100)
                                    };

                                    let handle_pagination_click = handle_pagination_click.clone();
                                    let page_info = page_info.clone();

                                    move |_| {
                                        if page_info.has_previous_page {
                                            handle_pagination_click.emit(set_cur_page.clone())
                                        }
                                    } 
                                })
                            }
                            class={
                                if page_info.has_previous_page {
                                    "cursor-pointer flex items-center justify-center outline text-gray-400 outline-gray-400 rounded px-2 py-1"
                                } else {
                                    "cursor-not-allowed flex items-center justify-center text-gray-500 outline outline-gray-500 rounded px-2 py-1"
                                }
                            }
                        >
                            <Icon
                                icon_id={IconId::HeroiconsOutlineChevronLeft}
                                onclick={move |_| {}}
                                height={"1.2em".to_owned()}
                                width={"1.2em".to_owned()}
                            />
                            <span>{"Previous"}</span>
                        </div>
                        <div 
                            onclick={
                                Callback::from({
                                    let set_cur_page = CurrentPagination {
                                        before: None,
                                        after: page_info.end_cursor.clone(),
                                        first: Some(100)
                                    };
                                    let handle_pagination_click = handle_pagination_click.clone();
                                    
                                    let page_info = page_info.clone();

                                    move |_| {
                                        if page_info.has_next_page {
                                            handle_pagination_click.emit(set_cur_page.clone())
                                        }
                                    } 
                                })
                            }
                            class={
                                if page_info.has_next_page {
                                    "cursor-pointer flex items-center justify-center outline text-gray-400 outline-gray-400 rounded px-2 py-1"
                                } else {
                                    "cursor-not-allowed flex items-center justify-center text-gray-500 outline outline-gray-500 rounded px-2 py-1"
                                }
                            }
                        >
                            <span>{"Next"}</span>
                            <Icon
                                icon_id={IconId::HeroiconsOutlineChevronRight}
                                onclick={move |_| {}}
                                height={"1.2em".to_owned()}
                                width={"1.2em".to_owned()}
                            />
                        </div>
            </div>
            </div>
        }
    }

    html!()
}
