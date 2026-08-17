use topcoat::{
	Result,
    router::{ page, path_param, },
	view::{view},
	context::Cx,
	
};

use crate::templates::frontend::t_home;

#[page("/")]
async fn home()->Result{
	view! {
		t_home::homepage()
	}
}

#[page("/products")]
async fn products()->Result{
	view! {
		t_home::product_list()
	}
}
//don't use older macro path_param!. 
#[path_param(error = bad_request("Product ID must be a number!"))]

pub struct ProductId(pub u32);

#[page("/productdetail/{product_id}")]
async fn productdetail(cx: &Cx)->Result{
	let product_id = path_param::<ProductId>(cx)?;
	let id_as_number:i32 = *product_id as i32;
	//must use id:id_as_number , otherwise report "expected view node"
	view! {
		t_home::product_detail(id:id_as_number)
	}
}