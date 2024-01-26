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

#[cfg(target_arch = "x86")]
use std::sync::{Arc, OnceLock};
#[cfg(target_arch = "x86")]
use crate::model::{Proxy32Data, Proxy32Packet};
#[cfg(target_arch = "x86")]
use crate::tts::ibmeci::Ibmeci;
#[cfg(target_arch = "x86")]
use rigela_utils::pipe::{server_run, PipeStream, PipeStreamError};
#[cfg(target_arch = "x86")]
use tokio::net::windows::named_pipe::NamedPipeServer;

//noinspection SpellCheckingInspection
#[cfg(target_arch = "x86")]
pub struct Proxy32Server {
    stream: PipeStream<Proxy32Packet, NamedPipeServer>,
    ibmeci: Arc<OnceLock<Ibmeci>>
}

#[cfg(target_arch = "x86")]
impl Proxy32Server {
    /**
     * 创建一个proxy32的服务端实例。
     * `pipe_name` 管道名称，需要与客户端使用的名称相同。
     * */
    pub async fn new(pipe_name: &str) -> Self {
        let stream = server_run(pipe_name).await;
        Self {
            stream,
            ibmeci: OnceLock::new().into()
        }
    }

    /**
     * 运行proxy32服务端，这会创建一个循环，不断地接收来自客户端的命令，然后把处理结果返回。
     * */
    pub async fn run(&mut self) {
        loop {
            let packet = self.stream.recv().await;
            match packet {
                Err(PipeStreamError::ReadEof) => break,
                Ok(p) => {
                    let data = self.on_exec(&p.data).await;
                    let packet = Proxy32Packet { id: p.id, data };
                    self.stream.send(&packet).await;
                    if Proxy32Data::Quit == packet.data {
                        break;
                    }
                }
                _ => {}
            };
        }
    }

    async fn on_exec(&self, data: &Proxy32Data) -> Proxy32Data {
        match data {
            Proxy32Data::EciSynthRequest(text) => Proxy32Data::EciSynthResponse(self.eci_synth(text).await),
            _ => data.clone()
        }
    }

    #[cfg(target_arch = "x86")]
    async fn eci_synth(&self, text: &str) -> Vec<u8> {
        let eci = self.ibmeci.get();
        if eci.is_none() {
            use crate::tts::create_ibmeci;
            self.ibmeci.set(create_ibmeci().await).unwrap_or(());
        }
        eci.unwrap().synth(text).await
    }
}
