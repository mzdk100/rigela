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

use crate::context::Context;
use std::sync::Arc;
use win_wrap::common::beep;

pub(crate) async fn subscribe_events(context: Arc<Context>) {
    let group = context.ui_automation.create_event_handler_group();
    group.add_active_text_position_changed_listener(|element, range| {
        beep(400, 40);
    });
    group.add_text_edit_text_changed_listener(|element| {});
}
