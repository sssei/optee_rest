use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write, Cursor, BufReader, stdout};

use rustls;
use webpki_roots;

fn measure(tls: &mut rustls::Stream<rustls::ClientConnection, TcpStream>){
    tls.write_all(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "Done\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    tls.flush().unwrap();
    let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "Current ciphersuite: {:?}",
        ciphersuite.suite()
    )
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();    
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
    
    measure(&mut tls);
/*     stdout().write_all(&plaintext).unwrap();     */
}