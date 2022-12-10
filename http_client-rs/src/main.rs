use std::sync::Arc;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader};

use rustls;
use webpki_roots;


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

/*     let req = b"POST /config \r\n";
    let data = &mut[0u8; 1024];
    for i in 0..req.len() {
        data[i] = req[i]
    }
 */    
    let data = b"POST /config \r\nHello TA\r\n";
    tls.write_all(data).unwrap();
    tls.flush().unwrap();
    
    let mut buf = vec![0u8; 1024];    
    tls.read(&mut buf).unwrap();
    let res = buf.iter().map(|&s| s as char).collect::<String>();
    println!("{:?}", res);

    tls.write_all(b"GET /config \r\n").unwrap();
    tls.flush().unwrap();
    tls.read(&mut buf).unwrap();
    let res = buf.iter().map(|&s| s as char).collect::<String>();
    println!("{:?}", res);

}