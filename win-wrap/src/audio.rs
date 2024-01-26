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
    fmt::{Debug, Formatter},
    future::Future,
    mem::size_of,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use windows::Win32::Media::Audio::{
    AudioCategory_SoundEffects,
    XAudio2::XAUDIO2_VOICE_STATE,
    XAudio2::{
        IXAudio2, IXAudio2MasteringVoice, IXAudio2SourceVoice, XAudio2CreateWithVersionInfo,
        XAUDIO2_BUFFER, XAUDIO2_COMMIT_NOW, XAUDIO2_MAX_FREQ_RATIO, XAUDIO2_USE_DEFAULT_PROCESSOR,
        XAUDIO2_VOICE_NOSRC,
    },
    WAVEFORMATEX, WAVE_FORMAT_PCM,
};

#[allow(dead_code)]
pub struct AudioOutputStream {
    engine: Arc<IXAudio2>,
    mastering_voice: Arc<IXAudio2MasteringVoice>,
    source_voice: Arc<IXAudio2SourceVoice>,
}

impl AudioOutputStream {
    /**
     * 创建一个音频输出流。
     * `sample_rate` 采样率。
     * `num_channels` 通道数。
     * */
    pub fn new(sample_rate: u32, num_channels: u32) -> Self {
        let mut engine: Option<IXAudio2> = None;
        let mut mastering_voice: Option<IXAudio2MasteringVoice> = None;
        let mut source_voice: Option<IXAudio2SourceVoice> = None;
        unsafe { XAudio2CreateWithVersionInfo(&mut engine, 0, XAUDIO2_USE_DEFAULT_PROCESSOR, 2) }
            .expect("Can't create the XAudio engine.");
        if let Some(x) = engine.as_ref() {
            unsafe {
                x.CreateMasteringVoice(
                    &mut mastering_voice,
                    num_channels,
                    sample_rate,
                    0,
                    None,
                    None,
                    AudioCategory_SoundEffects,
                )
            }
            .expect("Can't create the mastering voice.");
            let block_align = num_channels * 2; // 每个样本的字节数
            let format = WAVEFORMATEX {
                cbSize: size_of::<WAVEFORMATEX>() as u16,
                nChannels: num_channels as u16,
                nSamplesPerSec: sample_rate,
                nBlockAlign: block_align as u16,
                nAvgBytesPerSec: sample_rate * block_align,
                wBitsPerSample: (block_align * 8) as u16,
                wFormatTag: WAVE_FORMAT_PCM as u16,
            };
            unsafe {
                x.CreateSourceVoice(
                    &mut source_voice,
                    &format,
                    XAUDIO2_VOICE_NOSRC,
                    XAUDIO2_MAX_FREQ_RATIO,
                    None,
                    None,
                    None,
                )
            }
            .expect("Can't create the source voice.");
        }
        Self {
            engine: engine.unwrap().into(),
            mastering_voice: mastering_voice.unwrap().into(),
            source_voice: source_voice.unwrap().into(),
        }
    }

    /**
     * 写入音频数据，并等待播放完毕。
     * `data` 音频数据。
     * */
    pub async fn write(&self, data: &[u8]) {
        StreamState::new(self.source_voice.clone(), &data).await;
    }

    /**
     * 从语音队列中删除所有挂起的音频缓冲区。
     * */
    #[allow(dead_code)]
    pub fn flush(&self) {
        unsafe { self.source_voice.FlushSourceBuffers() }.unwrap_or(())
    }

    /**
     * 停止播放。
     * */
    pub fn stop(&self) {
        unsafe { self.source_voice.Stop(0, 0) }.expect("Can't stop the stream.");
    }

    /**
     * 开始播放。
     * */
    pub fn start(&self) {
        unsafe { self.source_voice.Start(0, XAUDIO2_COMMIT_NOW) }.expect("Can't start.");
    }
}
impl Debug for AudioOutputStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AudioOutputStream")
    }
}
unsafe impl Send for AudioOutputStream {}
unsafe impl Sync for AudioOutputStream {}
pub struct StreamState(Arc<IXAudio2SourceVoice>);
impl StreamState {
    fn new(source_voice: Arc<IXAudio2SourceVoice>, data: &[u8]) -> Self {
        let mut buf = XAUDIO2_BUFFER::default();
        buf.pAudioData = data.as_ptr();
        buf.AudioBytes = data.len() as u32;
        unsafe { source_voice.SubmitSourceBuffer(&buf, None) }.unwrap_or(());
        Self(source_voice)
    }
}
impl Future for StreamState {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = XAUDIO2_VOICE_STATE::default();
        unsafe {
            self.0.GetState(&mut state, 0);
        }
        let p = state.BuffersQueued;
        if p < 1 {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
unsafe impl Send for StreamState {}
unsafe impl Sync for StreamState {}
