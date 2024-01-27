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

use std::io::prelude::Seek;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn resample_audio(
    f_in_ram: Vec<u8>,
    input_sample_rate: usize,
    output_sample_rate: usize,
) -> Vec<u8> {
    let file_size = f_in_ram.len();
    let ratio = output_sample_rate as f32 / input_sample_rate as f32;
    let mut f_out_ram: Vec<u8> = Vec::with_capacity(2 * (file_size as f32 * ratio) as usize);

    let mut f_in = Cursor::new(&f_in_ram);
    let mut f_out = Cursor::new(&mut f_out_ram);
    let mut i = 0f32;
    let mut j = 0f32;
    loop {
        match f_in.read_i16_le().await {
            Err(_) => break,
            Ok(r) => {
                let k = ratio * i;
                while k >= j {
                    f_out.write_i16_le(r).await.unwrap_or(());
                    j += 1f32;
                }
            }
        };
        i += 1f32;
    }

    f_out.seek(std::io::SeekFrom::Start(0)).unwrap();
    f_out_ram
}
