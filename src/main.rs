mod app;
mod handlers;
mod templates;
use sqlx::mysql::{MySqlPoolOptions, MySqlPool};
//use std::env;

#[derive(Clone)]
pub struct AppState {
	pub db: MySqlPool
}

#[tokio::main]
async fn main()->Result<(),sqlx::Error>{	
	dotenvy::dotenv().ok();
	let database_url = std::env::var("DATABASE_URL").unwrap();
	println!("{}",database_url);
	//let pool = PgPoolOptions::new()
	let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

	let state = AppState{ db:pool };
	
	let router = app::router(state);
	topcoat::start(router).await.unwrap();
	
	Ok(())
}

