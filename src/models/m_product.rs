use topcoat::{
	Result,
};

#[derive(sqlx::FromRow)]
pub struct ProductItem{
	pub id: i32,
	pub product_name:String,
	pub product_quantity: i32,
}
pub async fn get_product_list()->Result<Vec<ProductItem>, sqlx::Error>{
	let db = crate::DB_POOL.get().expect("Database pool not initialized");
	let products: Vec<ProductItem> = sqlx::query_as("SELECT id, product_name, product_quantity FROM product")
	                                          .fetch_all(db)
											  .await?;
	Ok(products)	
}
pub async fn get_product_detail(id: i32)->Result<Option<ProductItem>, sqlx::Error>{
	let db = crate::DB_POOL.get().expect("Database pool not initialized");
	let product: ProductItem = sqlx::query_as("SELECT id, product_name, product_quantity FROM product WHERE id= ? ")
												.bind(id)
	                                          .fetch_optional(db)   //instead of fetch_one() which return ProductItem instead of Option<ProductItem>
											  .await?.expect("No product");
    Ok(Some(product))													
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[tokio::test]
	async fn test_get_product_detail(){
		//must let _ because init_test_db().await return Result which must be handled
		let _  = crate::init_test_db().await;
		
		let result = get_product_detail(1).await;
		assert!(result.is_ok(), "db return error: {:?}", result.err());
		let product_option = result.unwrap();
		match product_option {
			Some(product)=>{
				assert_eq!(product.id,1);
				assert!(!product.product_name.is_empty(), "product name should not be empty");
			}
			None=>{
				println!("Query succeeded, but no product found");
			}
		}
	}
}
