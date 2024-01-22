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


use std::{
    sync::Arc
};
use log::error;
use tokio::{
    runtime::{Builder, Runtime},
    net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    sync::Mutex
};
use rigela_utils::pipe::PipeStream;
use crate::model::{PeeperData, PeeperPacket};
use crate::utils::get_pipe_name;

type OnInputCharListener = dyn Fn(u16) + Send + Sync;

pub struct PeeperServer {
    on_input_char: Arc<Mutex<Vec<Box<OnInputCharListener>>>>,
    rt: Runtime
}

impl PeeperServer {
    pub fn new() -> Self {
        let rt = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .build()
            .unwrap();
        Self {
            on_input_char: Arc::new(vec![].into()),
            rt
        }
    }

    pub async fn run(&self) {
        loop {
            match ServerOptions::new().create(get_pipe_name()) {
                Ok(p) => {
                    if let Err(e) = p.connect().await {
                        error!("{}", e);
                        continue;
                    }
                    let stream = PipeStream::new(p);
                    self.on_client(stream);
                },
                Err(e) => {
                    error!("{}", e);
                    break
                }
            }
        }
    }

    fn on_client(&self, mut stream: PipeStream<PeeperPacket, NamedPipeServer>) {
        let on_input_char_listeners = self.on_input_char.clone();
        self.rt.spawn(async move {
            loop {
                let packet = stream.recv().await;
                if packet.is_none() {
                    break
                }
                let packet = packet.unwrap();
                match packet.data {
                    PeeperData::InputChar(c) => {
                        let listeners = on_input_char_listeners.lock().await;
                        for i in listeners.iter() {
                            let func = &*i;
                            func(c);
                        }
                    }
                };
            }
        });
    }

    pub async fn add_on_input_char_listener(&self, listener: impl Fn(u16) + Send + Sync + 'static) {
        let mut listeners = self.on_input_char.lock().await;
        listeners.push(Box::new(listener));
    }
}