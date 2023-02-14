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
use optee_teec::{ParamNone, ParamTmpRef, ParamType, ParamValue};
use proto::{Command, UUID};


fn main() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut ta_session = ctx.open_session(uuid)?;
    let mut session_id: u32 = 0;
    loop {
        deploy_server(&mut ta_session, session_id);
    }
    println!("Success");
    Ok(())
}

fn deploy_server(ta_session: &mut Session, session_id: u32) -> optee_teec::Result<()> {
    let p0 = ParamValue::new(session_id, 0, ParamType::ValueInput);
    let mut operation = Operation::new(0, p0, ParamNone, ParamNone, ParamNone);
    ta_session.invoke_command(Command::DeployServer as u32, &mut operation)?;
    Ok(())
}


