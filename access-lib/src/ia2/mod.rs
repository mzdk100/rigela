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

pub mod object;
pub mod relation;
pub mod text;

use crate::{
    ia2::{object::Accessible2Object, text::AccessibleText},
    IAccessible2Lib::AccessibleEventID::IA2EventID::{
        IA2_EVENT_ACTION_CHANGED, IA2_EVENT_ACTIVE_DESCENDANT_CHANGED,
        IA2_EVENT_DOCUMENT_ATTRIBUTE_CHANGED, IA2_EVENT_DOCUMENT_CONTENT_CHANGED,
        IA2_EVENT_DOCUMENT_LOAD_COMPLETE, IA2_EVENT_DOCUMENT_LOAD_STOPPED,
        IA2_EVENT_DOCUMENT_RELOAD, IA2_EVENT_HYPERLINK_END_INDEX_CHANGED,
        IA2_EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED, IA2_EVENT_HYPERLINK_SELECTED_LINK_CHANGED,
        IA2_EVENT_HYPERLINK_START_INDEX_CHANGED, IA2_EVENT_HYPERTEXT_CHANGED,
        IA2_EVENT_HYPERTEXT_LINK_ACTIVATED, IA2_EVENT_HYPERTEXT_LINK_SELECTED,
        IA2_EVENT_HYPERTEXT_NLINKS_CHANGED, IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED,
        IA2_EVENT_PAGE_CHANGED, IA2_EVENT_ROLE_CHANGED, IA2_EVENT_SECTION_CHANGED,
        IA2_EVENT_TABLE_CAPTION_CHANGED, IA2_EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED,
        IA2_EVENT_TABLE_COLUMN_HEADER_CHANGED, IA2_EVENT_TABLE_MODEL_CHANGED,
        IA2_EVENT_TABLE_ROW_DESCRIPTION_CHANGED, IA2_EVENT_TABLE_ROW_HEADER_CHANGED,
        IA2_EVENT_TABLE_SUMMARY_CHANGED, IA2_EVENT_TEXT_ATTRIBUTE_CHANGED,
        IA2_EVENT_TEXT_CARET_MOVED, IA2_EVENT_TEXT_COLUMN_CHANGED, IA2_EVENT_TEXT_INSERTED,
        IA2_EVENT_TEXT_REMOVED, IA2_EVENT_TEXT_SELECTION_CHANGED, IA2_EVENT_TEXT_UPDATED,
        IA2_EVENT_VISIBLE_DATA_CHANGED,
    },
};
use std::sync::RwLock;
use win_wrap::{
    common::Result,
    msaa::event::{WinEventHook, WinEventSource},
};

pub trait WinEventSourceExt {
    fn get_object2(&self) -> Result<Accessible2Object>;
    fn get_text(&self) -> Result<AccessibleText>;
}

impl WinEventSourceExt for WinEventSource {
    fn get_object2(&self) -> Result<Accessible2Object> {
        let obj = match self.get_object() {
            Ok((obj, _)) => obj,
            Err(e) => return Err(e),
        };
        Accessible2Object::from_accessible_object(obj)
    }

    fn get_text(&self) -> Result<AccessibleText> {
        let obj = match self.get_object() {
            Ok((obj, _)) => obj,
            Err(e) => return Err(e),
        };
        AccessibleText::from_accessible_object(obj)
    }
}

#[derive(Debug)]
pub struct Ia2 {
    events: RwLock<Vec<WinEventHook>>,
}

impl Ia2 {
    /**
     * 创建一个IA2实例。
     * */
    pub fn new() -> Self {
        Self {
            events: vec![].into(),
        }
    }

