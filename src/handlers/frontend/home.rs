use topcoat::{
	Result,
	router::{page,},
	view::{view},
	
};

use crate::templates::frontend::t_home;

#[page("/")]
async fn home()->Result{
	view! {
		t_home::homepage()
	}
}

#[page("/db")]
async fn db()->Result{
	view!{
		t_home::MyDatabaseComponent()
	}
}
#[page("/products")]
async fn products()->Result{
	view! {
		t_home::ProductList()
	}
}