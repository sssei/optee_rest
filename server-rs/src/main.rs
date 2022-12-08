use std::net::{TcpListener, TcpStream};
use lazy_static::lazy_static;
use rustls;
use rustls::{NoClientAuth, Session};
use std::collections::HashMap;
use std::io::Cursor;
use std::io::{BufReader, Read, Write};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

const MAX_PAYLOAD: u16 = 16384 + 2048;
const HEADER_SIZE: u16 = 1 + 2 + 2;
pub const MAX_WIRE_SIZE: usize = (MAX_PAYLOAD + HEADER_SIZE) as usize;


lazy_static! {
    static ref TLS_SESSIONS: RwLock<HashMap<u32, Mutex<rustls::ServerSession>>> =
        RwLock::new(HashMap::new());
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8089").unwrap();
    let mut session_id: u32 = 0;

    for stream in listener.incoming() {
        session_id += 1;
        handle_client(session_id, stream.unwrap());
    }

    println!("Success");
}


fn handle_client(
    session_id: u32,    
    mut stream: TcpStream,
) {
    println!("new session");
    new_tls_session(session_id);
    loop {
        let mut buf = [0u8; MAX_WIRE_SIZE];
        let mut flag;
        println!("stream read");
        match stream.read(&mut buf) {
            Ok(0) | Err(_) => {
                println!("close session");
                close_tls_session(session_id);
                break;
            }
            Ok(n) => {
                println!("read bytes: {}", n);
                flag = do_tls_read(session_id, &mut buf[..n]);
            }
        }

        let n = do_tls_write(session_id, &mut buf);
        println!("stream write n: {}", n);
        let res = stream.write_all(&buf[..n]);

        if res.is_err() || flag {
            println!("close session");
            close_tls_session(session_id);
            break;
        }
    }
}

fn do_tls_read(session_id: u32, buf: &[u8]) -> bool {
    let mut rd = Cursor::new(buf);
    let ts_guard = TLS_SESSIONS.read().unwrap();
    let mut tls_session = ts_guard.get(&session_id).unwrap().lock().unwrap();
    let _rc = tls_session.read_tls(&mut rd).unwrap();
    let _process =  tls_session.process_new_packets().unwrap();

        // Read and process all available plaintext.
    let mut buf = Vec::new();
    let _rc = tls_session.read_to_end(&mut buf);
    let plain_text: String = String::from_utf8(buf.clone()).unwrap();

    if !buf.is_empty() {
        tls_session.write_all(&buf).unwrap();
    }

    plain_text.contains("Done")
}

fn do_tls_write(session_id: u32, buf: &mut [u8]) -> usize {
    let ts_guard = TLS_SESSIONS.read().unwrap();
    let mut tls_session = ts_guard.get(&session_id).unwrap().lock().unwrap();
    let mut wr = Cursor::new(buf);
    let mut rc = 0;
    while tls_session.wants_write() {
        rc += tls_session.write_tls(&mut wr).unwrap();
    }

    rc
}

fn new_tls_session(session_id: u32) {
    let tls_config = make_config();
    let tls_session = rustls::ServerSession::new(&tls_config);
    TLS_SESSIONS
        .write()
        .unwrap()
        .insert(session_id, Mutex::new(tls_session));
}

fn close_tls_session(session_id: u32) {
    TLS_SESSIONS.write().unwrap().remove(&session_id);
}

fn make_config() -> Arc<rustls::ServerConfig> {
    let client_auth = NoClientAuth::new();
    let mut tls_config = rustls::ServerConfig::new(client_auth);
    let certs = load_certs();
    let privkey = load_private_key();
    tls_config
        .set_single_cert(certs, privkey)
        .expect("bad certificates/private key");

    Arc::new(tls_config)
}

fn load_certs() -> Vec<rustls::Certificate> {
    let bytes = include_bytes!("../test-ca/end.fullchain").to_vec();
    let cursor = std::io::Cursor::new(bytes);
    let mut reader = BufReader::new(cursor);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key() -> rustls::PrivateKey {
    let bytes = include_bytes!("../test-ca/end.key").to_vec();

    let rsa_keys = {
        let cursor = std::io::Cursor::new(bytes.clone());
        let mut reader = BufReader::new(cursor);
        rustls::internal::pemfile::rsa_private_keys(&mut reader)
            .expect("file contains invalid rsa private key")
    };

    let pkcs8_keys = {
        let cursor = std::io::Cursor::new(bytes);
        let mut reader = BufReader::new(cursor);
        rustls::internal::pemfile::pkcs8_private_keys(&mut reader)
            .expect("file contains invalid pkcs8 private key (encrypted keys not supported)")
    };

    // prefer to load pkcs8 keys
    if !pkcs8_keys.is_empty() {
        pkcs8_keys[0].clone()
    } else {
        assert!(!rsa_keys.is_empty());
        rsa_keys[0].clone()
    }
}