pub use axum::*;
use http::StatusCode;
use routing::get;

pub struct Provider {
    pub name: &'static str,
    pub routes: RouteList,
}

pub struct MethodRouter {
    pub get: (StatusCode, &'static str),
    pub post: (StatusCode, &'static str),
    pub put: (StatusCode, &'static str),
    pub patch: (StatusCode, &'static str),
    pub delete: (StatusCode, &'static str),
}

pub struct RouteSchema {
    pub path: &'static str,
    pub handler: MethodRouter,
}

pub enum Route {
    Router(Router),
    RouteSchema(RouteSchema),
}

pub type RouteList = Vec<Route>;

pub fn router(routes: &RouteList) -> Router {
    let mut router = Router::new();

    for route in routes {
        match route {
            Route::Router(route) => router = router.merge(route.clone()),
            Route::RouteSchema(route) => {
                router = router.merge(Router::new().route(route.path, get(route.handler.get)));
            }
        }
    }

    router
}
