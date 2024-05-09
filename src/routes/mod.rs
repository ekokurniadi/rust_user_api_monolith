mod user_router;

use rocket::Route;
use self::user_router::user_routes;

pub fn routes()-> Vec<Route>{
    let mut routers =Vec::new();
    routers.extend(user_routes());
    routers
}