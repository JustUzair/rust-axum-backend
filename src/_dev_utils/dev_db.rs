use std::{error::Error, fs, path::PathBuf, time::Duration};

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::info;

type Db = Pool<Postgres>;

// NOTE: Hardcode to prevent deployed system db update.
const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://app_user:dev_only_pwd@localhost/app_db";

const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "sql/init_dev";

const DEMO_PWD: &str = "welcome";

async fn new_db_pool(db_conn_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_conn_url)
        .await
}

async fn pexec(db: &Db, file_path: &str) -> Result<(), sqlx::Error> {
    info!("{:<12} - pexec: {file_path:?}", "FOR-DEV-ONLY");
    let content = fs::read_to_string(file_path)?;
    let sqls: Vec<&str> = content.split(";").collect();
    for sql in sqls {
        sqlx::query(sql).execute(db).await?;
    }
    Ok(())
}

pub async fn init_dev_db() -> Result<(), Box<dyn Error>> {
    info!("{:<12} - new_db_pool()", "FOR-DEV-ONLY");
    let current_dir = std::env::current_dir().unwrap();
    let sql_dir = current_dir.join(SQL_DIR);
    // Create db with our postgres user
    {
        // block to confine the variables to connect and execute within a block
        let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL).await?;
        pexec(&root_db, sql_recreate_db_file.to_str().unwrap()).await?;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    paths.sort();
    let app_db = new_db_pool(PG_DEV_APP_URL).await?;
    for path in paths {
        if let Some(path) = path.to_str() {
            let path = path.replace("\\", "/");
            if path.ends_with(".sql") && !path.contains(SQL_RECREATE_DB_FILE_NAME) {
                pexec(&app_db, &path).await?;
            }
        }
    }
    Ok(())
}