    /**
     * 可访问对象的操作数量或属性的更改由此类型的事件发出信号。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_action_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_ACTION_CHANGED as u32, func));
    }

    /**
     * 组件的活动后代已更改。活动后代用于具有瞬态子项的对象。
     * 注意： 由于 MSAA 的 WinEvents 不允许在IA2_EVENT_ACTIVE_DESCENDANT_CHANGED事件上传递活动的子索引，因此无法使用管理后代方案。相反，活动子对象必须触发 MSAA 的EVENT_OBJECT_FOCUS。在未来的版本中，可能会添加新的事件机制，以提供与事件一起传递的事件特定数据。那时，IA2_EVENT_ACTIVE_DESCENDANT_CHANGED事件和IA2_STATE_MANAGES_DESCENDANTS状态将很有用。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_active_descendant_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_ACTIVE_DESCENDANT_CHANGED as u32,
            func,
        ));
    }

    /**
     * 文档对象的文档范围属性已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_document_attribute_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_DOCUMENT_ATTRIBUTE_CHANGED as u32,
            func,
        ));
    }

    /**
     * 文档的内容已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_document_content_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_DOCUMENT_CONTENT_CHANGED as u32,
            func,
        ));
    }

    /**
     * 文档的加载已完成。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_document_load_complete_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_DOCUMENT_LOAD_COMPLETE as u32,
            func,
        ));
    }

    /**
     * 文档的加载中断。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_document_load_stopped_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_DOCUMENT_LOAD_STOPPED as u32,
            func,
        ));
    }

    /**
     * 正在重新加载文档内容。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_document_reload_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_DOCUMENT_RELOAD as u32, func));
    }

    /**
     * 包含字符串中此链接的结束索引已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hyperlink_end_index_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERLINK_END_INDEX_CHANGED as u32,
            func,
        ));
    }

    /**
     * 与此超链接对象关联的定位点数已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hyperlink_number_of_anchors_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERLINK_NUMBER_OF_ANCHORS_CHANGED as u32,
            func,
        ));
    }

    /**
     * 超链接选定状态从选定变为未选定或从未选定更改为选定。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hyperlink_selected_link_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERLINK_SELECTED_LINK_CHANGED as u32,
            func,
        ));
    }

    /**
     * 与超文本对象关联的链接之一已被激活。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hypertext_link_activated_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERTEXT_LINK_ACTIVATED as u32,
            func,
        ));
    }

    /**
     * 已选择与超文本对象关联的链接之一。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hypertext_link_selected_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERTEXT_LINK_SELECTED as u32,
            func,
        ));
    }

    /**
     * 包含字符串中此链接的起始索引已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hyperlink_start_index_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERLINK_START_INDEX_CHANGED as u32,
            func,
        ));
    }

    /**
     * 焦点已从一个超文本对象更改为另一个超文本对象，或者焦点已从非超文本对象移动到超文本对象，或者焦点已从超文本对象移动到非超文本对象。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hypertext_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_HYPERTEXT_CHANGED as u32, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 与超文本对象关联的超链接数已更改
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_hypertext_nlinks_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_HYPERTEXT_NLINKS_CHANGED as u32,
            func,
        ));
    }

    /**
     * 对象的属性已更改。另请参阅IA2_EVENT_TEXT_ATTRIBUTE_CHANGED。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_object_attribute_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED as u32,
            func,
        ));
    }

    /**
     * 演示文稿文档中更改的幻灯片或文字处理文档中的页面边界越过。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_page_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_PAGE_CHANGED as u32, func));
    }

    /**
     * 插入符号从一个部分移动到下一个部分。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_section_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_SECTION_CHANGED as u32, func));
    }

    /**
     * 表格标题已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_caption_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_CAPTION_CHANGED as u32,
            func,
        ));
    }

    /**
     * 表的列说明已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_column_description_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_COLUMN_DESCRIPTION_CHANGED as u32,
            func,
        ));
    }

    /**
     * 表的列标题已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_column_header_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_COLUMN_HEADER_CHANGED as u32,
            func,
        ));
    }

    /**
     * 表的数据已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_model_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_MODEL_CHANGED as u32,
            func,
        ));
    }

    /**
     * 表的行描述已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_row_description_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_ROW_DESCRIPTION_CHANGED as u32,
            func,
        ));
    }

    /**
     * 表的行标题已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_row_header_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_ROW_HEADER_CHANGED as u32,
            func,
        ));
    }

    /**
     * 表的摘要已更改。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_table_summary_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TABLE_SUMMARY_CHANGED as u32,
            func,
        ));
    }

    /**
     * 文本对象的属性已更改。另请参阅IA2_EVENT_OBJECT_ATTRIBUTE_CHANGED。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_attribute_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TEXT_ATTRIBUTE_CHANGED as u32,
            func,
        ));
    }

    /**
     * 插入符号已移至新位置。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_caret_moved_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_TEXT_CARET_MOVED as u32, func));
    }

    /**
     * 荒废的。此事件等效于 IA2_EVENT_TEXT_UPDATED。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TEXT_ATTRIBUTE_CHANGED as u32,
            func,
        ));
    }

    /**
     * 插入符号从一列移动到下一列。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_column_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TEXT_COLUMN_CHANGED as u32,
            func,
        ));
    }

    /**
     * 插入了文本。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_inserted_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_TEXT_INSERTED as u32, func));
    }

    /**
     * 文本已删除。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_removed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_TEXT_REMOVED as u32, func));
    }

    /**
     * 此事件指示常规文本更改，即对通过 IAccessibleText 接口公开的文本的更改。为了与没有等效事件的 ATK/AT-SPI 兼容，服务器也可以触发IA2_EVENT_TEXT_REMOVED和IA2_EVENT_TEXT_INSERTED。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_updated_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_TEXT_UPDATED as u32, func));
    }

    //noinspection SpellCheckingInspection
    /**
     * 文本选择已更改。更高版本的Microsoft开发环境标识了等效事件，EVENT_OBJECT_TEXTSELECTIONCHANGED。如果可用，服务器应使用它，否则IA2_EVENT_TEXT_SELECTION_CHANGED使用。客户端应准备好响应任一事件。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_text_selection_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_TEXT_SELECTION_CHANGED as u32,
            func,
        ));
    }

    /**
     * 可见数据事件指示可访问对象的视觉外观的更改。例如，这包括通过IAccessibleComponent接口提供的大多数属性。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_visible_data_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events.write().unwrap().push(WinEventHook::new(
            IA2_EVENT_VISIBLE_DATA_CHANGED as u32,
            func,
        ));
    }

    /**
     * 角色以改变。
     * `func` 用于接收事件的监听器函数。
     * */
    pub fn add_on_role_changed_listener(
        &self,
        func: impl Fn(WinEventSource) + Sync + Send + 'static,
    ) {
        self.events
            .write()
            .unwrap()
            .push(WinEventHook::new(IA2_EVENT_ROLE_CHANGED as u32, func));
    }

