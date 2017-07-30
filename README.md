# Summary
Supposed to be a simple http client (similar to wget), just a toy example to learn Rust.

# Requirements
You will need Rust nightly to be able to build this (the DNS lookup code specifically needs nightly).
# Testing
Easiest way to test is to run a local Python web server using `python -m http.server <socket_number>` and send http requests to it using this program (at least until I figure out DNS).