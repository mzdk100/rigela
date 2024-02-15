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

use crate::{
    context::Context,
    performer::{
        sound::Sound,
        tts::{sapi5::Sapi5Engine, vvtts::VvttsEngine, Tts},
    },
};
use std::sync::Arc;
use tokio::sync::OnceCell;

/// 表演者语音信息收集接口。 实现此接口的对象可以调用表演者的speak方法进行输出。
pub(crate) trait Speakable {
    fn get_sentence(&self) -> String;
}

/// 表演者对象结构。 可以进行语音输出或音效提示。
#[derive(Debug)]
pub(crate) struct Performer {
    tts: OnceCell<Arc<Tts>>,
    sound: Arc<Sound>,
}

impl Performer {
    /// 创建表演者对象。
    pub(crate) fn new() -> Self {
        Self {
            tts: OnceCell::new().into(),
            sound: Sound::new().into(),
        }
    }

    //noinspection StructuralWrap
    /**
     * 配置表演者。
     * `context` 读屏框架的上下文环境。
     * */
    pub(crate) async fn apply(&self, context: Arc<Context>) {
        let tts = Arc::new(Tts::new(context.clone()));
        self.tts.set(tts.clone()).unwrap_or(());
        tts.put_default_engine(Sapi5Engine::new())
            .await
            .add_engine(VvttsEngine::new(context.clone()))
            .await;
        self.sound.apply(context.clone()).await;
    }

    pub(crate) fn get_tts(&self) -> Arc<Tts> {
        self.tts.get().unwrap().clone()
    }

    pub(crate) async fn speak(&self, speakable: impl Speakable) {
        let text = speakable.get_sentence().trim_end().to_string();
        if text.is_empty() {
            return;
        }

        if let Some(tts) = self.tts.get() {
            tts.stop().await;
            tts.speak(text).await;
        }
    }

    /// 播放音效
    pub(crate) async fn play_sound(&self, res_name: &str) {
        self.sound.play(res_name).await;
    }
}
