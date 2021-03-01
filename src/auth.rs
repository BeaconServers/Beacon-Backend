use tokio::task;
use argon2::{self, Config};
use rand::prelude::*;
use warp::http::StatusCode;
use std::collections::HashMap;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
    email: String,
}

pub async fn register(new_user: User, db: Arc<Mutex<HashMap<String, User>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = db.lock().await;
        
    if users.contains_key(&new_user.username) {
        return Ok(
        warp::reply::with_status(
            "You've already signed up!",
            StatusCode::BAD_REQUEST,
        )
        );
    }
    
    let hashed_user = User {
        username: new_user.username,
        password: hash(new_user.password).await,
        email: new_user.email,
    };
        
    users.insert(hashed_user.username.clone(), hashed_user);
    Ok(warp::reply::with_status(
            "Registered sucessfuly!",
            StatusCode::OK,
        ))
}

pub async fn login(auth_user: User, db: Arc<Mutex<HashMap<String, User>>> ) -> Result<impl warp::Reply, warp::Rejection> {
    let users = db.lock().await;
        
    match users.get(&auth_user.username) {
        None => Ok(warp::reply::with_status(
                                "Bad username!",
                                StatusCode::UNAUTHORIZED,
                            )),
        Some(user) => {
            if verify((&*user.password).to_string(), auth_user.password).await {
                Ok(warp::reply::with_status(
                        "Logged in sucessfully!",
                        StatusCode::OK,
                ))
            
            } else {
                Ok(warp::reply::with_status(
                        "Bad password!",
                        StatusCode::UNAUTHORIZED,
                ))
                
            }
        }
    }
}

async fn verify(passwd: String, auth_passwd: String) -> bool {
    // Run in tokio's spawn_blocking for efficiency
    task::spawn_blocking(move || {
        argon2::verify_encoded(&passwd, auth_passwd.as_bytes()).unwrap()
        
    }).await.unwrap()    
}


async fn hash(passwd: String) -> String {
    // Run in tokio's spawn_blocking for efficiency
    task::spawn_blocking(move || {
        let salt = rand::thread_rng().gen::<[u8; 16]>();
        println!("{:?}", &salt);
    
        argon2::hash_encoded(passwd.as_bytes(), &salt, &Config::default()).unwrap()
        
    }).await.unwrap()
}
