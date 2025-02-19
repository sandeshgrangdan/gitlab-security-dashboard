use gloo_console::log;
use graphql_client::Response as GraphqlResponse;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::gitlab::{get_blob_info, path_last_commit};
use crate::api::time_ago;
use crate::atoms::{ CopyIcon, GitBranch };
use crate::container::home::Route;
use crate::context::api_context::ApiContext;

#[wasm_bindgen(module = "/src/js/monaco_loader.js")]
extern "C" {
    fn initMonaco(element_id: &str,content: &str, language: &str , start_line: usize , end_line: usize);
}

#[derive(Debug, Default)]
pub struct BlobInfo {
    pub path: String,
    pub size: String,
    pub language: String,
    pub name: String,
}

#[derive(Debug, Default)]
struct CommitInfo {
    title: String,
    author_name: String,
    author_date: String,
}

impl Clone for get_blob_info::GetBlobInfoProjectRepositoryBlobsNodes {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone(), size: self.size.clone(), raw_size: self.raw_size.clone(), raw_text_blob: self.raw_text_blob.clone(), language: self.language.clone(), path: self.path.clone(), archived: self.archived.clone(), external_storage_url: self.external_storage_url.clone(), simple_viewer: self.simple_viewer.clone(), rich_viewer: self.rich_viewer.clone() }
    }
}

impl Clone for get_blob_info::GetBlobInfoProjectRepositoryBlobsNodesSimpleViewer {
    fn clone(&self) -> Self {
        Self { file_type: self.file_type.clone(), too_large: self.too_large.clone(), type_: self.type_.clone(), render_error: self.render_error.clone() }
    }
}

impl Clone for get_blob_info::BlobViewersType {
    fn clone(&self) -> Self {
        match self {
            Self::rich => Self::rich,
            Self::simple => Self::simple,
            Self::auxiliary => Self::auxiliary,
            Self::Other(arg0) => Self::Other(arg0.clone()),
        }
    }
}

impl Clone for get_blob_info::GetBlobInfoProjectRepositoryBlobsNodesRichViewer {
    fn clone(&self) -> Self {
        Self { file_type: self.file_type.clone(), too_large: self.too_large.clone(), type_: self.type_.clone(), render_error: self.render_error.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitAuthor{
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), name: self.name.clone(), avatar_url: self.avatar_url.clone(), web_path: self.web_path.clone() }
    }
}


