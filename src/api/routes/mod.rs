use super::handlers;
use handlers::handle_rejection;
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

    let get_user_spaces = api
        .and(warp::path("activity"))
        .and(warp::path("user"))
        .and(warp::path("spaces"))
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::get_user_spaces)
        .recover(handle_rejection);

    let get_user_proposals = api
        .and(warp::path("activity"))
        .and(warp::path("user"))
        .and(warp::path("proposals"))
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::get_user_proposals)
        .recover(handle_rejection);

    let create_space = api
        .and(warp::path("create"))
        .and(warp::path("space"))
        .and(warp::post())
        .and(with_db(db.clone()))
        .and(warp::body::json())
        .and_then(handlers::create_space)
        .recover(handle_rejection);

    let join_space = api
        .and(warp::path("join"))
        .and(warp::path("space"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<i32>())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::join_space)
        .recover(handle_rejection);

    let get_all_spaces = api
        .and(warp::path("spaces"))
        .and(warp::path("all"))
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::get_all_spaces)
        .recover(handle_rejection);

    let get_space_by_id = api
        .and(warp::path("spaces"))
        .and(warp::path::param::<i32>())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handlers::get_space_by_id)
        .recover(handle_rejection);

    let upload_s3 = api
        .and(warp::path("upload"))
        .and(warp::path("logo"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::upload_space_logo)
        .recover(handle_rejection);

    health_check
        .or(get_user_spaces)
        .or(get_user_proposals)
        .or(create_space)
        .or(join_space)
        .or(get_all_spaces)
        .or(get_space_by_id)
        .or(upload_s3)
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_headers(vec!["content-type"]),
        )
        .with(warp::log("api"))
}

fn with_db(
    db: DatabaseConnection,
) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
