use std::net::{IpAddr, SocketAddr, Ipv4Addr};
use warp::{Filter, reply::html, Rejection};
use reqwest::{Client, Error};


async fn fetch_response(target: String) -> Result<impl warp::Reply, Rejection> {
    let target_url = format!("http://{}", target);
    let client = Client::new();
    match client.get(&target_url).send().await {
        Ok(res) => {
            let body = res.bytes().await;
            match body {
                Ok(body_bytes) => Ok(html(body_bytes)),
                Err(_) => Ok(html("Error occurred".to_string().into())),
            }
        }
        Err(_) => Ok(html("Error occurred".to_string().into())),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting NextProxy server...");

    let ip_address = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)); // Listen on all interfaces
    let port = 8088;
    let addr = SocketAddr::new(ip_address, port);

    let routes = warp::path!("proxy" / String)
        .and_then(fetch_response);

    let warp_server = warp::serve(routes).bind(addr);

    println!("NextProxy server started with Warp at 0.0.0.0:{}", port);

    warp_server.await;

    Ok(())
}
