// use ::entity::post;
use ::entity::prelude::{User, Space, Proposal};
use ::entity::{user, space, proposal};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use warp::{reject::Reject, Rejection, Reply};

pub mod error;
pub use error::handle_rejection;

#[derive(Serialize)]
struct GenericResponse {
    status: String,
    message: String,
}

#[derive(Debug)]
pub struct DatabaseError(pub DbErr);

// Implement `warp::reject::Reject` for your new type
impl Reject for DatabaseError {}

// pub async fn get_post(db: DatabaseConnection) -> Result<impl Reply, Rejection> {
//     let posts = Post::find().all(&db).await;
//     match posts {
//         Ok(posts) => Ok(warp::reply::json(&posts)),
//         Err(err) => Err(warp::reject::custom(DatabaseError(err))),
//     }
// }

// pub async fn create_post(
//     db: DatabaseConnection,
//     form_data: CreatePostForm,
// ) -> Result<impl Reply, Rejection> {
//     let post = post::ActiveModel {
//         title: Set(form_data.title.to_owned()),
//         text: Set(form_data.text.to_owned()),
//         ..Default::default()
//     };

//     // Perform the insert
//     let inserted_post = Post::insert(post).exec_with_returning(&db).await;
//     match inserted_post {
//         Ok(post) => Ok(warp::reply::json(&post)),
//         Err(err) => Err(warp::reject::custom(DatabaseError(err))), // Wrap the DbErr into DatabaseError
//     }
// }

pub async fn get_all_proposals(db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    match Proposal::find().all(&db).await {
        Ok(proposals) => {
            println!("{:?}", proposals);
            if proposals.is_empty() {
                Ok(warp::reply::json(&Vec::<proposal::Model>::new()))
            } else {
                Ok(warp::reply::json(&proposals))
            }
        }
        Err(err) => {
            eprintln!("Database error when fetching all proposals: {:?}", err);
            Err(warp::reject::custom(DatabaseError(err)))
        }
    }
}

pub async fn get_proposal_by_id(id: i32, db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    match Proposal::find_by_id(id).one(&db).await {
        Ok(Some(proposal)) => Ok(warp::reply::json(&proposal)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Err(warp::reject::custom(DatabaseError(err)))
        }
    }
}

pub async fn create_proposal(
    db: DatabaseConnection,
    data: proposal::Model,
) -> Result<impl Reply, Rejection> {

    println!("{:?}", data);

    let row = proposal::ActiveModel {
        id: Set(data.id.to_owned()),
        title: Set(data.title.to_owned()),
        description: Set(data.description.to_owned()),
        space_id: Set(data.space_id.to_owned()),
        creator_id: Set(data.creator_id.to_owned()),
        ..Default::default()
    };

    println!("{:?}", row);

    let inserted_row = Proposal::insert(row).exec_with_returning(&db).await;

    println!("{:?}", inserted_row);

    match inserted_row {
        Ok(row) => Ok(warp::reply::json(&row)),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))),
    }
}

// Space handlers
pub async fn get_all_spaces(db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    match Space::find().all(&db).await {
        Ok(spaces) => {
            println!("{:?}", spaces);
            if spaces.is_empty() {
                Ok(warp::reply::json(&Vec::<space::Model>::new()))
            } else {
                Ok(warp::reply::json(&spaces))
            }
        }
        Err(err) => {
            eprintln!("Database error when fetching all spaces: {:?}", err);
            Err(warp::reject::custom(DatabaseError(err)))
        }
    }
}

pub async fn get_space_by_id(id: i32, db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    match Space::find_by_id(id).one(&db).await {
        Ok(Some(space)) => Ok(warp::reply::json(&space)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Err(warp::reject::custom(DatabaseError(err)))
        }
    }
}

pub async fn create_space(
    db: DatabaseConnection,
    data: space::Model,
) -> Result<impl Reply, Rejection> {
    println!("{:?}", data);
    let row = space::ActiveModel {
        id: sea_orm::ActiveValue::Set(data.id.to_owned()),
        protocol: sea_orm::ActiveValue::Set(data.protocol.to_owned()),
        title: sea_orm::ActiveValue::Set(data.title.to_owned()),
        creator: sea_orm::ActiveValue::Set(data.creator.to_owned()),
        description: sea_orm::ActiveValue::Set(data.description.to_owned()),
        ..Default::default()
    };
    println!("{:?}", row);
    let inserted_row = Space::insert(row).exec_with_returning(&db).await;
    println!("{:?}", inserted_row);
    match inserted_row {
        Ok(row) => Ok(warp::reply::json(&row)),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))),
    }
}

// User handlers
pub async fn get_all_users(db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    match User::find().all(&db).await {
        Ok(users) => {
            println!("{:?}", users);
            if users.is_empty() {
                Ok(warp::reply::json(&Vec::<user::Model>::new()))
            } else {
                Ok(warp::reply::json(&users))
            }
        }
        Err(err) => {
            eprintln!("Database error when fetching all users: {:?}", err);
            Err(warp::reject::custom(DatabaseError(err)))
        }
    }
}

pub async fn get_user_by_id(id: i32, db: DatabaseConnection) -> Result<impl Reply, Rejection> {
    match User::find_by_id(id).one(&db).await {
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Err(warp::reject::custom(DatabaseError(err)))
        }
    }
}

pub async fn create_user(
    db: DatabaseConnection,
    data: user::Model,
) -> Result<impl Reply, Rejection> {
    println!("{:?}", data);
    let row = user::ActiveModel {
        id: sea_orm::ActiveValue::Set(data.id.to_owned()),
        r#type: sea_orm::ActiveValue::Set(data.r#type.to_owned()),
        ..Default::default()
    };
    println!("{:?}", row);
    let inserted_row = User::insert(row).exec_with_returning(&db).await;
    println!("{:?}", inserted_row);
    match inserted_row {
        Ok(row) => Ok(warp::reply::json(&row)),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))),
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
