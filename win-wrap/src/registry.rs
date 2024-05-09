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

use crate::common::Result;
pub use windows::Win32::System::Registry::{
    HKEY, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_CURRENT_USER_LOCAL_SETTINGS, HKEY_DYN_DATA, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA,
    HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_ALL_ACCESS, KEY_CREATE_LINK,
    KEY_CREATE_SUB_KEY, KEY_ENUMERATE_SUB_KEYS, KEY_EXECUTE, KEY_NOTIFY, KEY_QUERY_VALUE, KEY_READ,
    KEY_SET_VALUE, KEY_WOW64_32KEY, KEY_WOW64_64KEY, KEY_WOW64_RES, KEY_WRITE, REG_EXPAND_SZ,
    REG_MULTI_SZ, REG_NONE, REG_SAM_FLAGS, REG_SZ, REG_VALUE_TYPE,
};
use windows::{
    core::HSTRING,
    Win32::System::Registry::{RegCloseKey, RegDeleteValueW, RegOpenKeyExW, RegSetValueExW},
};

/**
 * 打开指定的注册表项。 请注意，键名称不区分大小写。
 * 若要对密钥执行事务注册表操作，请调用 reg_open_key_transacted 函数。
 * 与 reg_create_key_ex 函数不同，如果注册表中不存在指定键， reg_open_key_ex 函数不会创建指定键。
 * 某些注册表操作对密钥的安全描述符（而不是获取密钥句柄时指定的访问掩码）执行访问检查。 例如，即使某个项以KEY_READ的 sam_desir 打开，也可用于创建注册表项（如果密钥的安全描述符允许）。 相比之下， reg_set_value_ex 函数专门要求使用KEY_SET_VALUE访问权限打开密钥。
 * 如果服务或应用程序模拟不同的用户，请不要将此函数与 HKEY_CURRENT_USER一起使用。 请改为调用 reg_open_current_user 函数。
 * 请注意，将重定向访问某些注册表项的操作。 有关详细信息，请参阅 注册表虚拟化 和 注册表中的 32 位和 64 位应用程序数据。
 * `h_key` 打开的注册表项的句柄。 此句柄由 reg_create_key_ex 或 reg_open_key_ex 函数返回，也可以是以下 预定义键之一：
 * - HKEY_CLASSES_ROOT
 * - HKEY_CURRENT_CONFIG
 * - HKEY_CURRENT_USER
 * - HKEY_LOCAL_MACHINE
 * - HKEY_USERS
 * `sub_key` 要打开的注册表子项的名称。键名称不区分大小写。如果 sub_key 参数为 NULL 或指向空字符串的指针，并且 h_key 是预定义键，则系统会刷新预定义的键，并且 result 接收传递到函数中的同一 h_key 句柄。 否则， result 将接收打开的密钥的新句柄。有关详细信息，请参阅 注册表元素大小限制。
 * `options` 指定要在打开密钥时应用的选项。 将此参数设置为零或以下参数：
 * - REG_OPTION_OPEN_LINK 键是符号链接。 仅当绝对必要时才应使用注册表符号链接。
 * `sam_desired` 一个掩码，指定要打开的密钥的所需访问权限。 如果密钥的安全描述符不允许对调用进程进行请求的访问，则函数将失败。 有关详细信息，请参阅 注册表项安全和访问权限。
 * */
pub fn reg_open_key_ex(
    h_key: HKEY,
    sub_key: Option<&str>,
    options: u32,
    sam_desired: REG_SAM_FLAGS,
) -> HKEY {
    unsafe {
        let mut res = std::mem::zeroed();
        if match sub_key {
            None => RegOpenKeyExW(h_key, None, options, sam_desired, &mut res),
            Some(key) => RegOpenKeyExW(h_key, &HSTRING::from(key), options, sam_desired, &mut res),
        }
        .is_ok()
        {
            return res;
        }
        HKEY::default()
    }
}

