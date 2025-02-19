use gloo_console::log;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew_router::prelude::*;
use graphql_client::Response as GraphqlResponse;
use regex::Regex;

use crate::api::{get_route_full_path, time_ago};
use crate::api::gitlab::{get_vulnerability_blob_info, get_vul};
use crate::api::gitlab::get_vul::VulnerabilitySeverity;
use crate::atoms::{BugSeverityLevel, HelpIcon, Loader, Link};
use crate::container::home::Route;
use crate::context::api_context::ApiContext;
use crate::context::project_context::Project;
use crate::context::project_list_context::ProjectListContext;

const CVSS_HELP: &str = "The CVSS (Common Vulnerability Scoring System) is a standardized framework for assessing and communicating the severity of security vulnerabilities in software. It provides a numerical score (ranging from 0.0 to 10.0) to indicate the severity risk of the vulnerability.";
const EPSS_HELP: &str = "The Exploit Prediction Scoring System model produces a probability score between 0 and 1 indicating the likelihood that a vulnerability will be exploited in the next 30 days.";
const KEV_HELP: &str = r#"CISA (the Cybersecurity & Infrastructure Security Agency, a part of the U.S. Department of Homeland Security) maintains the Known Exploited Vulnerabilities (aka "KEV") catalog of vulnerabilities that have been exploited in the wild."#;
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Location {
    pub blob_path: String,
    pub file: String,
    pub start_line: String,
    pub end_line: String,
}

impl Default for get_vul::VulnerabilitySeverity {
    fn default() -> Self {
        get_vul::VulnerabilitySeverity::INFO
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

impl PartialEq for get_vul::GetVulVulnerabilityIdentifiers {
    fn eq(&self, other: &Self) -> bool {
        self.external_type == other.external_type && self.name == other.name
    }
}

impl Clone for get_vul::GetVulVulnerabilityIdentifiers {
    fn clone(&self) -> Self {
        Self { external_type: self.external_type.clone(), name: self.name.clone(), url: self.url.clone() }
    }
}

impl Clone for get_vulnerability_blob_info::GetVulnerabilityBlobInfoProjectRepositoryBlobsNodes{
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), raw_text_blob: self.raw_text_blob.clone(), language: self.language.clone() }
    }
}

impl PartialEq for get_vul::GetVulVulnerabilityCvss {
    fn eq(&self, other: &Self) -> bool {
        self.overall_score == other.overall_score && self.version == other.version
    }
}

impl Clone for get_vul::GetVulVulnerabilityCvss {
    fn clone(&self) -> Self {
        Self { overall_score: self.overall_score.clone(), version: self.version.clone() }
    }
}

impl PartialEq for get_vul::GetVulVulnerabilityCveEnrichment {
    fn eq(&self, other: &Self) -> bool {
        self.epss_score == other.epss_score && self.is_known_exploit == other.is_known_exploit
    }
}

impl Clone for get_vul::GetVulVulnerabilityCveEnrichment {
    fn clone(&self) -> Self {
        Self { epss_score: self.epss_score.clone(), is_known_exploit: self.is_known_exploit.clone() }
    }
}

impl Clone for get_vul::GetVulVulnerabilityLinks {
    fn clone(&self) -> Self {
        Self { url: self.url.clone(), name: self.name.clone() }
    }
}

impl PartialEq for get_vul::GetVulVulnerabilityLinks {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url && self.name == other.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddRepoArgs<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub path: &'a str,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Vulnarable {
    pub id: String,
    pub detected: String,
    pub severity: get_vul::VulnerabilitySeverity,
    pub title: String,
    pub description: String,
    pub location: Location,
    pub report_type: String,
    pub scanner_name: Option<String>,
    // pub scanner: get_vul::GetVulProjectVulnerabilitiesNodesScanner,
    pub identifiers: Vec<get_vul::GetVulVulnerabilityIdentifiers>,
    pub cvss: Vec<get_vul::GetVulVulnerabilityCvss>,
    pub cve_enrichment: Option<get_vul::GetVulVulnerabilityCveEnrichment>,
    pub links: Vec<get_vul::GetVulVulnerabilityLinks>,
    pub solution: Option<String>
}

