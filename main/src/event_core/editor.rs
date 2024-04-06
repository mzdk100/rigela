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
    commander::keyboard::keys::Keys::{self, VkDown, VkLeft, VkRight, VkUp},
    context::{Context, ContextAccessor},
    ext::element::UiAutomationElementExt,
    performer::sound::SoundArgument::{self, Single},
};
use a11y::{
    ia2::{
        text::{
            AccessibleText,
            IA2TextBoundaryType::{IA2_TEXT_BOUNDARY_CHAR, IA2_TEXT_BOUNDARY_LINE},
        },
        WinEventSourceExt,
    },
    jab::callback::AccessibleContextType,
};
use arc_swap::ArcSwap;
use log::error;
use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, OnceLock, Weak,
    },
    time::Duration,
};
use win_wrap::{
    control::{edit::Edit, WindowControl},
    msaa::{event::WinEventSource, object::OBJID_CARET},
    uia::{element::UiAutomationElement, pattern::text::TextUnit},
};

const DURATION: Duration = Duration::from_millis(50);

/// 编辑器控件的缓冲
#[derive(Debug, Clone)]
enum Control {
    None,
    Uia(UiAutomationElement),
    Ia2(AccessibleText),
    Jab(AccessibleContextType, i32),
}

unsafe impl Send for Control {}

unsafe impl Sync for Control {}

