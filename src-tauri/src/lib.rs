use std::env;

use sqlx::{Pool, Sqlite};
use tauri::Manager;

pub mod db;

#[tauri::command]
fn get_env() -> (String, String, String) {
    let gitlab_base_url = env::var("GITLAB_BASE_URL").unwrap_or_else(|_| String::from("https://gitlab.com"));
    let gitlab_token = env::var("GITLAB_TOKEN").unwrap_or_else(|_| env::var("CI_JOB_TOKEN").unwrap_or_else(|_| String::new()));
    let group = env::var("GITLAB_GROUP").unwrap_or_else(|_| String::from("innovate-tech"));

    println!("From get_envdd: {}, {}", gitlab_base_url, gitlab_token);
    (gitlab_base_url, gitlab_token, group)
}

type Db = Pool<Sqlite>;
 
pub struct AppState {
    pub db: Db,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
            .build()
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {

            tauri::async_runtime::block_on(async move {

                let init_db = db::Database::new();

                let db = init_db.setup_db().await;

                app.manage(AppState { db });
            });
            Ok(())
        })
        .invoke_handler(
            tauri::generate_handler![
                get_env, 
                db::repo::add_repo,
                db::repo::get_repos,
                db::repo::delete_repo
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