#[wasm_bindgen(module = "/src/js/monaco_loader_detail.js")]
unsafe extern "C" {
    fn initMonaco(element_id: &str,content: &str, language: &str , start_line: usize);
}

fn parse_text(input: &str) -> String {
    let mut result = String::new();
    let mut in_code_block = false;
    let mut in_inline_code = false;

    let lines: Vec<&str> = input.lines().collect();
    
    for line in lines {
        if line.trim() == "```" {
            in_code_block = !in_code_block;
            if in_code_block {
                result.push_str("<pre class=\"outline px-3 p-2 mx-1 mt-2 rounded-md\"><code class=\"text-sm\">");
            } else {
                result.push_str("</code></pre>\n");
            }
            continue;
        }

        if in_code_block {
            result.push_str(line);
            result.push('\n');
            continue;
        }

        let mut formatted_line = String::new();
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '`' {
                in_inline_code = !in_inline_code;
                if in_inline_code {
                    formatted_line.push_str("<code class=\"bg-stone-800 px-2 rounded-md\">");
                } else {
                    formatted_line.push_str("</code>");
                }
            } else {
                formatted_line.push(c);
            }
        }

        if formatted_line.is_empty(){
            result.push_str("</br></br>")

        }

        if let Some(text_start) = line.find('[') {
            if let Some(text_end) = line.find(']') {
                let text = &line[text_start + 1..text_end];
                if let Some(url_start) = line.find('(') {
                    if let Some(url_end) = line.find(')') {
                        // let text = &line[text_start + 1..text_end];
                        let url = &line[url_start + 1..url_end];
                        let html_link = format!(r#"<a href="{}" target="_blank">{}</a> "#, url, text);
                        log!("lets {}",&html_link);
    
                        let mut output = String::new();
                        output.push_str(&line[..text_start]);
                        output.push_str(&html_link);
                        output.push_str(&line[url_end + 1..]);

                        result.push_str(&output);
                        continue;
                    }
                }
            }
        }

        if line.trim().contains("https://") {
            let re = Regex::new(r"(https?://[^\s]+)").unwrap();

            let replaced_string = re.replace_all(line, r#"<a href="$1" target="_blank">$1</a> "#).to_string();
            result.push_str(&replaced_string);

            continue;
        }

        result.push_str(&formatted_line);
        result.push_str("\n");
    }

    result
}

#[function_component]
pub fn Detail() -> Html {
    let api = use_context::<ApiContext>().expect("no ctx found");

    let report: UseStateHandle<Vulnarable>  = use_state(|| Vulnarable::default());
    let loading = use_state(|| true);
    
    let route = use_route::<Route>().expect("Failed to get route");
    let vul_id = match route.clone() {
        Route::Detail { project_path: _, vul_id } => vul_id,
        _ => String::new(),
    };

    let route_full_path = get_route_full_path(route);

    let project_list_ctx = use_context::<ProjectListContext>().unwrap();

    let get_project = project_list_ctx.data
        .iter()
        .find(|p| p.full_path == route_full_path)
        .unwrap_or(&Project::default())
        .clone();


    let path = use_state(|| String::new());
    let ref_ = use_state(|| String::new());

    let start_line = use_state(|| String::new());
    let end_line = use_state(|| String::new());

    let code_line = use_state(|| 0);


    {
            log!("Fetching Blob Info Vuln..");
            let client = api.client.to_owned();
            let base_url = api.client.base_url.to_owned();
    
            let project_path = get_project.full_path.clone();
            let path = path.clone();
            let ref_ = ref_.clone();

            let start_line = start_line.clone();
            let end_line = end_line.clone();
            let code_line = code_line.clone();
    
            use_effect_with((base_url.clone(),path.clone(),ref_.clone()), move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if !base_url.is_empty() && !path.is_empty() && !ref_.is_empty() && *start_line != "0" && !start_line.is_empty() {
                        log!("Fetching Vuln..2");
                        let resp = client.get_vulnerability_raw_blob_info(
                            project_path,
                            path.to_string(),
                            ref_.to_string()
                        ).await;

    
                        let resp_body: GraphqlResponse<get_vulnerability_blob_info::ResponseData> =
                            resp.json().await.unwrap();
    
                        let blob_data = resp_body
                        .data
                        .unwrap()
                        .project
                        .unwrap().repository.unwrap().blobs.unwrap().nodes.unwrap();
    
                        if blob_data.len() > 0 {
                            let raw_blob_data: Option<get_vulnerability_blob_info::GetVulnerabilityBlobInfoProjectRepositoryBlobsNodes> = blob_data[0].clone();
    
                            match raw_blob_data {
                                Some(data) => {
                                    let content = data.raw_text_blob.clone().unwrap_or(String::new());

                                    let mut code_lines: Vec<String> = vec![];

                                    let mut lines: Vec<String> = content.split('\n').map(String::from).collect();

                                    
                                    let start_line = (*start_line).clone().parse::<usize>().unwrap_or(1);
                                    let end_line = (*end_line).clone().parse::<usize>().unwrap_or(start_line);

                                    if start_line != end_line {
                                        code_line.set(end_line - start_line);
                                    } else {
                                        code_line.set(1);
                                    }


                                    for (i, line) in lines.iter_mut().enumerate() {
                                        if (start_line-1..=end_line-1).contains(&i) {
                                            code_lines.push(line.to_string());
                                        }
                                    }

                                    let result = code_lines.join("\n");


                                    log!("blob !!{}",result.clone());

                                    log!(format!("start_line !!{:#?}",start_line));
                                    log!(format!("end_line !!{:#?}",end_line));
                                    let language = data.language.unwrap_or(String::new());
                                    initMonaco("specific-code",&result,&language,start_line);
    
                                    // blob_info.set(BlobInfo {
    
                                    //     path: data.path,
                                    //     size: data.size.unwrap_or(String::new()),
                                    //     language,
                                    //     name: data.name.unwrap_or(String::new())
                                    // });
                                },
                                None => {},
                            }
                        }
                    }
                });
                
                || ()
            });
        }

    

    {
        log!("Fetching Vuln..");
        let client = api.client.to_owned();
        let base_url = api.client.base_url.to_owned();
        let report_clone = report.clone();
        let vul_id = vul_id.clone();
        let loading_clone = loading.clone();

        let path = path.clone();
        let ref_ = ref_.clone();
        let start_line = start_line.clone();
        let end_line = end_line.clone();

        use_memo((base_url.clone(), vul_id.clone()), move |_| {
            loading_clone.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                if !base_url.is_empty() {
                    log!("Fetching Vuln..2");
                    let resp = client.get_vul(vul_id).await;
                    let resp_body: GraphqlResponse<get_vul::ResponseData> =
                        resp.json().await.unwrap();

                    let vul= resp_body
                    .data
                    .unwrap()
                    .vulnerability
                    .unwrap();

                    let location: get_vul::GetVulVulnerabilityLocation = vul.location.unwrap();
                    let identifiers: Vec<get_vul::GetVulVulnerabilityIdentifiers> = vul.identifiers;

                    let location : Location = match location {
                        // get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationClusterImageScanning(get_vul_vulnerability_location_on_vulnerability_location_cluster_image_scanning) => todo!(),
                        // get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationContainerScanning(get_vul_vulnerability_location_on_vulnerability_location_container_scanning) => todo!(),
                        // get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationCoverageFuzzing => todo!(),
                        // get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationDast(get_vul_vulnerability_location_on_vulnerability_location_dast) => todo!(),
                        get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationDependencyScanning(data) => {
                            path.set(data.file.clone().unwrap_or(String::new()));

                            let blob_path = data.blob_path.unwrap_or(String::new());

                            ref_.set(blob_path.clone().split("/blob/").nth(1)
                            .and_then(|s| s.split('/').next())
                            .unwrap_or(&String::new()).to_string());
                            
                            Location {
                                blob_path,
                                file: data.file.unwrap_or(String::new()),
                                start_line: String::from("0"),
                                end_line: String::from("0")
                            }
                        },
                        // get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationGeneric => todo!(),
                        get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationSast(data) => {
                            start_line.set(data.start_line.clone().unwrap_or(String::new()));
                            end_line.set(data.end_line.clone().unwrap_or(String::new()));

                            path.set(data.file.clone().unwrap_or(String::new()));

                            let blob_path = data.blob_path.unwrap_or(String::new());

                            ref_.set(blob_path.clone().split("/blob/").nth(1)
                            .and_then(|s| s.split('/').next())
                            .unwrap_or(&String::new()).to_string());

                            Location {
                                blob_path,
                                file: data.file.unwrap_or(String::new()),
                                start_line: data.start_line.unwrap_or(String::from("0")),
                                end_line: data.end_line.unwrap_or(String::from("0"))
                            }
                        },
                        get_vul::GetVulVulnerabilityLocation::VulnerabilityLocationSecretDetection(data) => {
                            start_line.set(data.start_line.clone().unwrap_or(String::new()));
                            end_line.set(data.end_line.clone().unwrap_or(String::new()));

                            path.set(data.file.clone().unwrap_or(String::new()));

                            let blob_path = data.blob_path.unwrap_or(String::new());

                            ref_.set(blob_path.clone().split("/blob/").nth(1)
                            .and_then(|s| s.split('/').next())
                            .unwrap_or(&String::new()).to_string());

                            Location {
                                blob_path,
                                file: data.file.unwrap_or(String::new()),
                                start_line: data.start_line.unwrap_or(String::from("0")),
                                end_line: data.end_line.unwrap_or(String::from("0"))
                            }
                        },
                        _ =>  Location::default()
                    };

                    let report_type = match vul.report_type.unwrap() {
                        get_vul::VulnerabilityReportType::SAST => "SAST",
                        get_vul::VulnerabilityReportType::DEPENDENCY_SCANNING => "DEPENDENCY SCANNING",
                        get_vul::VulnerabilityReportType::CONTAINER_SCANNING => "CONTAINER SCANNING",
                        get_vul::VulnerabilityReportType::DAST => "DAST",
                        get_vul::VulnerabilityReportType::SECRET_DETECTION => "SECRET DETECTION",
                        get_vul::VulnerabilityReportType::COVERAGE_FUZZING => "COVERAGE FUZZING",
                        get_vul::VulnerabilityReportType::API_FUZZING => "API FUZZING",
                        get_vul::VulnerabilityReportType::CLUSTER_IMAGE_SCANNING => "CLUSTER IMAGE SCANNING",
                        get_vul::VulnerabilityReportType::CONTAINER_SCANNING_FOR_REGISTRY => "CONTAINER SCANNING FOR REGISTRY",
                        get_vul::VulnerabilityReportType::GENERIC => "GENERIC",
                        get_vul::VulnerabilityReportType::Other(_) => "OTHERS",
                    };

                    let cvss: Vec<get_vul::GetVulVulnerabilityCvss> = vul.cvss;

                    let cve_enrichment: Option<get_vul::GetVulVulnerabilityCveEnrichment> = vul.cve_enrichment;

                    let links: Vec<get_vul::GetVulVulnerabilityLinks> = vul.links;

                    let solution = vul.solution;

                    let vul_data = Vulnarable {
                        id: vul.id.clone(),
                        detected: vul.detected_at,
                        title: vul.title.unwrap_or(String::new()),
                        description: vul.description.unwrap_or(String::new()),
                        scanner_name: vul.scanner.unwrap().name,
                        location,
                        identifiers,
                        report_type: report_type.to_string(),
                        severity: vul.severity.unwrap(),
                        cvss,
                        cve_enrichment,
                        links,
                        solution
                    };

                    report_clone.set(vul_data);
                    loading_clone.set(false);
                }
            });
            || ()
        });
    }

    let route = use_route::<Route>().expect("Failed to get route");
    let full_path = get_route_full_path(route);
    let navigator = use_navigator().unwrap();
    let navigator_clone = navigator.clone();

    if *loading {
        return html!{
            <div class="bg-gradient-to-b from-neutral-900 to-neutral-950 flex flex-col text-neutral-400 p-5 bg-neutral-900 rounded-md w-full h-full overflow-hidden items-center flex pt-10">
                <Loader class="size-3"/>
            </div>
        };
    };

    html! {
        <div class="flex flex-col bg-gradient-to-b from-neutral-900 to-neutral-950 z-0 items-start text-neutral-400 px-5 py-2 bg-neutral-900 rounded-md w-full h-screen">
            <div class="flex mb-5 items-center w-full border-b border-neutral-800">
                {
                    get_project.full_path.clone().split("/").into_iter().map( |name| {
                        html!{
                            <>
                                <span class="mx-1 text-lg">{"/"}</span>
                                <span class="">{name}</span>
                            </>
                        }
                    }).collect::<Html>()
                }
                <span class="ml-2 mx-1 text-lg">{"#"}</span>
                <a class="text-sm underline">
                    {  
                        report.id.split("/Vulnerability/").nth(1)
                            .unwrap_or("")
                    }
                </a>
            </div>
            <div class="overflow-y-auto z-0 w-full flex flex-col items-start h-full pb-50 custom-scrollbar">
                <div class="flex items-center text-sm">
                    <span class="px-2 bg-yellow-900 text-yellow-500 text-sm rounded-lg">{"Needs triage"}</span>
                    <span class="ml-2 font-medium">{format!("Detected - {} ago",time_ago(report.detected.clone()))}</span>
                </div>
                
                <h1 class="font-black mt-3 text-2xl text-neutral-400 mt-4 !text-left">{ 
                    report.title.clone()
                }</h1>
                <span class="text-lg text-neutral-400 font-black my-1">{"Description"}</span>

                <span class="my-2 leading-6 text-base">{ 
                        Html::from_html_unchecked(parse_text(&report.description.clone()).into())
                }</span>

                <div class="flex items-center">
                    <span class="mr-2 font-bold">{"Severity:"}</span>
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
                </div>

                {
                    report.cvss.clone().into_iter().map(|data| {
                        html!{
                            <div class="flex items-center mb-1">
                                <span class="mr-2 font-bold">{format!("CVSS: v{}",data.version)}</span>
                                <span class="mr-2 text-xs px-2 bg-red-950 outline rounded-full outline-red-700">{ data.overall_score }</span>
                                <HelpIcon 
                                    class="" 
                                    help={CVSS_HELP}
                                    head="CVSS"
                                />
                            </div>
                        }
                    }).collect::<Html>()
                }
                {
                    if let Some(cve_enrichment) = &report.cve_enrichment {
                        html!{
                            <>
                                <div class="flex items-center mb-1">
                                    <span class="mr-2 font-bold">{ "EPSS:"}</span>
                                    <span class="mr-2 text-xs px-2 bg-sky-950 outline rounded-full outline-sky-700">{ format!("{}%",cve_enrichment.epss_score) }</span>
                                    <HelpIcon 
                                        class="" 
                                        help={EPSS_HELP}
                                        head="EPSS"
                                    />
                                </div>
                                <div class="flex items-center mb-1">
                                    <span class="mr-2 font-bold">{ "Has Known Exploit (KEV):"}</span>
                                    <span class="mr-2 text-xs px-2 bg-sky-950 outline rounded-full outline-sky-700">{ 
                                        if cve_enrichment.is_known_exploit {
                                            "YES"
                                        } else {
                                            "NO"
                                        }
                                    }</span>
                                    <HelpIcon 
                                        class="" 
                                        help={KEV_HELP}
                                        head="'Has known exploit (KEV)'"
                                    />
                                </div>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }

                <div class="flex items-center">
                    <span class="mr-2 font-bold">
                        {"Project: "}
                        <a>{ get_project.full_path }</a>
                    </span>
                </div>

                <div class="flex items-center my-1">
                    <span class="mr-2 font-bold">
                        {"Tool:"}
                    </span>
                    <span>
                        { report.report_type.clone() }
                    </span>
                </div>

                <div class="flex items-center">
                    <span class="mr-2 font-bold">
                        {"Scanner:"}
                    </span>
                    <span>
                        { report.scanner_name.clone() }
                    </span>
                </div>

                <span class="text-lg text-neutral-400 font-black my-2 mt-4">{"Location"}</span>

                <div class="flex items-center mt-1">
                    <span class="mr-3 font-bold">
                        {"File: "}
                        <a onclick={
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
                                let start_line: String = report.location.start_line.clone();
                                let end_line: String = report.location.end_line.clone();

                                let blob_path = Route::Editor { project_path, with_ref, path,start_line,end_line, title: report.title.clone()};
                                            
                                move |_| {
                                    navigator_clone.push(
                                        &blob_path
                                    )
                                }
                            })
                        }>{ 
                            if !report.location.start_line.is_empty() && report.location.start_line != "0" {
                                if report.location.end_line != "0" {
                                    format!("{}:{}-{}", report.location.file.clone(), report.location.start_line.clone(), report.location.end_line.clone())
                                } else {
                                    format!("{}:{}", report.location.file.clone(), report.location.start_line.clone())
                                }
                            } else {
                                report.location.file.clone()
                            }
                        }</a>
                    </span>
                </div>

                {
                    if *start_line != "0" && !start_line.is_empty() {
                        html!{
                            <div class={
                                format!("w-full rounded-md mt-2")
                            }>
                                <div id="specific-code" class="overflow-hidden rounded-md pr-3"></div>
                            </div>
                        }
                    } else {
                        html!{
                            <></>
                        }
                    }
                }


                <span class="text-lg text-neutral-400 font-black my-2 mt-4">{"Identifiers"}</span>

                <ul class="list-disc text-md pl-5">
                    {
                        report.identifiers.iter().map(|ide| {
                            html!{
                                if let Some(url) = ide.url.clone() {
                                    <li>
                                        <Link url={url.clone()} class="" name={ide.name.clone().unwrap_or(String::new())} />
                                    </li>
                                } else {
                                    <li><span class="">{ ide.name.clone() }</span></li>
                                }
                            }
                        })
                        .collect::<Html>()
                    }
                </ul>

                if report.links.len() > 0 {

                    <span class="text-lg text-neutral-400 font-black my-2 mt-4">{"Links"}</span>
                    <ul class="list-disc text-md pl-5">
                        {
                            report.links.iter().map(|link| {
                                html!{
                                    <li>
                                        <Link url={link.url.clone()} class="" name={link.name.clone().unwrap_or(String::new())} />
                                    </li>
                                }
                            })
                            .collect::<Html>()
                        }
                    </ul>
                }
 

                if let Some(solution) = report.solution.clone() {

                    <span class="text-lg text-neutral-400 font-black my-2 mt-4">{"Solution"}</span>
    
                    <span class="leading-6 text-base">{ 
                        solution
                    }</span>
                }
                
            </div>
        </div>
    }
}
