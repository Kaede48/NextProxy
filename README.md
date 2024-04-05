# NextProxy
NextProxy is a basic HTTP Proxy written in Rust. This project is to help me learn and understand the Rust language for future projects. This is currently an HTTP only proxy and not using HTTPS through TLS. I plan to incorporate the library of Rustls to allow the user to use an HTTPS proxy. 

As with any Rust project, you will need to compile it in order to run. The Cargo.toml file will have the dependencies you will need to in order to run NextProxy. 

This is mostly a plug and play install for any user who wants to use a basic proxy server locally. 

# Install Guide for Rust

Download Rust for whatever OS you are running. The Windows version is here: https://www.rust-lang.org/tools/install

Unix/Linux can be downloading using the following command from Rust themselves: 

```bash 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
```
Once installed, you can use the rustc --version to determine if you have the latest version. 

# Install Guide for NextProxy

Once Rust is installed, you will need to git clone the project into a terminal. 

```bash 
git clone https://github.com/Kaede48/NextProxy.git
``` 

Then, you will need to cd into the folder where NextProxy is installed using cd NextProxy 

Once you cd into that directory, run the following to compile and run the project:

```bash 
cargo run --bin nextproxy
``` 

This will compile the code and run the next proxy. If successful, you should see the following: 

Starting NextProxy server...
NextProxy server started with Warp at 0.0.0.0:8088

You can now navigate to the following on a browser to access your web server. 

http://your_public_ip:8088 

The local address for your machine will also work as a fallback and the loopback address (127.0.0.1) also works if you want this to run locally. 

There are still issues with the server that I need to resolve. For example, I need to upgrade this to tls and get the traffic encrypted as well as fix some issues where the server wont fully act on certain urls. For example, http://your_public_ip:8088/proxy/youtube.com will bring you to Youtube, but cannot go further and watch videos. This is still a WIP and will be going getting updates as I learn more. 

I see this basic proxy server being ran on an ip in the cloud and it forwarding requests for ethical hacking which is the end goal for this project. I want this server to be able to take the requests a user sends it (encrypted) and forward them for web-based pen testing. It provides a layer of security by having the data being routed through an obscure port and different address. 

Thank you for trying this project. 
