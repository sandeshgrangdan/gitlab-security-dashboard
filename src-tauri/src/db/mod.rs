use std::fs;
use std::path::Path;

use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

pub mod repo;

const DB_PATH: &str = "/.config/gitlab-security/database.sqlite";

fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    fs::File::create(db_path).unwrap();
}

fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

pub fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + DB_PATH
}

pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
}

pub type Db = Pool<Sqlite>;

#[derive(Debug,Clone)]
pub struct Database {
}

impl Database {
    pub fn new() -> Self {
        init();
        Self {
        }
    }

    pub async fn setup_db(&self) -> Db {
   
        let path = get_db_path();
     
        Sqlite::create_database(
            format!(
                "sqlite:{}",
                &path
            )
            .as_str(),
        )
        .await
        .expect("failed to create database");
     
        let db = SqlitePoolOptions::new()
            .connect(&path)
            .await
            .unwrap();

        println!("Start Running migration.");
        sqlx::migrate!("./migrations").run(&db).await.unwrap();
        println!("Migration completed.");

        db
    }
}
