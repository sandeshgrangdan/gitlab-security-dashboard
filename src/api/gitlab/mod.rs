use crate::api::ApiClient;

use gloo_net::http::Response;
use graphql_client::GraphQLQuery;

// pub const API_VERSION: &str = "/api/v4";
pub const GRAPHQL_PATH: &str = "/api/graphql";

// const PROJECT_PATH: &str = "/projects?owned=true";

type Time = String;
type VulnerabilitiesScannerID = String;
type ClustersAgentID = String;
type BigInt = String;
type VulnerabilityID = String;
type ISO8601Date = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/projects.graphql",
    response_derives = "Debug"
)]
pub struct GetProjects;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/group_vul_history.graphql",
    response_derives = "Debug"
)]
pub struct GroupVulnerabilityHistory;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/vulnerabilitie.graphql",
    response_derives = "Debug"
)]
pub struct GroupVulnerabilities;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/getvuls.graphql",
    response_derives = "Debug"
)]
pub struct GetVuls;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/vulcount.graphql",
    response_derives = "Debug"
)]
pub struct VulnerabilitySeveritiesCount;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/raw_text_blob.graphql",
    response_derives = "Debug"
)]
pub struct GetVulnerabilityBlobInfo;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/blob_info.graphql",
    response_derives = "Debug"
)]
pub struct GetBlobInfo;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/path_last_commit.graphql",
    response_derives = "Debug"
)]
pub struct PathLastCommit;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "src/api/gitlab/graphql/schema.json",
    query_path = "src/api/gitlab/graphql/getvul.graphql",
    response_derives = "Debug"
)]
pub struct GetVul;

impl ApiClient {
    // pub async fn projects(self) -> Response {
    //     self.get(PROJECT_PATH).await
    // }

    pub async fn get_vul_history(self,full_path: String, start_date: String, end_date: String) -> Response {
        let variables = group_vulnerability_history::Variables {
            full_path,
            start_date,
            end_date,
        };

        let query = GroupVulnerabilityHistory::build_query(variables);
        self.post::<group_vulnerability_history::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    pub async fn get_path_last_commit(self,project_path: String, path: Option<String>, ref_: String) -> Response {
        let variables = path_last_commit::Variables {
            project_path,
            ref_,
            ref_type: None,
            path,
        };

        let query = PathLastCommit::build_query(variables);
        self.post::<path_last_commit::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    pub async fn get_projects(self, after: Option<String>) -> Response {
        let variables = get_projects::Variables{
            after
        };

        let projects_query = GetProjects::build_query(variables);
        self.post::<get_projects::Variables>(GRAPHQL_PATH, &projects_query)
            .await
    }

    pub async fn get_vuls(self, full_path: String, after :Option<String>, before: Option<String>, first: Option<i64> ) -> Response {
        let variables = get_vuls::Variables {
            full_path: full_path,
            after,
            before,
            first
        };
        let query = GetVuls::build_query(variables);
        self.post::<get_vuls::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    pub async fn get_vul(self, id: String) -> Response {
        let variables = get_vul::Variables {
            id
        };
        let query = GetVul::build_query(variables);
        self.post::<get_vul::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    pub async fn get_vulnerability_raw_blob_info(self,project_path: String, file_path: String, ref_: String) -> Response {
        let variables = get_vulnerability_blob_info::Variables {
            project_path,
            file_path,
            ref_,
        };
        let query = GetVulnerabilityBlobInfo::build_query(variables);
        self.post::<get_vulnerability_blob_info::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    pub async fn get_blob_info(self,project_path: String, file_path: Vec<String>, ref_: String) -> Response {
        let variables = get_blob_info::Variables {
            project_path,
            ref_,
            ref_type: None,
            should_fetch_raw_text: true,
            file_path,
        };

        let query = GetBlobInfo::build_query(variables);
        self.post::<get_blob_info::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    pub async fn get_vul_count(self, full_path: String) -> Response {
        let variables = vulnerability_severities_count::Variables {
            full_path: full_path,
            is_group: false,
            is_project: true,
            is_instance: false,
            capped: true,            
            state: Some(vec![
                vulnerability_severities_count::VulnerabilityState::DETECTED,
                vulnerability_severities_count::VulnerabilityState::CONFIRMED
            ]),
            dismissal_reason: Some(vec![]),
            has_resolution: Some(false),
            report_type: Some(vec![
                    vulnerability_severities_count::VulnerabilityReportType::API_FUZZING,
                    vulnerability_severities_count::VulnerabilityReportType::CONTAINER_SCANNING,
                    vulnerability_severities_count::VulnerabilityReportType::COVERAGE_FUZZING,
                    vulnerability_severities_count::VulnerabilityReportType::DAST,
                    vulnerability_severities_count::VulnerabilityReportType::DEPENDENCY_SCANNING,
                    vulnerability_severities_count::VulnerabilityReportType::SAST,
                    vulnerability_severities_count::VulnerabilityReportType::SECRET_DETECTION,
                    vulnerability_severities_count::VulnerabilityReportType::GENERIC,
            ]),
        };
        let query = VulnerabilitySeveritiesCount::build_query(variables);
        self.post::<vulnerability_severities_count::Variables>(GRAPHQL_PATH, &query)
            .await
    }

    // pub async fn get_group_vulnerabilitie(self) -> Response {
    //     let group_vulnerabilities_vars = group_vulnerabilities::Variables {
    //         after: None,
    //         before: None,
    //         last: None,
    //         project_id: None,
    //         has_issues: None,
    //         has_merge_request: None,
    //         severity: None,
    //         has_remediations: None,
    //         owasp_top_ten: None,
    //         scanner: None,
    //         scanner_id: None,
    //         has_resolution: None,
    //         first: Some(20),
    //         vet_enabled: Some(true),
    //         full_path: String::from("innovate-tech"),
    //         sort: Some(group_vulnerabilities::VulnerabilitySort::detected_asc),
    //         state: Some(vec![
    //             group_vulnerabilities::VulnerabilityState::DETECTED,
    //             group_vulnerabilities::VulnerabilityState::CONFIRMED,
    //         ]),
    //         dismissal_reason: Some(vec![]),
    //         report_type: Some(vec![
    //             group_vulnerabilities::VulnerabilityReportType::API_FUZZING,
    //             group_vulnerabilities::VulnerabilityReportType::COVERAGE_FUZZING,
    //             group_vulnerabilities::VulnerabilityReportType::DAST,
    //             group_vulnerabilities::VulnerabilityReportType::DEPENDENCY_SCANNING,
    //             group_vulnerabilities::VulnerabilityReportType::SAST,
    //             group_vulnerabilities::VulnerabilityReportType::SECRET_DETECTION,
    //             group_vulnerabilities::VulnerabilityReportType::GENERIC,
    //         ]),
    //         cluster_agent_id: None,
    //     };

    //     let vul_query = GroupVulnerabilities::build_query(group_vulnerabilities_vars);
    //     self.post::<group_vulnerabilities::Variables>(GRAPHQL_PATH, &vul_query)
    //         .await
    // }
}
