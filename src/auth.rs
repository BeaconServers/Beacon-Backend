#![deny(clippy::all)]

use tokio::task;
use rand::Rng;
use rand::rngs::{ThreadRng as SaltRng};
use warp::http::StatusCode;
use std::collections::HashMap;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
    email: String,
}

#[derive(Deserialize)]
struct AuthUser {
    cookie: String,
    username: String,
}

pub async fn register(new_user: User, cred_db: Arc<Mutex<HashMap<String, User>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users = cred_db.lock().await;
        
    if users.contains_key(&new_user.username) {
        return 
            Ok(
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

pub async fn login(auth_user: User, cred_db: Arc<Mutex<HashMap<String, User>>>, auth_token_db: Arc<Mutex<HashMap<String, (String, u64)>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let users = cred_db.lock().await;
        
    match users.get(&auth_user.username) {
        None => Ok(warp::reply::with_status(
                                "Bad username!",
                                StatusCode::UNAUTHORIZED,
                            )),
        Some(user) => {
            if verify((&*user.password).to_string(), auth_user.password).await {
                let mut hasher = Sha256::new(); 
                // Get a sha256 hash of a random 16 byte long byte array
                hasher.update(rand::thread_rng().gen::<[u8; 16]>());
            
                let mut auth_token_db = auth_token_db.lock().await;
                
                let auth_token = &hasher.finalize()[..];
                let auth_token = hex::encode(auth_token);
                
                //After a month, a cookie should be considered invalid
                let expiration_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 2629800;
                
                auth_token_db.insert((&*auth_token).to_string(), (auth_user.username, expiration_time));
                
                //Fuck the rust borrow checker, all my homies hate the Rust borrow checker
                //Literally the only way I can convert the cookie String into an &'static str so warp login
                // Can do its fucking job
                Ok(warp::reply::with_status(
                        Box::leak(auth_token.into_boxed_str()),
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

async fn verify(password_hash: String, auth_passwd: String) -> bool {
    // Run in tokio's spawn_blocking for efficiency
    task::spawn_blocking(move || {
        let argon2 = Argon2::default();
    
       let parsed_hash = PasswordHash::new(&password_hash).unwrap();
       
       argon2.verify_password(auth_passwd.as_bytes(), &parsed_hash).is_ok()
        
    }).await.unwrap()    
}


async fn hash(password: String) -> String {
    // Run in tokio's spawn_blocking for efficiency
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut SaltRng::default());
        let argon2 = Argon2::default();
        
        println!("{:?}", &salt);
        
        argon2.hash_password_simple(password.as_bytes(), salt.as_ref()).unwrap().to_string()
    
        
    }).await.unwrap()
}
