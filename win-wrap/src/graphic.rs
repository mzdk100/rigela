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

pub use windows::Win32::Graphics::Gdi::{
    BITMAPFILEHEADER, BITMAPINFO, BITMAPINFOHEADER, BI_BITFIELDS, BI_COMPRESSION, BI_JPEG, BI_PNG,
    BI_RGB, BI_RLE4, BI_RLE8, BLACKNESS, CAPTUREBLT, COMPLEXREGION, DIB_PAL_COLORS, DIB_RGB_COLORS,
    DIB_USAGE, DSTINVERT, HBITMAP, HDC, HGDIOBJ, MERGECOPY, MERGEPAINT, NOMIRRORBITMAP, NOTSRCCOPY,
    NOTSRCERASE, NULLREGION, PATCOPY, PATINVERT, PATPAINT, RGBQUAD, ROP_CODE, SIMPLEREGION, SRCAND,
    SRCCOPY, SRCERASE, SRCINVERT, SRCPAINT, WHITENESS,
};
use windows::{
    core::imp::{heap_alloc, heap_free},
    Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, ReleaseDC, SelectObject,
    },
};

use crate::{common::HWND, ext::ToBytesExt};

impl ToBytesExt for BITMAPFILEHEADER {
    type Input = Self;
}

impl ToBytesExt for BITMAPINFOHEADER {
    type Input = Self;
}

impl ToBytesExt for RGBQUAD {
    type Input = Self;
}

/**
 * get_dc函数查询指定窗口的工作区或整个屏幕的设备上下文(DC)的句柄。可以在后续GDI函数中使用返回的句柄在 DC 中绘制。 设备上下文是一种不透明的数据结构，其值由 GDI 在内部使用。
 * get_dc_ex 函数是 get_dc 的扩展，它使应用程序能够更好地控制在工作区中发生剪裁的方式和是否发生。
 * 如果函数成功，则返回值是指定窗口工作区的 DC 的句柄。
 * 如果函数失败，则返回值为 NULL。
 * get_dc函数根据指定窗口的类样式检索公共、类或专用DC。对于类和专用DC，get_dc保留以前分配的属性不变。但是，对于常见的DC，get_dc在每次检索DC时都会将默认属性分配给DC。例如，默认字体为System，即位图字体。因此，get_dc返回的通用DC的句柄不会告诉你在绘制窗口时使用了哪种字体、颜色或画笔。若要确定字体，请调用get_text_face。
 * 请注意，DC 的句柄一次只能由单个线程使用。使用通用 DC 进行绘制后，必须调用 release_dc 函数来释放 DC。 类和专用 DC 不必释放。 release_dc 必须从调用 get_dc 的同一线程调用。 DC 的数量仅受可用内存的限制。
 * */
pub fn get_dc(h_wnd: HWND) -> HDC {
    unsafe { GetDC(h_wnd) }
}

/**
 * create_compatible_dc 函数创建与指定设备兼容的内存设备上下文 （DC）。
 * 如果函数成功，则返回值是内存 DC 的句柄。
 * 如果函数失败，则返回值为 NULL。
 * 内存DC仅存在于内存中。创建内存DC时，其显示表面正好是一个单色像素宽和一个单色像素高。在应用程序可以使用内存DC进行绘图操作之前，它必须在DC中选择正确宽度和高度的位图。若要在DC中选择位图，请使用create_compatible_bitmap函数，指定所需的高度、宽度和颜色组织。
 * 创建内存 DC 时，所有属性都设置为正常默认值。存储器 DC 可以用作普通 DC。您可以设置属性;获取其属性的当前设置，然后选择钢笔、画笔和区域。
 * create_compatible_dc 函数只能用于支持栅格操作的设备。应用程序可以通过调用 get_device_caps 函数来确定设备是否支持这些操作。
 * 当您不再需要内存 DC 时，请调用 delete_dc 函数。建议您调用 delete_dc 删除 DC。但是，您也可以使用 HDC 调用 delete_object 来删除 DC。
 * 如果 hdc 为 NULL，则调用 create_compatible_dc 的线程拥有创建的 HDC。当此线程被销毁时，HDC 将不再有效。因此，如果创建 HDC 并将其传递给另一个线程，然后退出第一个线程，则第二个线程将无法使用 HDC。
 * 国际化学（ICM）：如果为图像颜色管理 （ICM） 启用了传递给此函数的 DC，则该函数创建的 DC 将启用 ICM。源和目标颜色空间在 DC 中指定。
 * `h_dc` 现有 DC 的句柄。如果此句柄为 NULL，则该函数将创建与应用程序当前屏幕兼容的内存 DC。
 * */
