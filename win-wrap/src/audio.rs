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

use std::fmt::{Display, Formatter};
use std::mem::size_of;
use windows::Win32::Media::Audio::XAudio2::{
    IXAudio2SourceVoice, XAUDIO2_BUFFER, XAUDIO2_MAX_FREQ_RATIO, XAUDIO2_VOICE_NOSRC,
};
use windows::Win32::Media::Audio::{
    AudioCategory_SoundEffects,
    XAudio2::{
        IXAudio2, IXAudio2MasteringVoice, XAudio2CreateWithVersionInfo,
        XAUDIO2_USE_DEFAULT_PROCESSOR,
    },
    WAVEFORMATEX, WAVE_FORMAT_PCM,
};

#[allow(dead_code)]
pub struct AudioOutputStream {
    engine: IXAudio2,
    mastering_voice: IXAudio2MasteringVoice,
    source_voice: IXAudio2SourceVoice,
}

impl AudioOutputStream {
    /**
     * 创建一个音频输出流。
     * */
    pub fn new(num_channels: u32, sample_rate: u32) -> Self {
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
            engine: engine.unwrap(),
            mastering_voice: mastering_voice.unwrap(),
            source_voice: source_voice.unwrap(),
        }
    }
    pub fn push(&self) {
        unsafe { self.source_voice.Start(0, 0) }.expect("Can't start.");
        let mut data: [u8; 32000] = [0; 32000];
        for i in 0..data.len() {
            data[i] = ((i as f64).sin() * 32f64) as u8
        }
        let mut buf = XAUDIO2_BUFFER::default();
        buf.pAudioData = data.as_ptr();
        buf.AudioBytes = data.len() as u32;
        unsafe { self.source_voice.SubmitSourceBuffer(&buf, None) }
            .expect("Can't submit the data.");
    }
}

impl Display for AudioOutputStream {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AudioOutputStream")
    }
}