/// 编辑器
#[derive(Debug, Clone)]
pub(crate) struct Editor {
    control: Arc<ArcSwap<Control>>,
    edge_handled: Arc<AtomicBool>,
    context: OnceLock<Weak<Context>>,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            control: ArcSwap::new(Control::None.into()).into(),
            edge_handled: AtomicBool::new(false).into(),
            context: OnceLock::new(),
        }
    }

    /// 订阅编辑器事件。
    /// `context` 读屏框架的上下文环境。
    pub(crate) async fn subscribe_events(&self, context: Weak<Context>) {
        self.context.set(context).unwrap_or(());

        self.subscribe_uia_events().await;
        self.subscribe_ia2_events().await;
        self.subscribe_msaa_events().await;
        self.subscribe_jab_events().await;

        self.subscribe_edge_cursor_events().await;
    }

    /// 取消编辑器边缘按键处理
    pub(crate) fn cancel_edge_handle(&self) {
        self.edge_handled.store(true, Ordering::SeqCst);
    }

    /**
     * 清除编辑框的焦点。
     * */
    pub(crate) fn clear_focus_control(&self) {
        let control = self.control.clone();
        control.store(Control::None.into());
    }

    // 订阅 msaa 编辑器事件
    async fn subscribe_msaa_events(&self) {
        let context = self.context.get().unwrap();

        let ctx = context.clone();
        let cb = move |src: WinEventSource| {
            if OBJID_CARET.0 != src.id_object {
                return;
            }
            let Ok((obj, _)) = src.get_object() else {
                return;
            };
            let Some(obj) = obj.parent() else {
                return;
            };
            let control = WindowControl::from(obj.window());
            let (start, _end) = control.get_sel();
            let sound = SoundArgument::WithFreq("progress.wav", (start * 10 + 400) as f32);

            let ctx2 = ctx.clone();
            ctx.get_work_runtime().spawn(async move {
                ctx2.get_performer().play_sound(sound).await;
            });
        };

        let msaa = context.get_msaa();
        msaa.add_on_object_location_change_listener(cb);
    }

    // 订阅 jab 编辑器事件
    async fn subscribe_jab_events(&self) {
        let context = self.context.get().unwrap();

        let ctx = context.clone();
        let mng = context.get_commander().get_keyboard_manager().clone();
        let control = self.control.clone();
        let edge_handled = self.edge_handled.clone();
        let cb = move |src: AccessibleContextType, _, new| {
            control.store(Control::Jab(src.clone(), new).into());

            let Some((char, _word, line)) = src.get_text_items(new) else {
                return;
            };
            edge_handled.store(true, Ordering::SeqCst);

            let ctx2 = ctx.clone();
            let mng = mng.clone();
            ctx.get_work_runtime().spawn(async move {
                let ec = ctx2.get_event_core();
                if ec.should_ignore(char.to_string(), DURATION).await {
                    return;
                }

                match mng.get_last_pressed_key() {
                    VkUp | VkDown => ctx2.get_performer().speak(&line).await,
                    _ => ctx2.get_performer().speak(&char).await,
                };
            });
        };

        context.get_jab().add_on_property_caret_change_listener(cb);
    }

    // 订阅 uia 编辑器事件
    async fn subscribe_uia_events(&self) {
        let context = self.context.get().unwrap();
        let ctx = context.clone();
        let edge_handled = self.edge_handled.clone();
        let control = self.control.clone();
        let mng = context.get_commander().get_keyboard_manager().clone();
        let root = context.get_ui_automation().get_root_element();

        let group = context.get_ui_automation().create_event_handler_group();
        // group.add_active_text_position_changed_listener(|element, range| {});
        // group.add_text_edit_text_changed_listener(|element| {});
        // group.add_changes_listener(|| {});

        group.add_text_selection_changed_listener(move |element| {
            control.store(Arc::new(Control::Uia(element.clone())));
            edge_handled.store(true, Ordering::SeqCst);

            let Some(caret) = element.get_caret() else {
                return;
            };

            let mut is_blank_line = false;

            match mng.get_last_pressed_key() {
                VkUp | VkDown => {
                    caret.expand_to_enclosing_unit(TextUnit::Line);
                    let text = caret.get_text(-1);
                    if ["\r", "\n", "\r\n"].contains(&text.as_str()) || text.is_empty() {
                        is_blank_line = true;
                    }
                }
                _ => caret.expand_to_enclosing_unit(TextUnit::Character),
            }

            let ctx2 = ctx.clone();
            ctx.get_work_runtime().spawn(async move {
                let ec = ctx2.get_event_core();
                if ec.should_ignore(caret.get_text(-1), DURATION).await {
                    return;
                }

                match is_blank_line {
                    true => {
                        ctx2.get_performer().speak(&("空航".to_string())).await;
                    }
                    false => {
                        ctx2.get_performer().speak(&caret).await;
                    }
                }
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

    // 订阅 ia2 编辑器事件
    async fn subscribe_ia2_events(&self) {
        let context = self.context.get().unwrap();

        let ctx = context.clone();
        let control = self.control.clone();
        let edge_handled = self.edge_handled.clone();
        let mng = context.get_commander().get_keyboard_manager().clone();
        let cb = move |src: WinEventSource| {
            let text = match src.get_text() {
                Ok(t) => t,
                Err(_) => return,
            };

            control.store(Arc::new(Control::Ia2(text.clone())));
            edge_handled.store(true, Ordering::SeqCst);

            let caret = text.caret_offset().unwrap_or(0);
            let (_, _, text) = match mng.get_last_pressed_key() {
                VkUp | VkDown => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE),
                _ => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
            };

            let ctx2 = ctx.clone();
            ctx.get_work_runtime().spawn(async move {
                let ec = ctx2.get_event_core();
                if ec.should_ignore(text.clone(), DURATION).await {
                    return;
                }
                ctx2.get_performer().speak(&text).await;
            });
        };

        context.get_ia2().add_on_text_caret_moved_listener(cb);
    }

    /// 处理编辑框的光标键播报
    pub(crate) async fn subscribe_edge_cursor_events(&self) {
        let context = self.context.get().unwrap();

        let ctx = context.clone();
        let control = self.control.clone();
        let edge_handled = self.edge_handled.clone();
        let cb_uia = move |key: Keys, pressed| {
            let control: Control = control.load().deref().deref().clone();
            let Control::Uia(ctrl) = control else {
                return;
            };

            match pressed {
                true => edge_handled.store(false, Ordering::SeqCst),

                false if !edge_handled.load(Ordering::Relaxed) => {
                    let Some(caret) = ctrl.get_caret() else {
                        return;
                    };
                    let mut is_blank_line = false;
                    match key {
                        VkUp | VkDown => {
                            caret.expand_to_enclosing_unit(TextUnit::Line);
                            let text = caret.get_text(-1);
                            if ["\r", "\n", "\r\n"].contains(&text.as_str()) || text.is_empty() {
                                is_blank_line = true;
                            }
                        }
                        _ => caret.expand_to_enclosing_unit(TextUnit::Character),
                    }

                    let ctx2 = ctx.clone();
                    ctx.get_work_runtime().spawn(async move {
                        ctx2.get_performer().play_sound(Single("edge.wav")).await;
                        match is_blank_line {
                            true => {
                                ctx2.get_performer().speak(&"空航".to_string()).await;
                            }
                            false => {
                                ctx2.get_performer().speak(&caret).await;
                            }
                        }
                    });
                }

                _ => {}
            }
        };

        let ctx = context.clone();
        let edge_handled = self.edge_handled.clone();
        let control = self.control.clone();
        let cb_ia2 = move |key: Keys, pressed| {
            let control = control.load().deref().deref().clone();
            let Control::Ia2(ctrl) = control else {
                return;
            };

            match pressed {
                true => edge_handled.store(false, Ordering::SeqCst),

                false if !edge_handled.load(Ordering::Relaxed) => {
                    let caret = ctrl.caret_offset().unwrap_or(0);
                    let (_, _, text) = match key {
                        VkUp | VkDown => ctrl.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE),
                        _ => ctrl.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
                    };

                    let ctx2 = ctx.clone();
                    ctx.get_work_runtime().spawn(async move {
                        let ec = ctx2.get_event_core();
                        if ec.should_ignore(text.clone(), DURATION).await {
                            return;
                        }
                        ctx2.get_performer().speak(&text).await;
                    });
                }

                _ => {}
            }
        };

        let ctx = context.clone();
        let edge_handled = self.edge_handled.clone();
        let control = self.control.clone();
        let cb_jab = move |key: Keys, pressed| {
            let control = control.load().deref().deref().clone();
            let Control::Jab(src, pos) = control else {
                return;
            };

            match pressed {
                true => edge_handled.store(false, Ordering::SeqCst),

                false if !edge_handled.load(Ordering::Relaxed) => {
                    let Some((char, _word, line)) = src.get_text_items(pos) else {
                        return;
                    };

                    let ctx2 = ctx.clone();
                    ctx.get_work_runtime().spawn(async move {
                        let ec = ctx2.get_event_core();
                        if ec.should_ignore(char.to_string(), DURATION).await {
                            return;
                        }
                        match key {
                            VkUp | VkDown => ctx2.get_performer().speak(&line).await,
                            _ => ctx2.get_performer().speak(&char).await,
                        };
                    });
                }

                _ => {}
            }
        };

        let keys = [VkUp, VkDown, VkLeft, VkRight];
        let mng = context.get_commander().get_keyboard_manager().clone();

        mng.add_key_event_listener(&keys, cb_uia);
        mng.add_key_event_listener(&keys, cb_ia2);
        mng.add_key_event_listener(&keys, cb_jab);
    }
}

/// 取消编辑框边缘光标键播报
#[macro_export]
macro_rules! cancel_edge_handle {
    ($context: expr) => {
        let control = $context.get_event_core().editor.clone();
        control.cancel_edge_handle();
    };
}
