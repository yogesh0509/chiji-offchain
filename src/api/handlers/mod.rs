use warp::{Rejection, Reply, reject::Reject};
use sea_orm::*;
use serde::{Serialize, Deserialize};
use ::entity::{post, post::Entity as Post};

#[derive(Serialize)]
struct GenericResponse {
    status: String,
    message: String,
}

#[derive(Debug)]
pub struct DatabaseError(pub DbErr);

// Implement `warp::reject::Reject` for your new type
impl Reject for DatabaseError {}

pub async fn get_post(
    db: DatabaseConnection,
) -> Result<impl Reply, Rejection> {

    let posts = Post::find().all(&db).await;
    match posts {
        Ok(posts) => Ok(warp::reply::json(&posts)),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))), 
    }
}

pub async fn create_post(
    db: DatabaseConnection,
    form_data: CreatePostForm,
) -> Result<impl Reply, Rejection> {
    let post = post::ActiveModel {
        title: Set(form_data.title.to_owned()),
        text: Set(form_data.text.to_owned()),
        ..Default::default()
    };

    // Perform the insert
    let inserted_post = Post::insert(post).exec_with_returning(&db).await;
    match inserted_post {
        Ok(post) => Ok(warp::reply::json(&post)),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))), // Wrap the DbErr into DatabaseError
    }
}

// Struct to deserialize incoming data
#[derive(Deserialize)]
pub struct CreatePostForm {
    pub title: String,
    pub text: String,
}

pub async fn health_checker_handler() -> Result<impl Reply, Rejection> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(warp::reply::json(response_json))
}
