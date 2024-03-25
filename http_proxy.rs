use warp::Filter;
use dialoguer::Input;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting NextProxy server...");

    // Prompt the user to input the IP address
    let ip_address: String = Input::new()
        .with_prompt("Enter IP address")
        .interact_text()?;
    let port = 8080; // default bind port 

    // Define the address to bind the server to
    let addr = format!("{}:{}", ip_address, port);

    // Define a warp filter for the root URL
    let root = warp::path::end().map(|| {
        "Welcome to your binded web server! This is only the beginning."
    });

    // Define a warp filter for the /hello route
    let hello = warp::path("hello").and(warp::path::param()).map(|name: String| {
        // Replace this with your custom response for the /hello route
        format!("Hello, {}!", name)
    });

    // Combine the root and hello filters
    let routes = root.or(hello);

    // Start the warp server
    warp::serve(routes)
        .run(addr.parse::<SocketAddr>()?)
        .await;

    println!("NextProxy server started.");

    Ok(())
}
