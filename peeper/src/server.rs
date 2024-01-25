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

use crate::{
    model::{PeeperData, PeeperPacket},
    utils::get_pipe_name,
};
use log::{error, info};
use rigela_utils::pipe::{PipeStream, PipeStreamError};
use std::sync::Arc;
use tokio::{
    net::windows::named_pipe::{NamedPipeServer, ServerOptions},
    runtime::{Builder, Runtime},
    sync::Mutex,
};
use crate::model::CandidateList;

enum ListenerType {
    OnInputChar(Box<dyn Fn(u16) + Send + Sync>),
    OnImeCandidateList(Box<dyn Fn(CandidateList) + Send + Sync>),
}

pub struct PeeperServer {
    listeners: Arc<Mutex<Vec<ListenerType>>>,
    rt: Runtime,
}

impl PeeperServer {
    pub fn new() -> Self {
        let rt = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .build()
            .unwrap();
        Self {
            listeners: Arc::new(vec![].into()),
            rt,
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
                    info!("New client has connected.");
                    let stream = PipeStream::new(p);
                    self.on_client(stream);
                }
                Err(e) => {
                    error!("{}", e);
                    break;
                }
            }
        }
    }

    fn on_client(&self, mut stream: PipeStream<PeeperPacket, NamedPipeServer>) {
        let listeners = self.listeners.clone();
        self.rt.spawn(async move {
            loop {
                let packet = stream.recv().await;
                if let Err(PipeStreamError::ReadEof) = &packet {
                    break;
                }
                if let Err(PipeStreamError::DecodeError(e)) = &packet {
                    error!("{}", e);
                    continue;
                }
                let packet = packet.unwrap();
                match packet.data {
                    PeeperData::Log(msg) => info!("{}: {}", packet.name, msg),
                    PeeperData::Quit => break,
                    _ => Self::call(listeners.clone(), packet.data).await
                };
            }
        });
    }

    async fn call(listeners: Arc<Mutex<Vec<ListenerType>>>, data: PeeperData) {
        let listeners = listeners.lock().await;
        for i in listeners.iter() {
            match i {
                ListenerType::OnInputChar(f) => match data {
                    PeeperData::InputChar(c) => (&*f) (c),
                    _       => {}
                },
                ListenerType::OnImeCandidateList(f) => match &data {
                    PeeperData::ImeCandidateList(c) => (&*f) (c.clone()),
                    _       => {}
                },
            }
        }
    }

    /**
     * 添加一个监听器，当用户输入内容到控件上时发出通知。
     * `listener` 一个监听函数。
     * */
    pub async fn add_on_input_char_listener(&self, listener: impl Fn(u16) + Send + Sync + 'static) {
        let mut listeners = self.listeners.lock().await;
        listeners.push(ListenerType::OnInputChar(Box::new(listener)));
    }

    /**
     * 添加一个监听器，当输入法候选列表呈现或改变时发出通知。
     * `listener` 一个监听函数。
     * */
    pub async fn add_on_ime_candidate_list_listener(&self, listener: impl Fn(CandidateList) + Send + Sync + 'static) {
        let mut listeners = self.listeners.lock().await;
        listeners.push(ListenerType::OnImeCandidateList(Box::new(listener)));
    }
}
