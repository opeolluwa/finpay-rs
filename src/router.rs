use crate::users::users_router;
use axum::Router;

pub fn load_routes() -> Router {
    let router = Router::new();

    router.merge(users_router())
    
    
}
