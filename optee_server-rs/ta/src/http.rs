use optee_utee::trace_println;
use crate::file;

#[derive(PartialEq)]
pub enum HttpParseState {
    Start,
    Finish,
    Get,
    Post,
    BodyStart,
    BodyIncomplete,
}

fn parse_start<'a>(request: &'a String, http_state: &'a mut HttpParseState) -> Vec<&'a str> {
    let res : Vec<&str> = request.split("\r\n").collect();
    let request_line : Vec<&str> = res[0].split(" ").collect();
    if request_line.len() != 3 {
        trace_println!("Request Line size is bad");
        panic!();
    }
    let uri = request_line[1];
    match request_line[0] {
        "POST" => *http_state = HttpParseState::Post,
        "GET" => {
            *http_state = HttpParseState::Finish;
            return vec!["GET", uri];
        }
        _ => {
            trace_println!("Request method is bad");
            panic!();
        }
    }

    let mut content_length : usize = 0;
    for r in &res[1..] {
        let field = "Content-Length: ";
        if r.contains(field) {
            content_length = r[field.len()..].parse::<usize>().unwrap();
        }else if r.len() == 0 {
            *http_state = HttpParseState::BodyStart;
        }else if *http_state == HttpParseState::BodyStart {
            if r.len() == content_length {
                *http_state = HttpParseState::Finish;
                return vec!["POST", uri, r];
            }else {
                *http_state = HttpParseState::BodyIncomplete;
                trace_println!("body size not content-length");
                panic!();
                return vec!["POST", uri, r];
            }
        }
    }
    trace_println!("Request header is bad");
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
    trace_println!("Request is {:?}", request);
    let mut res_header : Vec<u8> = b"HTTP 200 OK\r\n".to_vec();
    match request[0] {
        "POST" => {
            handle_post(request[2]);
            response.append(&mut res_header);
        }
        "GET" => {
            handle_get(response);
        }
        _ => {
            trace_println!("bad request");
            panic!();
        }
    }
} 

fn handle_post(body: &str){
    let bytes : &[u8] = body.as_bytes();
    file::create_raw_object(bytes.to_vec()).unwrap();
}

fn handle_get(data: &mut Vec<u8>){
    file::read_raw_object(data).unwrap();
}