pub fn create_compatible_dc(h_dc: HDC) -> HDC {
    unsafe { CreateCompatibleDC(h_dc) }
}

/**
 * create_compatible_bitmap 函数创建与设备兼容的位图，该位图与指定的设备上下文相关联。
 * 如果函数成功，则返回值是兼容位图 (DDB) 的句柄。
 * 如果函数失败，则返回值为 NULL。
 * create_compatible_bitmap 函数创建的位图的颜色格式与 h_dc 参数标识的设备的颜色格式匹配。 可以在与原始设备兼容的任何内存设备上下文中选择此位图。
 * 由于内存设备上下文同时允许彩色位图和单色位图，因此当指定的设备上下文为内存设备上下文时， create_compatible_bitmap 函数返回的位图格式会有所不同。 但是，为非内存设备上下文创建的兼容位图始终具有相同的颜色格式，并使用与指定设备上下文相同的调色板。
 * 注意：创建内存设备上下文时，它最初会选择一个 1 乘 1 的单色位图。 如果在 create_compatible_bitmap 中使用此内存设备上下文，则创建的位图是 单色 位图。 若要创建颜色位图，请使用用于创建内存设备上下文的 HDC ，如以下代码所示：
 * let mem_dc = create_compatible_dc ( h_dc );
 * let mem_bm = create_compatible_bitmap ( h_dc, n_width, n_height );
 * select_object ( mem_dc, mem_bm );
 * 如果应用程序将 n_width 或 n_height 参数设置为零， create_compatible_bitmap 会将句柄返回到 1 x 1 像素的单色位图。
 * 如果将 DIB 节（由 create_dib_section 函数创建的位图）选入 h_dc 参数标识的设备上下文中， 则 create_compatible_bitmap 将创建 DIB 节。
 * 如果不再需要位图，请调用 delete_object 函数将其删除。
 * `h_dc` 设备上下文的句柄。
 * `cx` 位图宽度（以像素为单位）。
 * `cy` 位图高度（以像素为单位）。
 * */
pub fn create_compatible_bitmap(h_dc: HDC, cx: i32, cy: i32) -> HBITMAP {
    unsafe { CreateCompatibleBitmap(h_dc, cx, cy) }
}

/**
 * select_object 函数在指定设备上下文(DC)中选择对象。 新 对象替换同一类型的上一个对象。
 * 如果所选对象不是区域且函数成功，则返回值是所替换对象的句柄。 如果所选对象是区域且函数成功，则返回值是以下值之一。
 * SIMPLEREGION | 区域由单个矩形组成。
 * COMPLEXREGION | 区域由多个矩形组成。
 * NULLREGION | 区域为空。
 * 如果发生错误，并且所选对象不是区域，则返回值为 NULL。 否则，它将发生HGDI_ERROR。
 * 此函数返回指定类型的以前选择的对象。 应用程序在用新对象完成绘图后，应始终将新对象替换为原始的默认对象。
 * 应用程序不能一次选择一个位图进入多个 DC。
 * Icm： 如果要选择的对象是画笔或笔，则执行颜色管理。
 * `h_dc` DC 的句柄。
 * `h` 要选择的对象的句柄。 指定的对象必须已使用以下函数之一创建。
 * Bitmap | create_bitmap、 create_bitmap_indirect、 create_compatible_bitmap、 create_dib_itmap、 create_dib_section | 位图只能选择到内存 DC 中。 不能将单个位图同时选入多个 DC。
 * Brush | create_brush_indirect、 create_dib_pattern_brush、 create_dib_pattern_brush_pt、 create_hatch_brush、 create_pattern_brush、 create_solid_brush |
 * Font | create_font、 create_font_indirect |
 * Pen | create_pen、 create_pen_indirect |
 * Region | combine_rgn、 create_elliptic_rgn、 create_elliptic_rgn_indirect、 create_polygon_rgn、 create_rect_rgn、 create_rect_rgn_indirect |
 * */
