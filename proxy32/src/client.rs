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

use crate::model::{IbmeciVoiceParams, Proxy32Data, Proxy32Packet};
use log::error;
use rigela_utils::pipe::{client_connect, PipeStream, PipeStreamError};
use std::collections::HashMap;
use tokio::net::windows::named_pipe::NamedPipeClient;

#[derive(Debug)]
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
        if let Err(e) = self.stream.send(&packet).await {
            error!("{}", e);
        }
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
                Err(PipeStreamError::ReadEof) => return None,
                Ok(p) if p.id == packet.id => break Some(p.data),
                Ok(p) => {
                    self.cached.insert(p.id, p.data);
                }
                _ => {}
            }
        }
    }

    /**
     * 通知服务器端退出程序。
     * */
    pub async fn quit(&mut self) {
        self.exec(&Proxy32Data::Quit).await;
    }

    //noinspection SpellCheckingInspection
    /**
     * 使用vvtts合成语音。
     * `text` 文字内容。
     * */
    pub async fn eci_synth(&mut self, text: &str) -> Vec<u8> {
        if let Some(Proxy32Data::EciSynthResponse(r)) = self
            .exec(&Proxy32Data::EciSynthRequest(text.to_string()))
            .await
        {
            r
        } else {
            vec![]
        }
    }

    //noinspection SpellCheckingInspection
    /**
     * 设置vvtts语音参数。
     * `params` 参数数据。
     * */
    pub async fn eci_set_voice_params(&mut self, params: &IbmeciVoiceParams) {
        self.exec(&Proxy32Data::EciSetParamsRequest(params.clone()))
            .await;
    }

    //noinspection SpellCheckingInspection
    /**
     * 获取vvtts语音参数。
     * */
    pub async fn eci_get_voice_params(&mut self) -> IbmeciVoiceParams {
        if let Some(Proxy32Data::EciGetParamsResponse(r)) =
            self.exec(&Proxy32Data::EciGetParamsRequest).await
        {
            return r;
        }
        IbmeciVoiceParams::default()
    }

    //noinspection SpellCheckingInspection
    /**
     * 获取vvtts发音人列表。
     * */
    pub async fn eci_get_voices(&mut self) -> Vec<(u32, String)> {
        if let Some(Proxy32Data::EciGetVoicesResponse(r)) =
            self.exec(&Proxy32Data::EciGetVoicesRequest).await
        {
            return r;
        }
        vec![]
    }

    //noinspection SpellCheckingInspection
    /**
     * 设置vvtts发音人。
     * `voice_id` 发音人id。
     * */
    pub async fn eci_set_voice(&mut self, voice_id: u32) {
        self.exec(&Proxy32Data::EciSetVoiceRequest(voice_id)).await;
    }
}
