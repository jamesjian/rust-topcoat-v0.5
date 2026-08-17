use topcoat::router::{Router, RouterBuilderDiscoverExt};

pub fn router(state: crate::AppState)->Router{
	Router::builder().discover().app_context(state.db).build()
}