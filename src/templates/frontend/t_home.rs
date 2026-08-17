
use topcoat::{
	Result,
	view::{component, view},
	context::{Cx, app_context},
};
use sqlx::MySqlPool;
#[component]
async fn hello(name: &str)->Result{
	view! {<h1> "Hello, " (name) "!"</h1>}
}


#[component]
pub async fn homepage()->Result{
	view! {
		<!DOCTYPE html>
		<html>
			<head>
				<title>"rust"</title>
			</head>
			<body>
				<h2>hello(name:"My Name")</h2>
			</body>
		</html>
	}
}

#[component]
pub async fn MyDatabaseComponent(cx: &Cx)->Result{
	let db = app_context::<MySqlPool>(cx);

	let _row: (i32,) = sqlx::query_as("SELECT 1")
							.fetch_one(db)
							.await?;
    view!{
		<div>"Database connected successfully"</div>
	}	
}
#[derive(sqlx::FromRow)]
struct ProductItem{
	id: i32,
	product_name:String,
	product_quantity: i32,
}
#[component]
pub async fn ProductList(cx: &Cx)->Result{
	let db = app_context::<MySqlPool>(cx);
	let products: Vec<ProductItem> = sqlx::query_as("SELECT id, product_name, product_quantity FROM product")
	                                          .fetch_all(db)
											  .await?;
    //can't use map closure in the following view! because it will call async code											  
	view!{
		<table>
			<tr><th>"ID"</th><th>"Product"</th><th>"Quantity"</th></tr>
			for item in products {
				<tr><td>(item.id)</td><td>(item.product_name)</td><td>(item.product_quantity)</td></tr>
			}
		</table>
	}		
}