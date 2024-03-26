use warp::Filter;
use std::net::{IpAddr, SocketAddr};
use get_if_addrs::get_if_addrs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting NextProxy server...");

    // Find the local IPv4 address
    let ip_address = find_local_ipv4().unwrap_or_else(|| IpAddr::V4("127.0.0.1".parse().unwrap()));
    let port = 8080; // default bind port 

    // Define the address to bind the server to
    let addr = SocketAddr::new(ip_address, port);

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
        .run(addr)
        .await;

    println!("NextProxy server started.");

    Ok(())
}

// Function to find the local IPv4 address
fn find_local_ipv4() -> Option<IpAddr> {
    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if !iface.is_loopback() && iface.ip().is_ipv4() {
                return Some(iface.ip());
            }
        }
    }
    None
}