pub fn select_object(h_dc: HDC, h: HGDIOBJ) -> HGDIOBJ {
    unsafe { SelectObject(h_dc, h) }
}

/**
 * bit_blt函数执行与像素矩形相对应的颜色数据的位块传输，从指定的源设备上下文传输到目标设备上下文。
 * bit_blt仅在目标 DC 上执行剪裁。
 * 如果旋转或剪切转换在源设备上下文中生效， bit_blt 将返回错误。 如果源设备上下文中存在其他转换 (并且匹配转换在目标设备上下文) 中无效，则根据需要拉伸、压缩或旋转目标设备上下文中的矩形。
 * 如果源和目标设备上下文的颜色格式不匹配， bit_blt 函数会将源颜色格式转换为与目标格式匹配。
 * 记录增强型图元文件时，如果源设备上下文标识增强型图元文件设备上下文，则会发生错误。
 * 并非所有设备都支持bit_blt函数。有关详细信息，请参阅get_device_caps函数中的RC_BITBLT光栅功能条目以及以下函数：mask_blt、plg_blt和stretch_blt。
 * 如果源和目标设备上下文表示不同的设备，bit_blt将返回错误。若要在不同设备的DC之间传输数据，请通过调用get_di_bits将内存位图转换为DIB。若要向第二台设备显示DIB，请调用set_di_bits或stretch_di_bits。
 * Icm： 发生 blits 时，不执行颜色管理。
 * `h_dc` 目标设备上下文的句柄。
 * `x` 目标矩形左上角的 x 坐标（以逻辑单位为单位）。
 * `y` 目标矩形左上角的 y 坐标（以逻辑单位为单位）。
 * `cx` 源矩形和目标矩形的宽度（以逻辑单位为单位）。
 * `cy` 源矩形和目标矩形的高度（以逻辑单位为单位）。
 * `h_dc_src` 源设备上下文的句柄。
 * `x1` 源矩形左上角的 x 坐标（以逻辑单位为单位）。
 * `y1` 源矩形左上角的 y 坐标（以逻辑单位为单位）。
 * `rop` 以下列表显示了一些常见的光栅操作代码。
 * BLACKNESS | 使用与物理调色板中的索引 0 关联的颜色填充目标矩形。 （对于默认的物理调色板，该颜色为黑色。）
 * CAPTUREBLT | 包括生成图像中窗口顶部分层的任何窗口。 默认情况下，图像仅包含你的窗口。 请注意，这通常无法用于打印设备上下文。
 * DSTINVERT | 反转目标矩形。
 * MERGECOPY | 使用布尔 AND 运算符将源矩形的颜色与 hdc_dest 中当前选择的画笔合并。
 * MERGEPAINT | 使用布尔 OR 运算符将倒置源矩形的颜色与目标矩形的颜色合并。
 * NOMIRRORBITMAP | 防止位图镜像。
 * NOTSRCCOPY | 将反转源矩形复制到目标。
 * NOTSRCERASE | 使用布尔 OR 运算符组合源矩形和目标矩形的颜色，然后反转生成的颜色。
 * PATCOPY | 将 hdc_dest 中当前选择的画笔复制到目标位图中。
 * PATINVERT | 使用布尔 XOR 运算符将 hdc_dest 中当前选择的画笔的颜色与目标矩形的颜色组合在一起。
 * PATPAINT | 使用布尔 OR 运算符将 hdc_dest 中当前选择的画笔的颜色与反转源矩形的颜色相结合。 此操作的结果通过使用布尔 OR 运算符与目标矩形的颜色相结合。
 * SRCAND | 使用布尔 AND 运算符组合源矩形和目标矩形的颜色。
 * SRCCOPY | 将源矩形直接复制到目标矩形。
 * SRCERASE | 使用布尔 AND 运算符将目标矩形的反转颜色与源矩形的颜色组合在一起。
 * SRCINVERT | 使用布尔 XOR 运算符组合源矩形和目标矩形的颜色。
 * SRCPAINT | 使用布尔 OR 运算符组合源矩形和目标矩形的颜色。
 * WHITENESS | 使用与物理调色板中的索引 1 关联的颜色填充目标矩形。 （对于默认的物理调色板，该颜色为白色。）
 * */
