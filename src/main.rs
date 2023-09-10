use rocket::tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use rocket::tokio::spawn;
use futures::StreamExt;
use futures::SinkExt;
use rocket::fs::NamedFile;
use std::path::Path;

#[rocket::get("/")]
async fn test() -> Option<NamedFile> {
    let path = Path::new("test.html");
    NamedFile::open(path).await.ok()
}

#[rocket::launch]
async fn rocket() -> _ {
    // 创建本地9001端口的监听器
    let listener = TcpListener::bind("localhost:9001").await.unwrap();
    spawn(async move {
        // 建立与客户端的TCP连接
        while let Ok((stream, _)) = listener.accept().await {
            // 对于每个来自客户端的请求，新建一个异步任务
            spawn(async move {
                 // 分离发送消息的端口和接收消息的端口
                let (mut websocket_sink, mut websocket_stream) = accept_async(stream).await.unwrap().split();
               
               spawn(async move {
                    while let Some(message) = websocket_stream.next().await {
                        match message {
                            Ok(msg) if msg.is_text() || msg.is_binary() => {
                                websocket_sink.send(msg).await.unwrap();
                            }
                            _ => (),
                        }
                    }
                });
            });
        }
    });

    rocket::build().mount("/", rocket::routes![test])
}