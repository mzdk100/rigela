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
use rigela_utils::pipe::{client_connect, PipeStream};
use std::collections::HashMap;
use tokio::net::windows::named_pipe::NamedPipeClient;

pub struct Proxy32Client {
    cached: HashMap<u32, Proxy32Data>,
    id: u32,
    stream: PipeStream<Proxy32Packet, NamedPipeClient>,
}

impl Proxy32Client {
    /**
     * 创建一个proxy32的客户端，可以下发各种请求命令。
     * */
    pub async fn new(pipe_name: &str) -> Self {
        let stream = client_connect(pipe_name).await;
        Self {
            cached: HashMap::new(),
            id: 0,
            stream,
        }
    }

    async fn exec(&mut self, data: &Proxy32Data) -> Option<Proxy32Data> {
        self.id += 1;
        let packet = Proxy32Packet {
            id: self.id,
            data: data.clone(),
        };
        self.stream.send(&packet).await;
        let res = match self.cached.get(&packet.id) {
            None => None,
            Some(x) => Some(x.clone()),
        };
        if let Some(data) = res {
            self.cached.remove(&packet.id);
            return Some(data);
        }
        loop {
            let res = self.stream.recv().await;
            match res {
                None => break None,
                Some(p) if p.id == packet.id => break Some(p.data),
                Some(p) => {
                    self.cached.insert(p.id, p.data);
                }
            }
        }
    }

    /**
     * 通知服务器端退出程序。
     * */
    pub async fn quit(&mut self) {
        self.exec(&Proxy32Data::QUIT).await;
    }
}