    /**
     * 移除所有注册的监听器。
     * */
    pub fn remove_all_listeners(&self) {
        let mut lock = self.events.write().unwrap();
        for i in lock.iter() {
            i.unhook();
        }
        lock.clear();
    }
}

#[cfg(test)]
mod test_ia2 {
    use crate::ia2::object::Accessible2Object;
    use crate::ia2::text::AccessibleText;
    use crate::ia2::{Ia2, WinEventSourceExt};
    use crate::IAccessible2Lib::IA2CommonTypes::IA2CoordinateType;
    use win_wrap::com::co_initialize_multi_thread;
    use win_wrap::common::beep;

    #[test]
    fn main() {
        co_initialize_multi_thread().unwrap();
        let ia2 = Ia2::new();
        ia2.add_on_text_caret_moved_listener(|src| {
            beep(500, 50);
            let (obj, child) = src.get_object().unwrap();
            dbg!(child);
            let obj = Accessible2Object::from_accessible_object(obj).unwrap();
            let group_position = obj.group_position();
            dbg!(group_position);
            let h_wnd = obj.window_handle();
            assert_eq!(h_wnd, src.h_wnd);
            let index_in_parent = obj.index_in_parent();
            dbg!(index_in_parent);
            dbg!(obj);
            let text = src.get_text().unwrap();
            // text.add_selection(0, 1);
            dbg!(text.character_extents(0, IA2CoordinateType::IA2_COORDTYPE_PARENT_RELATIVE));
            dbg!(text);
        });
        std::thread::sleep(std::time::Duration::from_millis(20000));
        dbg!(ia2);
    }
}