/**
 * 设置注册表项下指定值的数据和类型。
 * 值大小受可用内存的限制。 但是，在注册表中存储较大的值可能会影响其性能。 (超过 2，048 字节的长值) 应存储为文件，文件的位置存储在注册表中。
 * 应用程序元素（如图标、位图和可执行文件）应存储为文件，而不是放置在注册表中。
 * 如果 type 是REG_SZ、REG_MULTI_SZ或REG_EXPAND_SZ类型，并且此函数的 ANSI 版本通过显式调用 reg_set_value_ex 或未在将 Windows.h 文件) 之前未定义 UNICODE 来 (使用，则 data 参数指向的数据必须是 ANSI 字符串。 字符串在存储到注册表中之前会转换为 Unicode。
 * 请注意，将重定向访问某些注册表项的操作。 有关详细信息，请参阅 注册表虚拟化 和 注册表中的 32 位和 64 位应用程序数据。
 * 请考虑使用 reg_set_key_value 函数，该函数提供了一种更方便的方式来设置注册表项的值。
 * `h_key` 打开的注册表项的句柄。 Key 必须已使用KEY_SET_VALUE访问权限打开。 有关详细信息，请参阅 注册表项安全和访问权限。此句柄由 reg_create_key_ex、 reg_create_key_transacted、 reg_open_key_ex 或 reg_open_key_transacted 函数返回。 也可以是以下 预定义键之一：
 * - HKEY_CLASSES_ROOT
 * - HKEY_CURRENT_CONFIG
 * - HKEY_CURRENT_USER
 * - HKEY_LOCAL_MACHINE
 * - HKEY_USERS
 * - HKEY_PERFORMANCE_TEXT
 * - HKEY_PERFORMANCE_NLSTEXT
 * `value_name` 要设置的值的名称。 如果键中尚不存在具有此名称的值，则函数会将其添加到键中。如果 value_name 为 NULL 或空字符串“”，则该函数将设置键的未命名值或默认值的类型和数据。有关详细信息，请参阅 注册表元素大小限制。注册表项没有默认值，但它们可以有一个未命名的值，该值可以是任何类型的。
 * `reserved` 此参数是保留的，必须为零。
 * `type` data 参数指向的数据类型。
 * `data` 要存储的数据。对于基于字符串的类型（如 REG_SZ），字符串必须以 null 结尾。 对于 REG_MULTI_SZ 数据类型，字符串必须以两个 null 字符结尾。
 * */
pub fn reg_set_value_ex(
    h_key: HKEY,
    value_name: Option<&str>,
    reserved: u32,
    r#type: REG_VALUE_TYPE,
    data: Option<&[u8]>,
) -> Result<()> {
    unsafe {
        match value_name {
            None => RegSetValueExW(h_key, None, reserved, r#type, data),
            Some(name) => RegSetValueExW(h_key, &HSTRING::from(name), reserved, r#type, data),
        }
    }
    .ok()
}

/**
 * 从指定的注册表项中删除命名值。请注意，值名称不区分大小写。
 * `h_key` 打开的注册表项的句柄。Key 必须已使用KEY_SET_VALUE访问权限打开。有关详细信息，请参阅注册表项安全性和访问权限。此句柄由 reg_create_key_ex、reg_create_key_transacted、reg_open_key_ex 或 reg_open_key_transacted 函数返回。它也可以是以下预定义键之一：
 * - HKEY_CLASSES_ROOT
 * - HKEY_CURRENT_CONFIG
 * - HKEY_CURRENT_USER
 * - HKEY_LOCAL_MACHINE
 * - HKEY_USERS
 * `value_name` 要删除的注册表值。如果此参数为 NULL 或空字符串，则删除 RegSetValue 函数设置的值。有关更多信息，请参见注册表元素大小限制。
 * */
pub fn reg_delete_value(h_key: HKEY, value_name: Option<&str>) -> Result<()> {
    unsafe {
        match value_name {
            None => RegDeleteValueW(h_key, None),
            Some(name) => RegDeleteValueW(h_key, &HSTRING::from(name)),
        }
    }
    .ok()
}

/**
 * 关闭指定注册表项的句柄。
 * 指定键的句柄在关闭后不应使用，因为它将不再有效。 键句柄的打开时间不应超过必要时间。
 * reg_close_key 函数在返回之前不一定将信息写入注册表;缓存刷新到硬盘可能需要几秒钟的时间。 如果应用程序必须将注册表信息显式写入硬盘，则可以使用 reg_flush_key 函数。 但是，reg_flush_key 使用许多系统资源，仅在必要时才应调用。
 * `h_key` 要关闭的打开键的句柄。 该句柄必须已由 reg_create_key_ex、reg_create_key_transacted、reg_open_key_ex、reg_open_key_transacted 或 reg_connect_registry 函数打开。
 * */
pub fn reg_close_key(h_key: HKEY) -> bool {
    unsafe { RegCloseKey(h_key) }.is_ok()
}