pub fn bit_blt(
    h_dc: HDC,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    h_dc_src: HDC,
    x1: i32,
    y1: i32,
    rop: ROP_CODE,
) -> bool {
    unsafe { BitBlt(h_dc, x, y, cx, cy, h_dc_src, x1, y1, rop) }.is_ok()
}

/**
 * release_dc 函数 (DC) 释放设备上下文，释放它供其他应用程序使用。 release_dc 函数的效果取决于 DC 的类型。 它仅释放公用 DC 和窗口 DC。 它对类或专用 DC 没有影响。
 * 返回值指示是否释放了 DC。 如果释放 DC，则返回值为 1。如果未释放 DC，则返回值为零。
 * 对于每次调用 get_window_dc 函数和每次调用查询公用 DC 的 get_dc 函数，应用程序都必须调用 release_dc 函数。
 * 应用程序不能使用 release_dc 函数释放通过调用 create_dc 函数创建的 DC;相反，它必须使用 delete_dc 函数。 release_dc 必须从调用 get_dc 的同一线程调用。
 * `h_wnd` 要释放其 DC 的窗口的句柄。
 * `h_dc` 要释放的 DC 的句柄。
 * */
pub fn release_dc(h_wnd: HWND, h_dc: HDC) -> i32 {
    unsafe { ReleaseDC(h_wnd, h_dc) }
}

/**
 * delete_dc函数 (DC) 删除指定的设备上下文。
 * 应用程序不得删除通过调用 get_dc 函数获取其句柄的 DC。 相反，它必须调用 release_dc 函数来释放 DC。
 * `h_dc` 设备上下文的句柄。
 * */
pub fn delete_dc(h_dc: HDC) -> bool {
    unsafe { DeleteDC(h_dc) }.as_bool()
}

//noinspection StructuralWrap
/**
 * delete_object函数删除逻辑笔、画笔、字体、位图、区域或调色板，从而释放与该对象关联的所有系统资源。删除对象后，指定的句柄将不再有效。
 * 当绘图对象（钢笔或画笔）仍处于DC中时，请勿将其删除。
 * 删除图案画笔时，不会删除与该画笔关联的位图。必须单独删除位图。
 * `ho` 逻辑笔、画笔、字体、位图、区域或调色板的句柄。
 * */
pub fn delete_object(ho: HGDIOBJ) -> bool {
    unsafe { DeleteObject(ho) }.as_bool()
}

