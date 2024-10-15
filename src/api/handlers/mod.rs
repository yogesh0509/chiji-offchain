use std::any::Any;

use alloy::primitives::map::HashMap;
// use ::entity::post;
use ::entity::prelude::{Proposal, Space, Vote};
use ::entity::{proposal, space, vote};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use warp::filters::multipart::FormData;
use warp::{reject::Reject, Rejection, Reply};
use serde_json::json;

pub mod error;
pub use error::handle_rejection;

#[derive(Serialize)]
struct GenericResponse {
    status: String,
    message: String,
}

#[derive(Debug)]
pub struct DatabaseError(pub DbErr);

impl Reject for DatabaseError {}

// Proposal handlers
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
        creator_address: Set(data.creator_address.to_owned()),
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
        id: Set(data.id.to_owned()),
        name: Set(data.name.to_owned()),
        about: Set(data.about.to_owned()),
        avatar: Set(data.avatar.to_owned()),
        symbol: Set(data.symbol.to_owned()),
        token_address: Set(data.token_address.to_owned()),
        governance_contract_address: Set(data.governance_contract_address.to_owned()),
        twitter: Set(data.twitter.to_owned()),
        discord: Set(data.discord.to_owned()),
        terms: Set(data.terms.to_owned()),
        admins: Set(data.admins.to_owned()),
        authors: Set(data.authors.to_owned()),
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

pub async fn join_space(
    address: String,
    space_id: i32,
    db: DatabaseConnection,
) -> Result<impl Reply, Rejection> {

    println!("user: {}", address);
    let mut space: space::ActiveModel = match Space::find_by_id(space_id).one(&db).await {
        Ok(Some(s)) => s.into(),
        Ok(None) => return Err(warp::reject::not_found()),
        Err(e) => return Err(warp::reject::custom(DatabaseError(e))),
    };

    let users_json = space.users.as_ref();
    let mut users: Vec<String> = users_json
    .as_ref()
    .and_then(|v| v.as_array())  
    .map(|arr| arr.iter()
        .filter_map(|v| v.as_str().map(|s| s.to_owned()))  // Safely borrow & clone strings
        .collect()
    )
    .unwrap_or_default();

    println!("{:?}", users);

    if !users.contains(&address) {
        users.push(address.to_owned());

        space.users = Set(Some(JsonValue::Array(
            users.into_iter().map(JsonValue::String).collect(),
        )));

        match space.update(&db).await {
            Ok(updated_space) => Ok(warp::reply::json(&updated_space)),
            Err(e) => Err(warp::reject::custom(DatabaseError(e))),
        }
    } else {
        Ok(warp::reply::json(
            &json!({ "message": "user have already joined the space" }),
        ))
    }
}

pub async fn upload_space_logo(data: HashMap<String, String>) -> Result<impl Reply, Rejection> {
    println!("entering upload space logo");
    // println!("{:?}", form);

    const MESSAGE: &str = "Upload_space_logo";
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(warp::reply::json(response_json))
}

pub async fn create_vote(
    db: DatabaseConnection,
    data: vote::Model,
) -> Result<impl Reply, Rejection> {
    println!("{:?}", data);
    let row = vote::ActiveModel {
        id: Set(data.id.to_owned()),
        user_address: Set(data.user_address.to_owned()),
        proposal_id: Set(data.proposal_id.to_owned()),
        vote_type: Set(data.vote_type.to_owned()),
        ..Default::default()
    };
    println!("{:?}", row);
    let inserted_row = Vote::insert(row).exec_with_returning(&db).await;
    println!("{:?}", inserted_row);
    match inserted_row {
        Ok(row) => Ok(warp::reply::json(&row)),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))),
    }
}

pub async fn get_user_vote(
    user_address: String,
    proposal_id: i32,
    db: DatabaseConnection,
) -> Result<impl Reply, Rejection> {
    let vote = Vote::find()
        .filter(vote::Column::UserAddress.eq(user_address))
        .filter(vote::Column::ProposalId.eq(proposal_id))
        .one(&db)
        .await;

    match vote {
        Ok(Some(vote)) => Ok(warp::reply::json(&vote.vote_type)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(err) => Err(warp::reject::custom(DatabaseError(err))),
    }
}

pub async fn health_checker_handler() -> Result<impl Reply, Rejection> {
    const MESSAGE: &str = "Build Simple CRUD API with Rust";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(warp::reply::json(response_json))
}
