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
use windows::core::{HSTRING, Result};
use windows::Media::Core::MediaSource;
use windows::Media::Playback::{MediaPlaybackItem, MediaPlayer};
use windows::Media::SpeechSynthesis::SpeechSynthesizer;

pub struct Tts{
    synth: Arc<SpeechSynthesizer>,
    player: Arc<MediaPlayer>
}
impl Tts {
    /**
     * 创建一个TTS对象（语音合成，SAPI5）
     * */
    pub fn new() -> Self {
        // 创建语音合成器
        let synth = SpeechSynthesizer::new()
            .expect("Can't create the speech synthesizer.");
        // 创建媒体播放器
        let player = MediaPlayer::new()
            .expect("Can't create the media player.");
        Self {
            synth: Arc::new(synth),
            player: Arc::new(player)
        }
    }

    /**
     * 朗读一段文字（直接播放）
     * 此函数是异步函数，需要使用.await。
     * `text` 要朗读的文字。
     * */
    pub fn speak<'a>(&'a self, text: &'a str) -> impl Future<Output=Result<()>> + 'a {
        async move {
            let stream = self.synth.SynthesizeTextToStreamAsync(&HSTRING::from(text))?.await?;
            let source = MediaSource::CreateFromStream(&stream, &stream.ContentType()?)?;
            let item = MediaPlaybackItem::Create(&source)?;
            self.player.SetSource(&item)?;
            self.player.Play()?;
            Ok(())
        }
    }
}

impl Clone for Tts {
    fn clone(&self) -> Self {
        Self {
            synth: self.synth.clone(),
            player: self.player.clone()
        }
    }
}
