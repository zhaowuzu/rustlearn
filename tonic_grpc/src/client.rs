use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
pub mod hello_world {
    tonic::include_proto!("helloworld"); // 绑定对应的proto
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut client = GreeterClient::connect("http://[::1]:50051").await?; // ipv6形式的ip
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?; // ipv4

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}


