use topcoat::{
	Result,
	router::{page,},
	view::{view},

};
use crate::templates::frontend::home::homepage;

#[page("/")]
async fn home()->Result{
	view! {
		homepage()
	}
}

