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

use std::collections::HashMap;
use std::sync::OnceLock;

/// 单个字符的预处理
pub(crate) fn transform_single_char(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars();
    match get_single_char_transform_map().get(&chars.next().unwrap()) {
        Some(v) => {
            result.push_str(v);
            result.push_str(chars.as_str());
        }
        None => result.push_str(text),
    }

    result
}

fn get_single_char_transform_map() -> HashMap<char, &'static str> {
    OnceLock::new()
        .get_or_init(|| {
            let mut map = HashMap::new();
            for (k, v) in SINGLE_CHAR_TRANSFORM_DATA {
                map.insert(*k, *v);
            }
            map
        })
        .to_owned()
}

const SINGLE_CHAR_TRANSFORM_DATA: &[(char, &str)] = &[
    ('!', "叹号"),
    ('！', "叹号"),
    ('"', "双引号"),
    ('“', "左双引号"),
    ('”', "右双引号"),
    ('#', "井号"),
    ('$', "美元"),
    ('￥', "人民币"),
    ('%', "百分号"),
    ('&', "和"),
    ('\'', "单引号"),
    ('‘', "左单引号"),
    ('’', "右单引号"),
    ('(', "左括号"),
    ('（', "左括号"),
    (')', "右括号"),
    ('）', "右括号"),
    ('*', "星"),
    ('+', "加"),
    (',', "逗号"),
    ('，', "逗号"),
    ('-', "减"),
    ('.', "点"),
    ('。', "句号"),
    ('/', "斜杠"),
    (':', "冒号"),
    ('：', "冒号"),
    (';', "分号"),
    ('；', "分号"),
    ('<', "小于"),
    ('《', "左书名号"),
    ('=', "等于"),
    ('>', "大于"),
    ('》', "右书名号"),
    ('?', "问号"),
    ('？', "问号"),
    ('@', "艾特"),
    ('[', "左方括号"),
    ('【', "左方括号"),
    ('\\', "反斜杠"),
    ('、', "顿号"),
    (']', "右方括号"),
    ('】', "右方括号"),
    ('^', "上尖号"),
    ('…', "省略号"),
    ('_', "下划线"),
    ('—', "破折号"),
    ('`', "反撇号"),
    ('·', "圆点"),
    ('{', "左花括号"),
    ('|', "竖杠"),
    ('}', "右花括号"),
    ('~', "波浪号"),
];
