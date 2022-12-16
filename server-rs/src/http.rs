use std::fs::File;
use std::fs;
use std::io::{Write, Read};

#[derive(PartialEq)]
pub enum HttpParseState {
    Start,
    Finish,
    Post,
    BodyStart,
    BodyIncomplete,
}

fn parse_start<'a>(request: &'a String, http_state: &'a mut HttpParseState) -> Vec<&'a str> {
    let res : Vec<&str> = request.split("\r\n").collect();
    let request_line : Vec<&str> = res[0].split_whitespace().collect();
    if request_line.len() != 3 {
        println!("Request Line size is bad");
        panic!();
    }
    let uri = request_line[1];
    match request_line[0] {
        "POST" => *http_state = HttpParseState::Post,
        "GET" => {
            *http_state = HttpParseState::Finish;
            return vec!["GET", uri];
        }
        "DELETE" => {
            *http_state = HttpParseState::Finish;
            return vec!["DELETE", uri];
        }
        _ => {
            println!("Request method is bad");
            panic!();
        }
    }

    let mut content_length : usize = 0;
    for r in &res[1..] {
        let field = "Content-Length: ";
        if r.contains(field) {
            let res : String = r[field.len()..].split_whitespace().collect();
            content_length = res.parse::<usize>().unwrap();
        }else if r.len() == 0 {
            *http_state = HttpParseState::BodyStart;
        }else if *http_state == HttpParseState::BodyStart {
            if r.len() == content_length {
                *http_state = HttpParseState::Finish;
                return vec!["POST", uri, r];
            }else {
                *http_state = HttpParseState::BodyIncomplete;
                println!("body size not content-length");
                panic!();
            }
        }
    }
    println!("Request header is bad");
    panic!();
}

fn parse_request<'a>(request: &'a String, http_state: &'a mut HttpParseState) -> Vec<&'a str> {
    match http_state {
        HttpParseState::Start => parse_start(&request, http_state),
        HttpParseState::BodyStart => vec!["N", "N", "N"],
        HttpParseState::BodyIncomplete => vec!["N", "N", "N"],
        _ => vec!["N", "N", "N"],
    }
}

pub fn handle_request(plain_buf : &mut Vec<u8>, response : &mut Vec<u8>, http_state: &mut HttpParseState) {
    let res = plain_buf.iter().map(|&s| s as char).collect::<String>();
    let request = parse_request(&res, http_state);    
    let uri = request[1];
    let mut res_header : Vec<u8> = b"HTTP 200 OK\r\nContent-Length: ".to_vec();
    response.append(&mut res_header);
    match request[0] {
        "POST" => {
            let body = request[2];
            handle_post(uri, body);
            response.append(&mut b"0\r\n\r\n".to_vec());
        }
        "GET" => {
            handle_get(uri, response);
        }
        "DELETE" => {
            handle_delete(uri);
            response.append(&mut b"0\r\n\r\n".to_vec());                        
        }
        _ => {
            println!("bad request");
            panic!();
        }
    }
} 

fn handle_post(uri: &str, body: &str){
    let bytes : &[u8] = body.as_bytes();
    let mut file = File::create(uri).unwrap();
    file.write_all(bytes).unwrap();
}

fn handle_get(uri: &str, response: &mut Vec<u8>) {
    let mut data = String::new();
    let mut file = File::open(uri).unwrap();
    let size = file.read_to_string(&mut data).unwrap();
    response.append(&mut size.to_string().as_bytes().to_vec());
    response.append(&mut b"\r\n\r\n".to_vec());
    response.append(&mut data.as_bytes().to_vec());
}

fn handle_delete(uri: &str){
    fs::remove_file(uri).unwrap();
}
