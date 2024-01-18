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

use log::error;
use tokio::{
    io::BufReader,
    net::windows::named_pipe::ClientOptions,
    time::{sleep, Duration},
};

pub async fn client_run(pipe_name: &str) {
    // 使用循环方法连接管道，因为可能在连接的时候管道还没创建完毕
    let client = loop {
        // 推迟一秒连接，尽量确保管道创建完毕
        sleep(Duration::from_millis(1000)).await;

        match ClientOptions::new().open(pipe_name) {
            Ok(x) => break x,
            Err(e) => {
                error!("Can't open the named pipe ({}). {}", pipe_name, e);
                continue;
            }
        }
    };
}