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
