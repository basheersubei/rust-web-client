use std::io::prelude::*;
use std::env;
use std::net::TcpStream;

fn main() {

    println!("Welcome to this simple Rust wget clone...");

    // First, parse the url the user wants to wget.
    let url = parse_args();

    // TODO Then, construct the HTTP message to send.
//    let http_message: String = format!("GET {} HTTP/1.1\n\n", url);

//    let address = "64.78.59.92:80";
//    let http_message = "GET /java/host/test.html HTTP/1.0\n\n";

    let http_message = "GET / HTTP/1.1\n\n";
    println!("[DEBUG] The http message is:\n{}", http_message);

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