impl Clone for path_last_commit::VerificationStatus {
    fn clone(&self) -> Self {
        match self {
            Self::UNVERIFIED => Self::UNVERIFIED,
            Self::VERIFIED => Self::VERIFIED,
            Self::SAME_USER_DIFFERENT_EMAIL => Self::SAME_USER_DIFFERENT_EMAIL,
            Self::OTHER_USER => Self::OTHER_USER,
            Self::UNVERIFIED_KEY => Self::UNVERIFIED_KEY,
            Self::UNKNOWN_KEY => Self::UNKNOWN_KEY,
            Self::MULTIPLE_SIGNATURES => Self::MULTIPLE_SIGNATURES,
            Self::REVOKED_KEY => Self::REVOKED_KEY,
            Self::VERIFIED_SYSTEM => Self::VERIFIED_SYSTEM,
            Self::VERIFIED_CA => Self::VERIFIED_CA,
            Self::Other(arg0) => Self::Other(arg0.clone()),
        }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitSignatureOnSshSignature {
    fn clone(&self) -> Self {
        Self { verification_status: self.verification_status.clone(), key_fingerprint_sha256: self.key_fingerprint_sha256.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitSignatureOnX509Signature{
    fn clone(&self) -> Self {
        Self { verification_status: self.verification_status.clone(), x509_certificate: self.x509_certificate.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitSignatureOnX509SignatureX509Certificate {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), subject: self.subject.clone(), subject_key_identifier: self.subject_key_identifier.clone(), x509_issuer: self.x509_issuer.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitSignatureOnX509SignatureX509CertificateX509Issuer{
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), subject: self.subject.clone(), subject_key_identifier: self.subject_key_identifier.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitSignatureOnGpgSignature {
    fn clone(&self) -> Self {
        Self { gpg_key_primary_keyid: self.gpg_key_primary_keyid.clone(), verification_status: self.verification_status.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitSignature {
    fn clone(&self) -> Self {
        match self {
            Self::GpgSignature(arg0) => Self::GpgSignature(arg0.clone()),
            Self::SshSignature(arg0) => Self::SshSignature(arg0.clone()),
            Self::X509Signature(arg0) => Self::X509Signature(arg0.clone()),
        }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitPipelinesEdges {
    fn clone(&self) -> Self {
        Self { node: self.node.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitPipelinesEdgesNode{
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), detailed_status: self.detailed_status.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitPipelinesEdgesNodeDetailedStatus{
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), details_path: self.details_path.clone(), icon: self.icon.clone(), tooltip: self.tooltip.clone(), text: self.text.clone(), group: self.group.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommitPipelines{
    fn clone(&self) -> Self {
        Self { edges: self.edges.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodesLastCommit {
    fn clone(&self) -> Self {
        Self { id: self.id.clone(), sha: self.sha.clone(), title: self.title.clone(), title_html: self.title_html.clone(), description_html: self.description_html.clone(), message: self.message.clone(), web_path: self.web_path.clone(), authored_date: self.authored_date.clone(), author_name: self.author_name.clone(), author_gravatar: self.author_gravatar.clone(), author: self.author.clone(), signature: self.signature.clone(), pipelines: self.pipelines.clone() }
    }
}

impl Clone for path_last_commit::PathLastCommitProjectRepositoryPaginatedTreeNodes {
    fn clone(&self) -> Self {
        Self { last_commit: self.last_commit.clone() }
    }
}



#[function_component]
pub fn Editor() -> Html {
    let api = use_context::<ApiContext>().expect("no ctx found");

    let blob_info = use_state(|| BlobInfo::default());
    let loading = use_state(|| false);

    let route = use_route::<Route>().expect("Failed to get route");

    let (project_path,ref_, path,start_line, end_line, title) = match route {
        Route::Editor { project_path, with_ref, path, start_line, end_line, title } => (project_path,with_ref,path,start_line, end_line,title),
        _ => (String::new(),String::new(),String::new(),String::new(), String::new(), String::new()),
    };

    {
        log!("Fetching Blob Info Vuln..");
        let client = api.client.to_owned();
        let base_url = api.client.base_url.to_owned();

        let blob_info = blob_info.clone();
        let loading = loading.clone();

        let project_path = project_path.clone();
        let path = path.clone();
        let ref_ = ref_.clone();
        let start_line = start_line.clone();
        let end_line = end_line.clone();

        use_effect_with(base_url.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if !base_url.is_empty() {
                    loading.set(true);
                    log!("Fetching Vuln..2");
                    let resp = client.get_blob_info(
                        project_path,
                        vec![path],
                        ref_
                    ).await;

                    let resp_body: GraphqlResponse<get_blob_info::ResponseData> =
                        resp.json().await.unwrap();

                    let blob_data = resp_body
                    .data
                    .unwrap()
                    .project
                    .unwrap().repository.unwrap().blobs.unwrap().nodes.unwrap();

                    if blob_data.len() > 0 {
                        let raw_blob_data: Option<get_blob_info::GetBlobInfoProjectRepositoryBlobsNodes>= blob_data[0].clone();

                        match raw_blob_data {
                            Some(data) => {
                                let content = data.raw_text_blob.clone().unwrap_or(String::new());
                                let language = data.language.unwrap_or(String::new());

                                let start_line = start_line.clone().parse::<usize>().unwrap_or(1);
                                let end_line = end_line.clone().parse::<usize>().unwrap_or(0);

                                   
                                initMonaco("monaco-container",&content,&language, start_line, end_line);

                                blob_info.set(BlobInfo {

                                    path: data.path,
                                    size: data.size.unwrap_or(String::new()),
                                    language,
                                    name: data.name.unwrap_or(String::new())
                                });
                            },
                            None => {},
                        }
                    }
                    loading.set(false);                    
                }
            });
            
            || ()
        });
    }

    let commit_info = use_state(|| CommitInfo::default());
    {

        log!("Fetching Commit Info Vuln..");
        let client = api.client.to_owned();
        let base_url = api.client.base_url.to_owned();

        let commit_info = commit_info.clone();
        let loading = loading.clone();

        let project_path = project_path.clone();
        let path = path.clone();
        let ref_ = ref_.clone();

        use_effect_with(base_url.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if !base_url.is_empty() {
                    loading.set(true);
                    log!("Fetching Vuln..2");
                    let resp = client.get_path_last_commit(
                        project_path,
                        Some(path),
                        ref_
                    ).await;

                    let resp_body: GraphqlResponse<path_last_commit::ResponseData> =
                        resp.json().await.unwrap();

                    let blob_data = resp_body
                    .data
                    .unwrap()
                    .project
                    .unwrap().repository.unwrap().paginated_tree.unwrap().nodes.unwrap();

                    if blob_data.len() > 0 {
                        let raw_blob_data = blob_data[0].clone();

                        match raw_blob_data {
                            Some(data) => {
                                let last_commit =  data.last_commit.unwrap();
                                if let Some(last_commit_author) = last_commit.author {
                                    commit_info.set(CommitInfo { 
                                        title: last_commit.title.unwrap_or(String::new()), 
                                        author_date: last_commit.authored_date.unwrap_or(String::new()), 
                                        author_name: last_commit_author.name
                                    });
                                } else {
                                    commit_info.set(CommitInfo { 
                                        title: last_commit.title.unwrap_or(String::new()), 
                                        author_date: last_commit.authored_date.unwrap_or(String::new()), 
                                        author_name: last_commit.author_name.unwrap_or(String::new())
                                    });
                                }
                            },
                            None => {},
                        }
                    }
                    loading.set(false);                    
                }
            });
            
            || ()
        });
    }

    let bytes: u32 = blob_info.size.clone().parse().unwrap_or(0);
    let kib = bytes as f64 / 1024.0;

    html! {
        <div class="flex flex-col items-start text-neutral-400 p-1 bg-neutral-900 rounded-md w-full h-full overflow-hidden gap-2">
            <div class="flex p-2 flex-wrap">
                {
                    project_path.clone().split("/").into_iter().map( |name| {
                        html!{
                            <>
                                <span class="mx-1 text-lg">{"/"}</span>
                                <span class={
                                    if name == blob_info.name {
                                        "text-neutral-300 font-bold"
                                    }else{
                                        ""
                                    }
                                }>{name}</span>
                            </>
                        }
                    }).collect::<Html>()
                }
                {
                    blob_info.path.clone().split("/").into_iter().map( |name| {
                        html!{
                            <>
                                <span class="mx-1 text-lg">{"/"}</span>
                                <span class={
                                    if name == blob_info.name {
                                        "text-neutral-300 font-bold"
                                    }else{
                                        ""
                                    }
                                }>{name}</span>
                            </>
                        }
                    }).collect::<Html>()
                }
            </div>

            <div class="flex gap-3 items-center w-full px-2 pb-2">
                <div class=" flex items-center gap-3 text-base">
                    <i class={format!("devicon-{}-plain colored",blob_info.language)}></i>
                    <a class="font-black text-gray-300 text-xl !underline">{
                        if start_line != "1" {
                            if end_line != "0" {
                                format!("{}:{}-{}", blob_info.path.clone(), start_line, end_line)
                            } else if start_line != "0" {
                                format!("{}:{}", blob_info.path.clone(), start_line)
                            } else {
                                blob_info.path.clone()
                            }
                        } else {
                            blob_info.path.clone()
                        }
                    }</a>
                </div>
            </div>

            <div class="outline outline-neutral-600 p-2 rounded-lg w-full">
                <div class="flex justify-between">
                    <div class="flex mb-4">
                        <span class="size-11 text-2xl font-black bg-lime-700 flex rounded-full justify-center items-center text-gray-300">{ 
                            commit_info.author_name.chars().next().map(|c| c.to_uppercase().to_string())
                        }</span>
                        <div class="flex flex-col ml-3">
                            <div class="flex items-center">
                                <span class="font-black text-gray-300">{ commit_info.title.clone() }</span>
                                <GitBranch onclick={Callback::from(|_|{})} class="" />
                                <a class="text-sm mx-1 !underline">{ ref_ }</a>
                                <CopyIcon onclick={Callback::from(|_|{})} class="" />
                            </div>
                            <span class="text-sm">
                                <span class="font-bold">{commit_info.author_name.clone()}</span>
                                {" "}
                                <span class="">{ format!("authored {} ago ",time_ago(commit_info.author_date.clone())) }</span>
                            </span>
                        </div>
                    </div>
                </div>
                <div class="flex font-white text-stone-300">
                    <span>{ title }</span>
                    
                </div>
                // <a class="ml-2 font-bold text-gray-400 !underline">{blob_info.path.clone()}</a>
            </div>

            <div class="overflow-hidden w-full h-full rounded-lg outline outline-neutral-600">
                <div class="p-2 flex items-center gap-2 text-base">
                    <i class={format!("devicon-{}-plain colored",blob_info.language)}></i>
                    <span class="font-black text-gray-300">{blob_info.name.clone()}</span>
                    <CopyIcon onclick={Callback::from(|_|{})} class="" />
                    <span class="!text-xs">{ 
                        format!("{:.2} KiB", kib)
                     }</span>
                </div>
                <div id="monaco-container" class="w-full h-full rounded-lg outline outline-neutral-600"></div>
            </div>
        </div>
    }
}
