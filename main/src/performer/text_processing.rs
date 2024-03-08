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

pub(crate) fn transform_single_char(char: &char) -> String {
    if let Some(t) = get_data_map().get(char) {
        return t.to_string();
    }
    String::from(char.clone())
}

fn get_data_map() -> HashMap<char, &'static str> {
    OnceLock::new()
        .get_or_init(|| {
            let mut map = HashMap::new();
            for (k, v) in DATA {
                map.insert(*k, *v);
            }
            map
        })
        .to_owned()
}

const DATA: &[(char, &str)] = &[(' ', "空格"), ('\t', "Tab"), ('\n', "换行"), (',', "逗号")];
