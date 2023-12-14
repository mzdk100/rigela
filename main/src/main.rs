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


use std::time::Duration;
use tokio::time;
use windows::core::{HSTRING, Result};
use windows::Media::{
    Core::MediaSource,
    Playback::{MediaPlaybackItem, MediaPlayer},
    SpeechSynthesis::SpeechSynthesizer,
};
async fn speak(text: &str, synth: &SpeechSynthesizer, player: &MediaPlayer) -> Result<()> {
    let stream = synth.SynthesizeTextToStreamAsync(&HSTRING::from(text))?.await?;
    let source = MediaSource::CreateFromStream(&stream, &stream.ContentType()?)?;
    let item = MediaPlaybackItem::Create(&source)?;
    player.SetSource(&item)?;
    player.Play()?;
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, {}!", peeper::add(3, 4));
    let player = MediaPlayer::new()?;
    let synth = SpeechSynthesizer::new()?;
    for i in 0..10 {
        speak(format!("{}", i).as_str(), &synth, &player).await?;
        time::sleep(Duration::from_millis(300)).await;
    }
    time::sleep(Duration::from_millis(1000)).await;
    Ok(())
}
