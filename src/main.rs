#![feature(lookup_host)]
use std::io::prelude::*;
use std::env;
use std::net::TcpStream;

use std::net;

fn main() {

    println!("Welcome to this simple Rust wget clone...");

    // First, parse the url the user wants to wget.
    let url = parse_args();

    // TODO Then, construct the HTTP message to send.
//    let http_message: String = format!("GET {} HTTP/1.1\n\n", url);

    let http_message = "GET / HTTP/1.1\n\n";
    println!("[DEBUG] The http message is:\n{}", http_message);

    
    // DEBUG ONLY: print out all IPs for every result we get from the DNS query.
    for host in net::lookup_host(&url) {
    	host.map(|sock_addr: std::net::SocketAddr| println!("ip: {}", sock_addr.ip())).count();
    	// Same as doing this:
    	// for sock_addr in host {
    	// 	println!("ip: {}", sock_addr.ip());
    	// }
    }

    // Then, open a socket/whatever to the URL.
    	// TODO First, we need to do a DNS lookup on the given url to get the IP address.
    	// Then we can open a TCP socket to that IP address.
	let address = "localhost:1234";
    let mut stream = TcpStream::connect(address).unwrap();

    // Write out the entire HTTP message byte-by-byte.
    let _ = stream.write_all(&http_message.as_bytes());

    // Read back the response from the stream.
    let mut response = String::new();
    let _ = stream.read_to_string(&mut response);

    // Display the response.
    println!("response:\n{:?}", response);
}

fn parse_args() -> String {
	// Grab all the args from cmdline.
	let args: Vec<String> = env::args().collect();

	// TODO clean up this ugly mess!
	return match args.len() {
		// If no args, panic!
		1 => {help(); panic!();},
		2 => {
			// If one arg, check that it can be parsed as a string.
			match args[1].parse() {
				Ok(arg) => {arg},
				// Otherwise, panic!
				_ => {help(); panic!();},
			}
		},
		// If more than one arg, panic!
		_ => {help(); panic!();},
	}
}

fn help() {
    println!("usage:
rust_web_client <url>
Fetch stuff from HTTP server at given url.");
}