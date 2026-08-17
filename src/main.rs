mod app;
mod handlers;
mod models;
mod templates;
use sqlx::mysql::{MySqlPoolOptions, MySqlPool};
use std::sync::OnceLock;

pub static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

#[tokio::main]
async fn main()->Result<(),sqlx::Error>{	
	dotenvy::dotenv().ok();
	let database_url = std::env::var("DATABASE_URL").unwrap();
	//let pool = PgPoolOptions::new()
	let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

	DB_POOL.set(pool).expect("Failed to initialized globle DB pool");

	
	let router = app::router();
	topcoat::start(router).await.unwrap();
	
	Ok(())
}

#[cfg(test)]
pub async fn init_test_db() -> Result<(), sqlx::Error>{
	dotenvy::dotenv().ok();
	let database_url = std::env::var("DATABASE_URL").unwrap();
	let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

	let _ = DB_POOL.set(pool);
	Ok(())
}