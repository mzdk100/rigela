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

use tokio::net::windows::named_pipe::{ClientOptions, NamedPipeClient};
use rigela_utils::pipe::PipeStream;
use crate::model::PeeperPacket;
use crate::utils::get_pipe_name;

/**
 * peeper的client运行在远进程中。
 * */
pub(crate) struct PeeperClient {
    stream: PipeStream<PeeperPacket, NamedPipeClient>
}

impl PeeperClient {
    pub async fn new() -> Self {
        let pipe = ClientOptions::new()
            .open(get_pipe_name())
            .unwrap();
        let stream = PipeStream::new(pipe);
        Self {
            stream
        }
    }
    pub fn quit(&self) {}
}