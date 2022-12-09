// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};
use optee_utee::net::TcpStream;
use proto::Command;

use rustls;
use rustls::{NoClientAuth, Session};
use std::io::Cursor;
use std::io::{BufReader, Read, Write};
use std::sync::Arc;

const MAX_PAYLOAD: u16 = 16384 + 2048;
const HEADER_SIZE: u16 = 1 + 2 + 2;
pub const MAX_WIRE_SIZE: usize = (MAX_PAYLOAD + HEADER_SIZE) as usize;

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(cmd_id: u32, params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA invoke command");
    let session_id = unsafe { params.0.as_value().unwrap().a() };
    trace_println!("[+] session id: {}", session_id);
    match Command::from(cmd_id) {
        Command::DeployServer => {
            trace_println!("[+] socket_listen");
            deploy_server();
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

pub fn deploy_server() {
    let mut stream = TcpStream::listen("0.0.0.0", 8089).unwrap();
    trace_println!("[+] deploy_server");
    stream.accept().unwrap();

    let mut tls_session = new_tls_session();
    loop {
        let mut plain_buf : Vec<u8> = Vec::new();
        let mut buf = [0u8; MAX_WIRE_SIZE];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                do_tls_read(&mut tls_session, &buf[..n], &mut plain_buf);
            }
            Err(_) => {
                trace_println!("Read Error");
                break;
            }
        }
        let n = do_tls_write(&mut tls_session, &mut buf, &plain_buf);
        stream.write(&buf[..n]).unwrap();
    }
    
}

pub fn new_tls_session() -> rustls::ServerSession {
    let tls_config = make_config();
    rustls::ServerSession::new(&tls_config)
}

fn do_tls_read(tls_session: &mut rustls::ServerSession, encrypted_buf: &[u8], plain_buf: &mut Vec<u8>){
    let mut rd = Cursor::new(encrypted_buf);
    let _rc = tls_session.read_tls(&mut rd).unwrap();
    let _process =  tls_session.process_new_packets().unwrap();
    let _rc = tls_session.read_to_end(plain_buf);
}

fn do_tls_write(tls_session: &mut rustls::ServerSession, encrypted_buf: &mut [u8], plain_buf: &[u8]) -> usize {
    if !plain_buf.is_empty() {
        tls_session.write_all(&plain_buf).unwrap();
    }
    let mut wr = Cursor::new(encrypted_buf);
    let mut rc = 0;
    while tls_session.wants_write() {
        rc += tls_session.write_tls(&mut wr).unwrap();
    }
    rc
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
    let bytes = include_bytes!("../test-ca/ecdsa/end.fullchain").to_vec();
    let cursor = std::io::Cursor::new(bytes);
    let mut reader = BufReader::new(cursor);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key() -> rustls::PrivateKey {
    let bytes = include_bytes!("../test-ca/ecdsa/end.key").to_vec();

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

// TA configurations
const TA_FLAGS: u32 = 0;
const TA_DATA_SIZE: u32 = 18 * 1024 * 1024;
const TA_STACK_SIZE: u32 = 2 * 1024 * 1024;
const TA_VERSION: &[u8] = b"0.2\0";
const TA_DESCRIPTION: &[u8] = b"This is a tls server example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"TLS Server TA\0";
const EXT_PROP_VALUE_2: u32 = 0x0010;
const TRACE_LEVEL: i32 = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: u32 = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));
