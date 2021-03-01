#![deny(clippy::all)]

use warp::Filter;
use tokio::runtime::Builder;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use auth::{login, register, User};
mod auth;

fn main() {
let rt =  Builder::new_multi_thread().enable_io().build().unwrap();
rt.block_on(async {

    // The big ol' database storing user data n stuff
    // The key is the username, and the val is said user's User struct
    let cred_db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let cred_db = warp::any().map(move || Arc::clone(&cred_db));
    
    //Currently logged in users and their session cookies
    let cookie_db = Arc::new(Mutex::new(HashMap::<String, (String, u64)>::new()));
    let cookie_db = warp::any().map(move || Arc::clone(&cookie_db));
    
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::form())
        .and(cred_db.clone())
        .and_then(register);
    
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::form())
        .and(cred_db.clone())
        .and(cookie_db.clone())
        .and_then(login);
    
    let routes = warp::post().and(
        register
        .or(login)
    );


    warp::serve(routes)
        .run(([127, 0, 0, 1], 7878))
        .await;

})
}
