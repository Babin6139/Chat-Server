use tokio::{
    net::TcpListener, 
    io::{AsyncWriteExt,BufReader, AsyncBufReadExt}, sync::broadcast
};

// fn give_me_default<T>()->T where T:Default{
//     Default::default()
// }

#[tokio::main]
async fn main(){

    // let value=give_me_default::<i32>(); // turbofish rust

    let listener=TcpListener::bind("localhost:8080").await.unwrap();
    let (tx,rx)=broadcast::channel(10);
    loop{
        let (mut socket,address)=listener.accept().await.unwrap();
        let tx=tx.clone();
        let mut rx=tx.subscribe();
        tokio::spawn(async move{
            let (reader,mut writer)=socket.split();
        
        let mut reader=BufReader::new(reader);
        let mut line=String::new();
        loop{
            tokio::select! {
                result = reader.read_line(&mut line)=>{
                    if result.unwrap() == 0{
                        break;
                    }
                    tx.send((line.clone(),address)).unwrap();
                    line.clear();
                }
                result = rx.recv()=>{
                    let (msg,other_addr)=result.unwrap();
                    if address != other_addr{
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                    
                }
            }

        }
    });
    }
}