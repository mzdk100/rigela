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

use crate::pipe::{server_run, PipeStream};
use log::error;
use serde::{Deserialize, Serialize};
use tokio::{
    net::windows::named_pipe::ClientOptions,
    time::{sleep, Duration},
};
use win_wrap::{
    common::close_handle,
    threading::{
        get_current_process_id, open_process, wait_for_single_object, PROCESS_SYNCHRONIZE,
    },
};

const PIPE_NAME: &str = r"\\.\PIPE\RIGELA_KILLER";

#[derive(Deserialize, Serialize)]
enum KillSignal {
    Request,
    Response(u32),
}

pub async fn wait_until_killed() {
    let mut stream = server_run::<KillSignal>(PIPE_NAME).await;
    while let Ok(_) = stream.recv().await {
        break;
    }
    stream
        .send(&KillSignal::Response(get_current_process_id()))
        .await
        .unwrap_or(());
}

pub async fn kill() {
    let mut stream = match ClientOptions::new().open(PIPE_NAME) {
        Ok(x) => PipeStream::new(x),
        Err(_) => return,
    };

    if let Err(e) = stream.send(&KillSignal::Request).await {
        error!("{}", e);
    }
    if let Ok(KillSignal::Response(pid)) = stream.recv().await {
        if let Ok(handle) = open_process(PROCESS_SYNCHRONIZE, false, pid) {
            wait_for_single_object(handle, 5000);
            close_handle(handle);
        }
    }

    sleep(Duration::from_millis(1000)).await;
}
