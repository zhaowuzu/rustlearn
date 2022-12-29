// tonic的gprc的实现，所以需要引入
use tonic::{transport::Server, Request, Response, Status};

// proto创建的哪个服务，引用进来
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");// 绑定对应的proto
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter { // 实现proto中定义的接口
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main] // 使用Tokin运行时来跑服务器，所以需要加入Tokin的依赖
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   // let addr = "[::1]:50051".parse()?;// ipv6的形式的地址
    let addr = "0.0.0.0:50051".parse()?;// ipv4
    let greeter = MyGreeter::default();
    println!("{}",addr);
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}


