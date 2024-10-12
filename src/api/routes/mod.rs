use super::handlers;
use sea_orm::DatabaseConnection;
use warp::{Filter, Rejection, Reply};

pub fn setup_routes(
    db: DatabaseConnection,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let api = warp::path("api");

    let health_check = api
        .and(warp::path("healthchecker"))
        .and(warp::get())
        .and_then(handlers::health_checker_handler);

    let create_post = api
        .and(warp::path("posts"))
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(handlers::create_post);

    health_check
        .or(create_post)
        .with(warp::cors().allow_any_origin().allow_headers(vec!["content-type"]))
        .with(warp::log("api"))
}

fn with_db(db: DatabaseConnection) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
