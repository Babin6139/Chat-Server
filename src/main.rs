use tokio::net::TcpListener;

#[tokio::main]
async fn main(){
    let listener=TcpListener::bind("localhost:8080").await.unwrap();
    let (socket,address)=listener.accept().await.unwrap();
}