//noinspection StructuralWrap
/**
 * get_di_bits函数查询指定兼容位图的位，并使用指定格式将其作为DIB复制到缓冲区中。
 * 如果DIB请求的格式与其内部格式匹配，则会复制位图的RGB值。如果请求的格式与内部格式不匹配，则会合成颜色表。下表描述了针对每种格式合成的颜色表。
 * 1_BPP | 颜色表由黑色和白色条目组成。
 * 4_BPP | 颜色表由标准 VGA 调色板相同的颜色组合组成。
 * 8_BPP | 颜色表由 GDI 定义的 256 种颜色的常规混合组成。 (包含在这 256 种颜色中的是默认逻辑调色板中的 20 种颜色。)
 * 24_BPP | 不返回颜色表。
 * 自下而上 DIB 是通过将高度设置为正数来指定的，而自上而下 DIB 是通过将高度设置为负数来指定的。 位图颜色表将追加到 BITMAPINFO 结构中。
 * 应用程序调用此函数时，不得在设备上下文中选择 由 h_bmp 参数标识的位图。
 * 自下而上 DIB 的原点是位图的左下角;自上而下 DIB 的原点为左上角。
 * `h_dc` 设备上下文的句柄。
 * `h_bm` 位图的句柄。 这必须是兼容位图 (DDB) 。
 * `start` 要查询的第一个扫描行。
 * `c_lines` 要检索的扫描行数。
 * `usage` BITMAPINFO 结构的 bmiColors 成员的格式。 必须是以下值之一。
 * DIB_PAL_COLORS | 颜色表应包含当前逻辑调色板中的 16 位索引数组。
 * DIB_RGB_COLORS | 颜色表应包含文本红色、绿色、蓝色 (RGB) 值。
 * */
pub fn get_di_bits(
    h_dc: HDC,
    h_bm: HBITMAP,
    start: u32,
    c_lines: u32,
    usage: DIB_USAGE,
) -> (Vec<Vec<RGBQUAD>>, BITMAPINFOHEADER, Option<Vec<RGBQUAD>>) {
    unsafe {
        let mut bmi: BITMAPINFO = std::mem::zeroed();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        let (pixels, color_table) =
            if GetDIBits(h_dc, h_bm, start, c_lines, None, &mut bmi, usage) != 0 {
                let ptr = heap_alloc(bmi.bmiHeader.biSizeImage as usize).unwrap();
                ptr.write_bytes(0, bmi.bmiHeader.biSizeImage as usize);
                let bmi_ptr = heap_alloc((bmi.bmiHeader.biSize + bmi.bmiHeader.biClrUsed) as usize * std::mem::size_of::<RGBQUAD>()).unwrap();
                bmi_ptr.write_bytes(0, (bmi.bmiHeader.biSize + bmi.bmiHeader.biClrUsed) as usize * std::mem::size_of::<RGBQUAD>());
                let bmi_ptr2 = bmi_ptr as *mut BITMAPINFO;
                bmi_ptr2.write(bmi);
                let lines = GetDIBits(h_dc, h_bm, start, c_lines, Some(ptr), bmi_ptr2, usage);
                let bmi = bmi_ptr2.read();
                let color_table = if bmi.bmiHeader.biClrUsed > 0 {
                    let mut bmi_ptr2 = (bmi_ptr as *const u8).wrapping_add(bmi.bmiHeader.biSize as usize) as *const RGBQUAD;
                    let mut arr = Vec::with_capacity(bmi.bmiHeader.biClrUsed as usize);
                    for _ in 0..bmi.bmiHeader.biClrUsed {
                        arr.push(bmi_ptr2.read());
                        bmi_ptr2 = bmi_ptr2.wrapping_add(1);
                    }
                    Some(arr)
                } else {
                    None
                };
                heap_free(bmi_ptr);
                let mut ptr2 = ptr as *mut RGBQUAD;
                let mut data = Vec::with_capacity(lines as usize);
                for _ in 0..lines {
                    let mut line = Vec::with_capacity(bmi.bmiHeader.biWidth as usize);
                    for _ in 0..bmi.bmiHeader.biWidth as usize {
                        line.push(ptr2.read());
                        ptr2 = ptr2.wrapping_add(1);
                    }
                    data.push(line);
                }
                heap_free(ptr);
                (data, color_table)
            } else {
                (vec![], None)
            };
        (pixels, bmi.bmiHeader, color_table)
    }
}
