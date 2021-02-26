// If I ever want to combine threads with async
//use async_std::task;
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::spawn;
use tokio::runtime::Builder;
use format::validate_request;

mod format;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let rt =  Builder::new_multi_thread().build().unwrap();
    
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:7878").await?;

        loop {
            let (tcp_stream, _) = listener.accept().await?;
            spawn(async move {
            handle_connection(tcp_stream).await.unwrap();
            
            });
        }
    
    })
        
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).await?;
    
    let buffer = String::from_utf8_lossy(&buffer).to_string();
    let (response, path, post_data, get_data) = validate_request(buffer).await;
    
    stream.write(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
