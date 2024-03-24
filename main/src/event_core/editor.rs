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

use crate::performer::sound::SoundArgument::WithFreq;
use crate::{
    commander::keys::Keys, context::Context, ext::element::UiAutomationElementExt,
    performer::sound::SoundArgument::Single,
};
use a11y::ia2::{
    text::{
        AccessibleText,
        IA2TextBoundaryType::{IA2_TEXT_BOUNDARY_CHAR, IA2_TEXT_BOUNDARY_LINE},
    },
    WinEventSourceExt,
};
use log::error;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use win_wrap::control::edit::Edit;
use win_wrap::control::WindowControl;
use win_wrap::msaa::object::OBJID_CARET;
use win_wrap::uia::pattern::text::TextUnit;

//noinspection SpellCheckingInspection
/**
 * 订阅编辑器事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_editor_events(context: Arc<Context>) {
    subscribe_uia_events(context.clone()).await;
    subscribe_ia2_events(context.clone()).await;
    subscribe_msaa_events(context.clone()).await;
    subscribe_jab_events(context.clone()).await;
    subscribe_cusor_key_events(context.clone()).await;
}

#[allow(unused_variables)]
async fn subscribe_msaa_events(context: Arc<Context>) {
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();

    context
        .msaa
        .add_on_object_location_change_listener(move |src| {
            let performer = performer.clone();

            if OBJID_CARET.0 != src.id_object {
                return;
            }
            let Ok((obj, _)) = src.get_object() else {
                return;
            };
            let Some(obj) = obj.parent() else {
                return;
            };
            main_handler.spawn(async move {
                let control = WindowControl::from(obj.window());
                let (start, end) = control.get_sel();
                performer
                    .play_sound(WithFreq("progress.wav", (start * 10 + 400) as f32))
                    .await;
            });
        });
}

#[allow(unused_variables)]
async fn subscribe_jab_events(context: Arc<Context>) {
    let commander = context.commander.clone();
    let event_core = context.event_core.clone();
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();

    context
        .jab
        .add_on_property_caret_change_listener(move |src, old, new| {
            let Some((char, word, line)) = src.get_text_items(new) else {
                return;
            };

            let commander = commander.clone();
            let event_core = event_core.clone();
            let performer = performer.clone();

            main_handler.spawn(async move {
                if event_core
                    .should_ignore(char.to_string(), Duration::from_millis(50))
                    .await
                {
                    return;
                }
                match commander.get_last_pressed_key() {
                    Keys::VkUp | Keys::VkDown => performer.speak(&line).await,
                    _ => performer.speak(&char).await,
                };
            });
        });
}

#[allow(dead_code)]
async fn subscribe_uia_events(context: Arc<Context>) {
    let event_core = context.event_core.clone();
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();
    let commander = context.commander.clone();
    let ui_automation = context.ui_automation.clone();
    let root = ui_automation.get_root_element();

    let group = ui_automation.create_event_handler_group();
    // group.add_active_text_position_changed_listener(|element, range| {});
    // group.add_text_edit_text_changed_listener(|element| {});
    // group.add_changes_listener(|| {});

    group.add_text_selection_changed_listener(move |element| {
        {
            *editor_key_handle().lock().unwrap() = true;
        }

        let Some(caret) = element.get_caret() else {
            return;
        };
        match commander.get_last_pressed_key() {
            Keys::VkUp | Keys::VkDown => caret.expand_to_enclosing_unit(TextUnit::Line),
            _ => caret.expand_to_enclosing_unit(TextUnit::Character),
        }

        let event_core = event_core.clone();
        let performer = performer.clone();

        main_handler.spawn(async move {
            if event_core
                .should_ignore(caret.get_text(-1), Duration::from_millis(50))
                .await
            {
                return;
            }
            performer.speak(&caret).await;
        });
    });

    if ui_automation
        .add_event_handler_group(&root, &group)
        .is_err()
    {
        error!("Add the event handler group of the uia is failed.");
    }

    context
        .terminator
        .add_exiting_listener(move || ui_automation.remove_event_handler_group(&root, &group))
        .await;
}

async fn subscribe_ia2_events(context: Arc<Context>) {
    let commander = context.commander.clone();
    let event_core = context.event_core.clone();
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();

    context.ia2.add_on_text_caret_moved_listener(move |src| {
        let text = match src.get_text() {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };

        {
            *editor_key_handle().lock().unwrap() = true;
        }

        let caret = text.caret_offset().unwrap_or(0);
        let (_, _, text) = match commander.get_last_pressed_key() {
            Keys::VkUp | Keys::VkDown => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE),
            _ => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
        };

        let event_core = event_core.clone();
        let performer = performer.clone();

        main_handler.spawn(async move {
            if event_core
                .should_ignore(text.clone(), Duration::from_millis(50))
                .await
            {
                return;
            }
            performer.speak(&text).await;
        });
    })
}

pub(crate) fn editor_key_handle() -> &'static Mutex<bool> {
    static INSTANCE: OnceLock<Mutex<bool>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(false))
}

/// 处理编辑框的光标键播报
pub(crate) async fn subscribe_cusor_key_events(context: Arc<Context>) {
    let ctx = context.clone();
    let cb_uia = move |key: Keys, pressed| {
        let Ok(ctrl) = ctx.ui_automation.get_focused_element() else {
            return;
        };
        match pressed {
            true => {
                *editor_key_handle().lock().unwrap() = false;
            }
            false => {
                if !*editor_key_handle().lock().unwrap() {
                    let Some(caret) = ctrl.get_caret() else {
                        return;
                    };
                    match key {
                        Keys::VkLeft | Keys::VkRight => {
                            caret.expand_to_enclosing_unit(TextUnit::Character)
                        }
                        Keys::VkUp | Keys::VkDown => caret.expand_to_enclosing_unit(TextUnit::Line),
                        _ => {}
                    }

                    let pf = ctx.performer.clone();
                    ctx.main_handler.spawn(async move {
                        pf.play_sound(Single("edge.wav")).await;
                        pf.speak(&caret).await;
                    });
                }
            }
        }
    };

    let ctx = context.clone();
    let cb_ia2 = move |key: Keys, pressed| {
        if let Ok(acc_obj) = ctx.msaa.get_focus_object() {
            if let Ok(text) = AccessibleText::from_accessible_object(acc_obj) {
                match pressed {
                    true => {
                        *editor_key_handle().lock().unwrap() = false;
                    }
                    false => {
                        if !*editor_key_handle().lock().unwrap() {
                            let caret = text.caret_offset().unwrap_or(0);
                            let (_, _, text) = match key {
                                Keys::VkUp | Keys::VkDown => {
                                    text.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE)
                                }
                                _ => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
                            };

                            let pf = ctx.performer.clone();
                            ctx.main_handler.spawn(async move {
                                pf.play_sound(Single("edge.wav")).await;
                                pf.speak(&text).await;
                            });
                        }
                    }
                }
            }
        }
    };

    let keys = [Keys::VkUp, Keys::VkDown, Keys::VkLeft, Keys::VkRight];
    context.commander.add_key_event_listener(&keys, cb_uia);
    context.commander.add_key_event_listener(&keys, cb_ia2);
}
