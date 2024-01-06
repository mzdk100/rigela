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

use std::future::Future;
use std::sync::Arc;
use win_wrap::tts::Tts;
use crate::configs::tts::TtsConfig;
use crate::context::Context;


/**
 * 表演者语音信息收集接口。
 * 实现此接口的对象可以调用表演者的speak方法进行输出。
 * */
pub(crate) trait Speakable {
    fn get_sentence(&self) -> String;
}

/**
 * 表演者对象结构。
 * 可以进行语音输出或音效提示。
 * */
#[derive(Clone)]
pub(crate) struct Performer {
    tts: Arc<Tts>
}
impl Performer {
    /**
     * 创建表演者对象。
     * */
    pub(crate) fn new() -> Self {
        let tts = Tts::new();
        Self {
            tts: tts.into()
        }
    }

    /**
     * 设置表演者的参数。
     * `context` 框架的上下文环境。
     * `slot` 一个用于修改参数的函数或闭包。
     * */
    pub(crate) fn apply_config(&self, context: Arc<Context>, slot: impl  FnOnce(&mut TtsConfig) + Send + Sync + 'static) {
        let main_handler = context.main_handler.clone();
        let tts = self.tts.clone();
        main_handler.spawn(async move {
            let mut config = context
                .config_manager
                .read()
                .await;
            let mut tts_config = config
                .tts_config
                .clone()
                .unwrap_or(TtsConfig::default());
            slot(&mut tts_config);
            let speed = tts_config.speed.clone().unwrap();
            tts.set_speed(speed);
            config.tts_config.replace(tts_config);
            context
                .config_manager
                .write(&config)
                .await;
        });
    }

    /**
     * 使用语音输出，播报对象的信息。
     * */
    pub(crate) fn speak<'a>(&'a self, speakable: &'a (dyn Speakable + Sync)) -> impl Future<Output = ()> + 'a {
        async {
            self
                .tts
                .speak(speakable.get_sentence().as_str())
                .await
                .unwrap();
        }
    }
    pub(crate) fn speak_text<'a>(&'a self, text: &'a str) -> impl Future<Output = ()> + 'a {
        async { self
            .tts
            .speak(text)
            .await
            .unwrap();
        }
    }
}
