use std::sync::Arc;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader};
use std::time::Instant;
use std::fs::File;

use rustls;
use webpki_roots;
use chrono::Utc;

fn measure(tls: &mut rustls::Stream<rustls::ClientConnection, TcpStream>, size: usize, buf : &[u8], iter: u32){
    let mut bytes = vec![0u8; size];
    for _ in 0..iter { 
        tls.write_all(&buf[0..size]).unwrap();
        tls.flush().unwrap();
        let mut n = 0;
        loop {
            n += tls.read(&mut bytes).unwrap();
            if n >= size{
                break;
            }
        }
    }
}

fn main() {
    let mut root_store = rustls::RootCertStore::empty();    
    let bytes = include_bytes!("../ca.cert").to_vec();
    let cursor = std::io::Cursor::new(bytes);
    let mut reader = BufReader::new(cursor);    

    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }),
    );    
    root_store.add_parsable_certificates(&rustls_pemfile::certs(&mut reader).unwrap());

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();    

    let server_name = "localhost".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();    
    let mut sock = TcpStream::connect("localhost:8089").unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
 
    let file_name = Utc::now().format("%Y-%m-%d_%H-%M").to_string();
    let mut file = File::create(file_name).unwrap();
    let buf = vec![0u8; 1 << 27];
    let iter = 100;
    let mut time_buf = vec![0u128; 72];
    for i in 1..72 { 
        let sz = 256 * i;
/*         write!(file, "size = {}\n", sz).unwrap(); */
        let st = Instant::now();
        measure(&mut tls, sz, &buf, iter);
        let ed = Instant::now();
        time_buf[i] = ed.duration_since(st).as_nanos();
/*         write!(file, "{:?}\n", (ed - st) / iter).unwrap(); */
    }
    for i in 1..72 {
        write!(file, "size = {}\n", 256 * i).unwrap();
        write!(file, "{:?}ns\n", time_buf[i]).unwrap();
    }
    file.flush().unwrap();
/*     stdout().write_all(&plaintext).unwrap();     */
}