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

use std::io::{Read, Write};

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
    
    loop{
        let mut cur_stream = stream.accept().unwrap();
        loop {
            let mut buf = [0u8; MAX_WIRE_SIZE];
            match cur_stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => cur_stream.write_all(&buf[..n]).unwrap(),
                Err(_) => {
                    trace_println!("Read Error");
                    break;
                }
            }
        }
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
