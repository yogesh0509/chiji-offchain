use super::handlers;
use sea_orm::DatabaseConnection;
use warp::filters::BoxedFilter;
use warp::{path, Filter};

fn path_prefix() -> BoxedFilter<()> {
    path!("api" / ..).boxed()
}

fn with_db(
    db: DatabaseConnection,
) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn health_check() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("healthchecker"))
        .and(warp::path::end())
        .boxed()
}

pub fn get_user_spaces() -> BoxedFilter<(String,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("activity"))
        .and(warp::path("user"))
        .and(warp::path("spaces"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .boxed()
}

pub fn get_user_proposals() -> BoxedFilter<(String,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("activity"))
        .and(warp::path("user"))
        .and(warp::path("proposals"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .boxed()
}

pub fn create_space() -> BoxedFilter<()> {
    warp::post()
        .and(path_prefix())
        .and(warp::path("spaces"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .boxed()
}

pub fn join_space() -> BoxedFilter<(String, i32)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("spaces"))
        .and(warp::path("join"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .boxed()
}

pub fn get_all_spaces() -> BoxedFilter<()> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("spaces"))
        .and(warp::path("all"))
        .and(warp::path::end())
        .boxed()
}

pub fn get_space_by_id() -> BoxedFilter<(i32,)> {
    warp::get()
        .and(path_prefix())
        .and(warp::path("spaces"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .boxed()
}

pub fn upload_space_logo() -> BoxedFilter<()> {
    warp::post()
        .and(path_prefix())
        .and(warp::path("upload"))
        .and(warp::path("logo"))
        .and(warp::path::end())
        .boxed()
}

pub fn setup_routes(
    db: DatabaseConnection,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    health_check()
        .and_then(handlers::health_checker_handler)
        .or(get_user_spaces()
            .and(with_db(db.clone()))
            .and_then(handlers::get_user_spaces))
        .or(get_user_proposals()
            .and(with_db(db.clone()))
            .and_then(handlers::get_user_proposals))
        .or(create_space()
            .and(with_db(db.clone()))
            .and(warp::body::json())
            .and_then(handlers::create_space))
        .or(join_space()
            .and(with_db(db.clone()))
            .and_then(handlers::join_space))
        .or(get_all_spaces()
            .and(with_db(db.clone()))
            .and_then(handlers::get_all_spaces))
        .or(get_space_by_id()
            .and(with_db(db.clone()))
            .and_then(handlers::get_space_by_id))
        .or(upload_space_logo()
            .and(warp::body::json())
            .and_then(handlers::upload_space_logo))
        .recover(handlers::handle_rejection)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["content-type"])
                .allow_methods(vec!["GET", "POST"]),
        )
        .with(warp::log("api"))
}
