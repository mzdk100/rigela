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
use rigela_utils::{
    ibmeci::{
        Ibmeci, VP_BREATHINESS, VP_GENDER, VP_HEAD_SIZE, VP_PITCH_BASELINE, VP_PITCH_FLUCTUATION,
        VP_ROUGHNESS, VP_SPEED, VP_VOLUME,
    },
    pipe::{server_run, PipeStream, PipeStreamError},
};
use tokio::net::windows::named_pipe::NamedPipeServer;

//noinspection SpellCheckingInspection
pub struct Proxy32Server {
    stream: PipeStream<Proxy32Packet, NamedPipeServer>,
}

impl Proxy32Server {
    /**
     * 创建一个proxy32的服务端实例。
     * `pipe_name` 管道名称，需要与客户端使用的名称相同。
     * */
    pub async fn new(pipe_name: &str) -> Self {
        let stream = server_run(pipe_name).await;
        Self { stream }
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
                    if let Err(e) = self.stream.send(&packet).await {
                        error!("{}", e);
                    }
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
            Proxy32Data::EciSynthRequest(text) => {
                Proxy32Data::EciSynthResponse(self.eci_synth(text).await)
            }
            Proxy32Data::EciSetParamsRequest(params) => {
                Proxy32Data::EciSetParamsResponse(self.eci_set_voice_params(params).await)
            }
            Proxy32Data::EciGetParamsRequest => {
                Proxy32Data::EciGetParamsResponse(self.eci_get_voice_params().await)
            }
            Proxy32Data::EciSetVoiceRequest(v) => {
                Proxy32Data::EciSetVoiceResponse(self.eci_set_voice(v.clone()).await)
            }
            Proxy32Data::EciGetVoicesRequest => {
                Proxy32Data::EciGetVoicesResponse(self.eci_get_voices().await)
            }
            _ => data.clone(),
        }
    }

    async fn eci_synth(&self, text: &str) -> Vec<u8> {
        let eci = Ibmeci::get().await.unwrap();
        eci.synth(text).await
    }

    async fn eci_set_voice_params(&self, params: &IbmeciVoiceParams) {
        let eci = Ibmeci::get().await.unwrap();
        eci.set_voice_param(VP_GENDER, params.gender);
        eci.set_voice_param(VP_VOLUME, params.volume);
        eci.set_voice_param(VP_SPEED, params.speed);
        eci.set_voice_param(VP_PITCH_FLUCTUATION, params.pitch_fluctuation);
        eci.set_voice_param(VP_PITCH_BASELINE, params.pitch_baseline);
        eci.set_voice_param(VP_BREATHINESS, params.breathiness);
        eci.set_voice_param(VP_HEAD_SIZE, params.head_size);
        eci.set_voice_param(VP_ROUGHNESS, params.roughness);
    }

    async fn eci_get_voice_params(&self) -> IbmeciVoiceParams {
        let eci = Ibmeci::get().await.unwrap();
        IbmeciVoiceParams {
            gender: eci.get_voice_param(VP_GENDER),
            head_size: eci.get_voice_param(VP_HEAD_SIZE),
            pitch_baseline: eci.get_voice_param(VP_PITCH_BASELINE),
            pitch_fluctuation: eci.get_voice_param(VP_PITCH_FLUCTUATION),
            roughness: eci.get_voice_param(VP_ROUGHNESS),
            breathiness: eci.get_voice_param(VP_BREATHINESS),
            speed: eci.get_voice_param(VP_SPEED),
            volume: eci.get_voice_param(VP_VOLUME),
        }
    }

    async fn eci_get_voices(&self) -> Vec<(u32, String)> {
        let eci = Ibmeci::get().await.unwrap();
        eci.get_voices()
    }

    async fn eci_set_voice(&self, voice_id: u32) {
        let eci = Ibmeci::get().await.unwrap();
        eci.set_voice(voice_id)
    }
}
