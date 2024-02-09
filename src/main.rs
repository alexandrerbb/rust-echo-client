use tokio::{
    spawn, 
    io::{AsyncReadExt, AsyncWriteExt}, 
    net::TcpStream, 
    task::JoinHandle
};
use std::str::from_utf8;

#[tokio::main]
async fn main() -> io::Result<()> {
    const SERV_ADDR:&str = "127.0.0.1:6142";
    
    let mut handles = Vec::<JoinHandle<()>>::new();

    for _ in 0..999 { 
        let mut stream = TcpStream::connect(SERV_ADDR).await?;

        handles.push(spawn(async move {
            let mut response_buff = [0_u8; 128];
            stream.write(&mut "Hello world".as_bytes()).await.unwrap();
            stream.read(&mut response_buff).await.unwrap();
            println!("{}", from_utf8(&response_buff).unwrap());
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }
    Ok(())
}
