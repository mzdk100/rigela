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

use png::{AdaptiveFilterType, BitDepth, ColorType, Compression, Encoder};
use std::io::Write;
use win_wrap::{
    common::HWND,
    ext::ToBytesExt,
    graphic::{
        bit_blt, create_compatible_bitmap, create_compatible_dc, delete_dc, delete_object, get_dc,
        get_di_bits, release_dc, select_object, BITMAPFILEHEADER, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
    },
};

/**
 * 图像压缩格式。
 * */
pub enum ImageCompressionFormat {
    BMP,
    PNG,
}

/**
 * 屏幕截图，返回未压缩的位图数据。
 * `h_wnd` 窗口句柄，0表示整个屏幕。
 * `left` 要截取的左上角x坐标。
 * `top` 要截取的左上角y坐标。
 * `width` 要截取的宽度。
 * `height` 要截取的高度。
 * */
pub fn snapshot(
    h_wnd: HWND,
    left: i32,
    top: i32,
    with: i32,
    height: i32,
) -> Option<(Vec<Vec<RGBQUAD>>, BITMAPINFOHEADER, Option<Vec<RGBQUAD>>)> {
    let h_dc = get_dc(h_wnd);
    if h_dc.is_invalid() {
        return None;
    }
    let h_mem_dc = create_compatible_dc(h_dc);
    if h_mem_dc.is_invalid() {
        release_dc(h_wnd, h_dc);
        return None;
    }
    let h_bm = create_compatible_bitmap(h_dc, with, height);
    if h_bm.is_invalid() {
        delete_dc(h_mem_dc);
        release_dc(h_wnd, h_dc);
    }
    let h_old_obj = select_object(h_mem_dc, h_bm.into());
    let res = bit_blt(h_mem_dc, 0, 0, with, height, h_dc, left, top, SRCCOPY);
    let data = if res {
        let (pixels, mut bm_header, color_table) =
            get_di_bits(h_dc, h_bm, 0, height as u32, DIB_RGB_COLORS);
        bm_header.biCompression = BI_RGB.0;
        Some((pixels, bm_header, color_table))
    } else {
        None
    };
    if !h_old_obj.is_invalid() {
        select_object(h_mem_dc, h_old_obj);
    }
    delete_object(h_bm.into());
    delete_dc(h_mem_dc);
    release_dc(h_wnd, h_dc);
    data
}

/**
 * 屏幕截图，返回二进制bytes数据。
 * `h_wnd` 窗口句柄，0表示整个屏幕。
 * `left` 要截取的左上角x坐标。
 * `top` 要截取的左上角y坐标。
 * `width` 要截取的宽度。
 * `height` 要截取的高度。
 * `format` 图像格式。
 * */
pub fn snapshot_bytes(
    h_wnd: HWND,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    format: ImageCompressionFormat,
) -> Option<Vec<u8>> {
    let Some((mut pixels, bm_header, color_table)) = snapshot(h_wnd, left, top, width, height)
        else {
            return None;
        };
    match format {
        ImageCompressionFormat::BMP => {
            let offset = bm_header.biSize
                + std::mem::size_of::<BITMAPFILEHEADER>() as u32
                + bm_header.biClrUsed * std::mem::size_of::<RGBQUAD>() as u32;
            let header = BITMAPFILEHEADER {
                bfType: 0x4d42,
                bfSize: bm_header.biSizeImage + offset,
                bfReserved1: 0,
                bfReserved2: 0,
                bfOffBits: offset,
            };
            let mut v = Vec::<u8>::with_capacity((bm_header.biSize + offset) as usize);
            v.write(header.to_bytes()).unwrap();
            v.write(bm_header.to_bytes()).unwrap();
            if let Some(color_table) = color_table {
                for i in color_table.iter() {
                    v.write(i.to_bytes()).unwrap();
                }
            }
            for i in pixels.iter() {
                for j in i.iter() {
                    v.write(j.to_bytes()).unwrap();
                }
            }
            Some(v)
        }
        ImageCompressionFormat::PNG => {
            let mut v2 = Vec::with_capacity((bm_header.biSize + bm_header.biSizeImage) as usize);
            let mut encoder =
                Encoder::new(&mut v2, bm_header.biWidth as u32, bm_header.biHeight as u32);
            encoder.set_color(ColorType::Rgba);
            encoder.set_depth(BitDepth::Eight);
            encoder.set_compression(Compression::Best);
            encoder.set_adaptive_filter(AdaptiveFilterType::Adaptive);
            if let Ok(mut w) = encoder.write_header() {
                let mut buf = Vec::with_capacity(bm_header.biSizeImage as usize);
                if bm_header.biHeight > 0 {
                    // 高度如果是正数则图像是倒立的，需要翻转
                    pixels.reverse();
                }
                for i in pixels.iter() {
                    for j in i.iter() {
                        buf.write(j.to_bytes()).unwrap();
                    }
                }
                w.write_image_data(&buf).unwrap();
                w.finish().unwrap();
            }
            Some(v2)
        }
    }
}

#[cfg(test)]
mod test_screen {
    use std::{fs::OpenOptions, io::Write};

    use win_wrap::common::get_desktop_window;

    use crate::screen::{snapshot_bytes, ImageCompressionFormat};

    #[test]
    fn main() {
        let Some(data) = snapshot_bytes(
            get_desktop_window(),
            20,
            20,
            1300,
            620,
            ImageCompressionFormat::PNG,
        ) else {
            return;
        };
        OpenOptions::new()
            .create(true)
            .write(true)
            .open("screen.png")
            .unwrap()
            .write(&data)
            .unwrap();
    }
}
