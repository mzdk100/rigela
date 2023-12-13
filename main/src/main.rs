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

#![windows_subsystem = "windows"]

use windows::core::{h, Result};
use windows::Media::{
    Core::MediaSource,
    Playback::{MediaPlaybackItem, MediaPlayer},
    SpeechSynthesis::SpeechSynthesizer,
};
#[link(name = "peeper.dll", kind = "static")]
extern "C" {
    fn add(left: usize, right: usize) -> usize;
}
fn main() -> Result<()> {
    unsafe {
        println!("Hello, {}!", add(3, 4));
    }
    futures::executor::block_on(main_async())
}
async fn main_async() -> Result<()> {
    let player = MediaPlayer::new()?;
    let synth = SpeechSynthesizer::new()?;
    let stream = synth.SynthesizeTextToStreamAsync(h!("你好"))?.await?;
    let source = MediaSource::CreateFromStream(&stream, &stream.ContentType()?)?;
    let item = MediaPlaybackItem::Create(&source)?;
    player.SetSource(&item)?;
    player.Play()?;
    Ok(())
}
