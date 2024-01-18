/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

#![windows_subsystem = "windows"]


#[cfg(target_arch = "x86")]
#[tokio::main]
async fn main() {
    use std::env;
    use tokio::net::windows::named_pipe::ServerOptions;

    let pipe_name = env::args().nth(1).unwrap();
    let server = ServerOptions::new()
        .create(pipe_name.as_str())
        .unwrap();
    server.connect()
        .await
        .unwrap();
    println!("{} connected.", pipe_name);
}

#[cfg(not(target_arch = "x86"))]
fn main() {
    panic!("X86 arch target only!");
}
