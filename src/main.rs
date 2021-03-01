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
    let db = Arc::new(Mutex::new(HashMap::<String, User>::new()));
    let db = warp::any().map(move || Arc::clone(&db));
    
    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::form())
        .and(db.clone())
        .and_then(register);
    
    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::form())
        .and(db.clone())
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
