#![feature(str_split_whitespace_remainder)]

use std::str::SplitWhitespace;



pub fn router(http_request: &Vec<String>) -> String {
    let header: &String = http_request.get(0).unwrap_or(&"ERROR 400: no request header specified".to_owned());
    println!("{:?}", http_request.get(0));
    let request_split = header.split_whitespace();
    let request_method = request_split.next().unwrap_or("ERROR 400");
    let request_path = request_split.next().unwrap_or("ERRORv400");

}

fn handle_root(http_request: &Vec<String>) -> String {
    
}