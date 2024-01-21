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

use crate::model::{Proxy32Data, Proxy32Packet};
use rigela_utils::pipe::{server_run, PipeStream};
use tokio::net::windows::named_pipe::NamedPipeServer;

pub struct Proxy32Server {
    stream: PipeStream<Proxy32Packet, NamedPipeServer>,
}
impl Proxy32Server {
    pub async fn new(pipe_name: &str) -> Self {
        let stream = server_run(pipe_name).await;
        Self { stream }
    }
    pub async fn run(&mut self) {
        loop {
            let packet = self.stream.recv().await;
            match packet {
                None => break,
                Some(p) => {
                    let data = self.on_exec(&p.data).await;
                    let packet = Proxy32Packet { id: p.id, data };
                    self.stream.send(&packet).await;
                    if Proxy32Data::QUIT == packet.data {
                        break;
                    }
                }
            };
        }
    }
    async fn on_exec(&self, data: &Proxy32Data) -> Proxy32Data {
        data.clone()
    }
}
