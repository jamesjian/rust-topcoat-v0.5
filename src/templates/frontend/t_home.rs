
use topcoat::{
	Result,
	view::{component, view},

};

use crate::models::m_product;
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
pub async fn product_list()->Result{
	let products: Vec<m_product::ProductItem> = m_product::get_product_list().await?;	
	view!{
		<table>
			<tr><th>"ID"</th><th>"Product"</th><th>"Quantity"</th></tr>
			for item in products {
				<tr><td>(item.id)</td><td>(item.product_name)</td><td>(item.product_quantity)</td></tr>
			}
		</table>
	}		
}
#[component]
pub async fn product_detail(id: i32)->Result{
	let product: Option<m_product::ProductItem> = m_product::get_product_detail(id).await?;			
	match product {
		None => { view! {<h2>"Product not found"</h2>} }
		Some(product) =>
			view!{
				<table>
					<tr><th>"ID"</th><th>"Product"</th><th>"Quantity"</th></tr>
						<tr><td>(product.id)</td><td>(product.product_name)</td><td>(product.product_quantity)</td></tr>
				</table>
			},		
	}
}