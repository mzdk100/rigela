/*
 * Copyright (c) 2023. The RigelA open source project team and
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

pub(crate) mod sound;
pub(crate) mod tts;

use crate::context::Context;
use crate::performer::sound::Sound;
use crate::performer::tts::Tts;
use rigela_utils::bass::BassChannelOutputStream;
use std::sync::{Arc, OnceLock};

const SAMPLE_RATE: u32 = 16000;
const NUM_CHANNELS: u32 = 1;

/// 表演者语音信息收集接口。 实现此接口的对象可以调用表演者的speak方法进行输出。
pub(crate) trait Speakable {
    fn get_sentence(&self) -> String;
}

/// 表演者对象结构。 可以进行语音输出或音效提示。
#[derive(Debug)]
pub(crate) struct Performer {
    pub(crate) tts: OnceLock<Arc<Tts>>,
    pub(crate) output_stream: Arc<BassChannelOutputStream>,
    context: OnceLock<Arc<Context>>,
    sound: Arc<Sound>,
}

impl Performer {
    /// 创建表演者对象。
    pub(crate) fn new() -> Self {
        Self {
            tts: OnceLock::new(),
            output_stream: BassChannelOutputStream::new(SAMPLE_RATE, NUM_CHANNELS).into(),
            context: OnceLock::new(),
            sound: Sound::new().into(),
        }
    }

    ///  配置表演者。
    pub(crate) async fn apply(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap_or(());

        let tts = Tts::build(context.clone()).await;
        self.tts.set(tts.into()).unwrap();

        self.sound.apply(context.clone()).await;
    }

    pub(crate) fn speak(&self, speakable: impl Speakable) {
        let text = speakable.get_sentence();
        let tts = self.tts.get().unwrap().clone();

        self.context.get().unwrap().main_handler.spawn(async move {
            tts.speak(text).await;
        });
    }

    /// 播放音效
    pub(crate) async fn play_sound(&self, res_name: &str) {
        self.sound.play(res_name).await;
    }
}
