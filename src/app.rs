use topcoat::router::{Router, RouterBuilderDiscoverExt};

pub fn router()->Router{
	Router::builder().discover().build()
}