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

use optee_teec::{Context, Operation, Session, Uuid};
use optee_teec::{ParamNone, ParamType, ParamValue};
use proto::{Command, UUID};
use std::time::{Duration, Instant};

fn measure(ta_session: &mut Session, size: u32) {
    let mut time_nothing = Duration::new(0, 0);
    let mut time_create = Duration::new(0, 0);
    let mut time_read  = Duration::new(0, 0);
    let mut time_delete = Duration::new(0, 0);        

    let iter = 10; 

    for _ in 0..iter {
        let time0 = Instant::now();
        nothing(ta_session).unwrap();
        let time1 = Instant::now();
        create_file(ta_session, size).unwrap();
        let time2 = Instant::now();
        read_file(ta_session).unwrap();
        let time3 = Instant::now();    
        delete_file(ta_session).unwrap();
        let time4 = Instant::now();

        time_nothing += time1 - time0;
        time_create += time2 - time1;
        time_read += time3 - time2; 
        time_delete += time4 - time3;
    }

    println!("size = {:?}", size);
    println!("{:?}", (time_create - time_nothing) / iter );
    println!("{:?}", (time_read - time_nothing) / iter);
    println!("{:?}", (time_delete - time_nothing) / iter);
}

fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut ta_session = ctx.open_session(uuid)?;

    for i in 1..62 {
        let size = i * 256;
        measure(&mut ta_session, size);
    }

    Ok(())
}

fn nothing(ta_session: &mut Session) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(0, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    ta_session.invoke_command(Command::Nothing as u32, &mut operation)?;
    Ok(())
}

fn create_file(ta_session: &mut Session, file_size: u32) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(file_size, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    ta_session.invoke_command(Command::CreateFile as u32, &mut operation)?;
    Ok(())
}

fn read_file(ta_session: &mut Session) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(0, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    ta_session.invoke_command(Command::ReadFile as u32, &mut operation)?;
    Ok(())
}

fn delete_file(ta_session: &mut Session) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(0, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    ta_session.invoke_command(Command::DeleteFile as u32, &mut operation)?;
    Ok(())
}


