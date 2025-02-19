use chrono::{DateTime, Utc};

pub mod gitlab;

// use yew::prelude::*;
// use std::rc::Rc;
// use reqwest::Client;
// use graphql_client::{GraphQLQuery, Response as GraphqlResponse};
// use wasm_bindgen_futures::spawn_local;

use gloo_console::log;
use gloo_net::http::{Request, Response};
use graphql_client::QueryBody;
use serde::Serialize;

use crate::container::home::Route;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ApiClient {
    pub base_url: String,
    pub ci_job_token: String,
    pub group: String,
}
// Define a wrapper enum for Response

impl ApiClient {
    pub fn new(base_url: String, ci_job_token: String, group: String) -> Self {
        ApiClient {
            base_url,
            ci_job_token,
            group
        }
    }

    // pub async fn get(self, path: &str) -> Response {
    //     Request::get(&format!("{}{}", self.base_url, path))
    //         .header("PRIVATE-TOKEN", &self.ci_job_token)
    //         .send()
    //         .await
    //         .unwrap()
    // }

    pub async fn post<T>(self, path: &str, body: &QueryBody<T>) -> Response
    where
        T: Serialize,
    {
        log!(format!("Test call: {} {}", self.base_url, path));
        let data = Request::post(&format!("{}{}", self.base_url, path))
            .header("PRIVATE-TOKEN", &self.ci_job_token)
            .json(&body)
            .unwrap()
            .send()
            .await;

        match data {
            Ok(req_data) => req_data,
            Err(err) => {
                log!(format!("Test call 2 pannic: {} {}", self.base_url, path));
                log!(format!("{}", err));
                panic!()
            }
        }
    }
}

pub fn time_ago(datetime_str: String) -> String {
    if datetime_str.is_empty(){
        return String::new();
    }

    let datetime = DateTime::parse_from_str(&datetime_str, "%+").expect("Invalid date format");

    let now = Utc::now();
    let duration = now.signed_duration_since(datetime);

    if duration.num_seconds() < 60 {
        format!("{} second{}", duration.num_seconds(), if duration.num_seconds() == 1 { "" } else { "s" })
    } else if duration.num_minutes() < 60 {
        format!("{} minute{}", duration.num_minutes(), if duration.num_minutes() == 1 { "" } else { "s" })
    } else if duration.num_hours() < 24 {
        format!("{} hour{}", duration.num_hours(), if duration.num_hours() == 1 { "" } else { "s" })
    } else if duration.num_days() < 30 {
        format!("{} day{}", duration.num_days(), if duration.num_days() == 1 { "" } else { "s" })
    } else if duration.num_days() < 365 {
        format!("{} month{}", duration.num_days() / 30, if duration.num_days() / 30 == 1 { "" } else { "s" })
    } else {
        format!("{} year{}", duration.num_days() / 365, if duration.num_days() / 365 == 1 { "" } else { "s" })
    }
}

pub fn get_route_full_path(route: Route ) -> String {
    match route {
        Route::Reports { id } => id,
        Route::Editor { project_path, with_ref: _, path: _, start_line: _, end_line: _, title: _ } => project_path,
        Route::Detail { project_path, vul_id: _ } => project_path,
        _ => String::from("No ID found"),
    }
}

// #[hook]
// fn use_graphql_data<T, Q>(query: Q) -> Option<Rc<T>>
// where
//     T: 'static,
//     Q: GraphQLQuery + 'static,
//     Q::Variables: Clone,
// {
//     let data = use_memo(|_| None, ());

//     // use_effect_with(query, move |_| {
//     //     let query_variables = Q::Variables::default(); // Use default variables or customize as needed
//     //     let query_body = Q::build_query(query_variables.clone());

//     //     spawn_local(async move {
//     //         let client = Client::new();
//     //         let res = client
//     //             .post("https://your-graphql-endpoint.com/graphql")
//     //             .json(&query_body)
//     //             .send()
//     //             .await
//     //             .ok()
//     //             .and_then(|res| res.json::<GraphqlResponse<T>>().await.ok());

//     //         if let Some(response) = res {
//     //             data.set(Some(Rc::new(response.data?)));
//     //         }
//     //     });
//     // });

//     (*data).clone()
// }
