mod app;
mod handlers;
mod templates;

#[tokio::main]
async fn main(){
	let router = app::router();
	topcoat::start(router).await.unwrap();
}

