use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use futures::TryStreamExt;

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub full_path: String,
    pub id: String,
    pub name: String,
}

#[tauri::command]
pub async fn add_repo(state: tauri::State<'_, AppState>, id: &str, path: &str, name: &str ) -> Result<(), String> {
    println!("Init adding repo");

    // println!("{}, {}, {}", id, path, name);
    
    sqlx::query("INSERT INTO repo (id, full_path, name) VALUES (?1, ?2, ?3)")
    .bind(id)
    .bind(path)
    .bind(name)
    .execute(&state.db)
    .await
    .map_err(|e| {
        println!("{:#?}",e);
        format!("Error saving repo: {}", e)
    }
    )?;

    println!("Repo added successfully.");
    Ok(())
}


#[tauri::command]
pub async fn get_repos(state: tauri::State<'_, AppState>) -> Result<Vec<Project>, String> {
    // println!("Get repos");

    let repos: Vec<Project> = sqlx::query_as::<_, Project>("SELECT * FROM repo")
        .fetch(&state.db)
        .try_collect()
        .await
        .map_err(|e| format!("Failed to get repos {}", e))?;

    // println!("{:#?}",repos);
 
    Ok(repos)
}

#[tauri::command]
pub async fn delete_repo(state: tauri::State<'_, AppState>, id: &str) -> Result<(), String> {
    println!("Init deleting repo with id: {}", id);

    // Execute the delete query
    sqlx::query("DELETE FROM repo WHERE id = ?1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            println!("{:#?}", e);
            format!("Error deleting repo: {}", e)
        })?;

    // println!("Repo deleted successfully.");
    Ok(())
}