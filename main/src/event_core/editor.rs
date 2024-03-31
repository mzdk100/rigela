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

use crate::{
    commander::keyboard::keys::Keys,
    context::{Context, ContextAccessor},
    ext::element::UiAutomationElementExt,
    performer::sound::SoundArgument::{Single, WithFreq},
};
use a11y::ia2::{
    text::{
        AccessibleText,
        IA2TextBoundaryType::{IA2_TEXT_BOUNDARY_CHAR, IA2_TEXT_BOUNDARY_LINE},
    },
    WinEventSourceExt,
};
use log::error;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Weak,
    },
    time::Duration,
};
use win_wrap::{
    control::{edit::Edit, WindowControl},
    msaa::object::OBJID_CARET,
    uia::pattern::text::TextUnit,
};

//noinspection SpellCheckingInspection
/**
 * 订阅编辑器事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_editor_events(context: Weak<Context>) {
    subscribe_uia_events(context.clone()).await;
    subscribe_ia2_events(context.clone()).await;
    subscribe_msaa_events(context.clone()).await;
    subscribe_jab_events(context.clone()).await;
    subscribe_cusor_key_events(context.clone()).await;
}

#[allow(unused_variables)]
async fn subscribe_msaa_events(context: Weak<Context>) {
    let ctx = context.clone();
    context
        .get_msaa()
        .add_on_object_location_change_listener(move |src| {
            if OBJID_CARET.0 != src.id_object {
                return;
            }
            let Ok((obj, _)) = src.get_object() else {
                return;
            };
            let Some(obj) = obj.parent() else {
                return;
            };

            let ctx2 = ctx.clone();
            ctx.get_work_runtime().spawn(async move {
                let control = WindowControl::from(obj.window());
                let (start, end) = control.get_sel();
                ctx2.get_performer()
                    .play_sound(WithFreq("progress.wav", (start * 10 + 400) as f32))
                    .await;
            });
        });
}

#[allow(unused_variables)]
async fn subscribe_jab_events(context: Weak<Context>) {
    let ctx = context.clone();
    let mng = context.get_commander().get_keyboard_manager().clone();

    context
        .get_jab()
        .add_on_property_caret_change_listener(move |src, old, new| {
            let Some((char, word, line)) = src.get_text_items(new) else {
                return;
            };

            let ctx2 = ctx.clone();
            let commander = mng.clone();

            ctx.get_work_runtime().spawn(async move {
                if ctx2
                    .get_event_core()
                    .should_ignore(char.to_string(), Duration::from_millis(50))
                    .await
                {
                    return;
                }
                match commander.get_last_pressed_key() {
                    Keys::VkUp | Keys::VkDown => ctx2.get_performer().speak(&line).await,
                    _ => ctx2.get_performer().speak(&char).await,
                };
            });
        });
}

#[allow(dead_code)]
async fn subscribe_uia_events(context: Weak<Context>) {
    let ctx = context.clone();
    let mng = context.get_commander().get_keyboard_manager().clone();
    let root = context.get_ui_automation().get_root_element();

    let group = context.get_ui_automation().create_event_handler_group();
    // group.add_active_text_position_changed_listener(|element, range| {});
    // group.add_text_edit_text_changed_listener(|element| {});
    // group.add_changes_listener(|| {});

    group.add_text_selection_changed_listener(move |element| {
        EDITOR_EDGE_KEY_HANDLE.store(true, Ordering::SeqCst);

        let Some(caret) = element.get_caret() else {
            return;
        };

        match mng.get_last_pressed_key() {
            Keys::VkUp | Keys::VkDown => caret.expand_to_enclosing_unit(TextUnit::Line),
            _ => caret.expand_to_enclosing_unit(TextUnit::Character),
        }

        let ctx2 = ctx.clone();
        ctx.get_work_runtime().spawn(async move {
            if ctx2
                .get_event_core()
                .should_ignore(caret.get_text(-1), Duration::from_millis(50))
                .await
            {
                return;
            }
            ctx2.get_performer().speak(&caret).await;
        });
    });

    if context
        .get_ui_automation()
        .add_event_handler_group(&root, &group)
        .is_err()
    {
        error!("Add the event handler group of the uia is failed.");
    }

    let ctx = context.clone();
    context.get_terminator().add_exiting_listener(move || {
        ctx.get_ui_automation()
            .remove_event_handler_group(&root, &group)
    });
}

async fn subscribe_ia2_events(context: Weak<Context>) {
    let ctx = context.clone();
    let mng = context.get_commander().get_keyboard_manager().clone();

    context
        .get_ia2()
        .add_on_text_caret_moved_listener(move |src| {
            let text = match src.get_text() {
                Ok(t) => t,
                Err(e) => {
                    error!("Can't get the text. ({})", e);
                    return;
                }
            };

            EDITOR_EDGE_KEY_HANDLE.store(true, Ordering::SeqCst);

            let caret = text.caret_offset().unwrap_or(0);
            let (_, _, text) = match mng.get_last_pressed_key() {
                Keys::VkUp | Keys::VkDown => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE),
                _ => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
            };

            let ctx2 = ctx.clone();
            ctx.get_work_runtime().spawn(async move {
                if ctx2
                    .get_event_core()
                    .should_ignore(text.clone(), Duration::from_millis(50))
                    .await
                {
                    return;
                }
                ctx2.get_performer().speak(&text).await;
            });
        })
}

/// 编辑框边缘光标按键是否处理
pub(crate) static EDITOR_EDGE_KEY_HANDLE: AtomicBool = AtomicBool::new(false);

/// 处理编辑框的光标键播报
pub(crate) async fn subscribe_cusor_key_events(context: Weak<Context>) {
    let ctx = context.clone();
    let cb_uia = move |key: Keys, pressed| {
        let Ok(ctrl) = ctx.get_ui_automation().get_focused_element() else {
            return;
        };
        match pressed {
            true => EDITOR_EDGE_KEY_HANDLE.store(false, Ordering::SeqCst),
            false => {
                if !EDITOR_EDGE_KEY_HANDLE.load(Ordering::Relaxed) {
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

                    let ctx2 = ctx.clone();
                    ctx.get_work_runtime().spawn(async move {
                        ctx2.get_performer().play_sound(Single("edge.wav")).await;
                        ctx2.get_performer().speak(&caret).await;
                    });
                }
            }
        }
    };

    let ctx = context.clone();
    let cb_ia2 = move |key: Keys, pressed| {
        if let Ok(acc_obj) = ctx.get_msaa().get_focus_object() {
            if let Ok(text) = AccessibleText::from_accessible_object(acc_obj) {
                match pressed {
                    true => EDITOR_EDGE_KEY_HANDLE.store(false, Ordering::SeqCst),
                    false => {
                        if !EDITOR_EDGE_KEY_HANDLE.load(Ordering::Relaxed) {
                            let caret = text.caret_offset().unwrap_or(0);
                            let (_, _, text) = match key {
                                Keys::VkUp | Keys::VkDown => {
                                    text.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE)
                                }
                                _ => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
                            };

                            let ctx2 = ctx.clone();
                            ctx.get_work_runtime().spawn(async move {
                                ctx2.get_performer().play_sound(Single("edge.wav")).await;
                                ctx2.get_performer().speak(&text).await;
                            });
                        }
                    }
                }
            }
        }
    };

    let keys = [Keys::VkUp, Keys::VkDown, Keys::VkLeft, Keys::VkRight];
    let mng = context.get_commander().get_keyboard_manager().clone();

    mng.add_key_event_listener(&keys, cb_uia);
    mng.add_key_event_listener(&keys, cb_ia2);
}
