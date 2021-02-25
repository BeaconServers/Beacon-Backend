// If I ever want to combine threads with async
//use async_std::task;
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::spawn;
use std::time::Duration;
use std::thread::sleep;
use reinda::{assets, Assets, Config, Setup};

const ASSETS: Setup = assets! {
    // Folder which contains your assets, relative to your `Cargo.toml`.
    #![base_path = "Web"]

    // List of assets to include, with different settings.
    "index.html": { template },
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    /*
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            handle_connection(tcpstream).await.unwrap();
        })
        .await;*/
        
    loop {
        let (mut tcp_stream, _) = listener.accept().await.unwrap();
        spawn(async move {
           handle_connection(tcp_stream).await;
        });
    }
        
        Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await?;

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";

    sleep(Duration::from_secs(10));
    
    let assets = Assets::new(ASSETS, Config::default()).await?;
    let content_bytes = &assets.get("index.html").await?.unwrap();
    let content = String::from_utf8_lossy(content_bytes);

    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).await?;
    stream.flush().await?;
    
    Ok(())
}

/*use std::io::prelude::{Write, Read};
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::read_to_string;
use threadpool::ThreadPool;
use format::validate_request;

mod format;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];
    
    stream.read(&mut buffer).unwrap();
    let buffer = String::from_utf8_lossy(&buffer[..]);

    let (response_header, path, post_data, get_data) = validate_request(buffer.to_string());
    
    
    let response = response_header;

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}*/
