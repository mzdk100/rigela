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

use crate::jab::context::AccessibleContext;
use std::sync::Arc;

pub(crate) type AccessibleContextType = Arc<AccessibleContext<'static>>;

pub(crate) enum AccessibleCallback {
    CaretUpdate(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    FocusGained(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseClicked(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseEntered(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseExited(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MousePressed(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MouseReleased(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MenuCanceled(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MenuDeselected(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    MenuSelected(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PopupMenuCanceled(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PopupMenuWillBecomeInvisible(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PopupMenuWillBecomeVisible(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertySelectionChange(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertyTextChange(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertyVisibleDataChange(Box<dyn Fn(AccessibleContextType) + Sync + Send>),
    PropertyChange(Box<dyn Fn(AccessibleContextType, String, String, String) + Sync + Send>),
    PropertyNameChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyDescriptionChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyStateChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyValueChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    PropertyCaretChange(Box<dyn Fn(AccessibleContextType, i32, i32) + Sync + Send>),
    PropertyChildChange(
        Box<
            dyn Fn(AccessibleContextType, AccessibleContextType, AccessibleContextType)
            + Sync
            + Send,
        >,
    ),
    PropertyActiveDescendentChange(
        Box<
            dyn Fn(AccessibleContextType, AccessibleContextType, AccessibleContextType)
            + Sync
            + Send,
        >,
    ),
    PropertyTableModelChange(Box<dyn Fn(AccessibleContextType, String, String) + Sync + Send>),
    JavaShutdown(Box<dyn Fn(i32) + Sync + Send>),
}

#[macro_export]
macro_rules! add_event_fp {
    (general,$lib:expr,$store:expr,$cb_name:ident,$func_name:ident,$type:path,$origin_name:ident,$doc:literal) => {
        extern "cdecl" fn $cb_name(vm_id: i32, event: JObject64, source: JObject64) {
            let source = unsafe {
                let Some(lib) = $lib.get() else {
                    return;
                };
                let source = Arc::new(AccessibleContext::new(lib, vm_id, source));
                lib.release_java_object(vm_id, event);
                source
            };

            let lock = $store.lock().unwrap();
            lock.iter().for_each(move |cb| {
                if let $type(f) = cb {
                    f(source.clone());
                }
            });
        }
        impl Jab {
            #[doc=concat!("添加", $doc,"监听器\n","`func` 一个监听器函数或闭包。")]
            pub fn $func_name(&self, func: impl Fn(AccessibleContextType) + Sync + Send + 'static) {
                static $origin_name: OnceLock<()> = OnceLock::new();
                $origin_name.get_or_init(|| self._lib.$origin_name($cb_name));

                let mut lock = $store.lock().unwrap();
                lock.push($type(Box::new(func)));
            }
        }
    };

        (property_change,$lib:expr,$store:expr,$doc:literal) => {
            extern "cdecl" fn cb_property_change(vm_id: i32, event: JObject64, source: JObject64,property:*const u16,old_value:*const u16,new_value: *const u16) {
                let (source,property,old_value,new_value) = unsafe {
                    let Some(lib) = $lib.get() else {
                        return;
                    };
                    let source = Arc::new(AccessibleContext::new(lib, vm_id, source));
                    let property2 = property.to_string_utf16();
                    let old_value2 = old_value.to_string_utf16();
                    let new_value2 = new_value.to_string_utf16();
                    lib.release_java_object(vm_id, property as JObject64);
                    lib.release_java_object(vm_id, old_value as JObject64);
                    lib.release_java_object(vm_id, new_value as JObject64);

                    lib.release_java_object(vm_id, event);
                    (source,property2,old_value2,new_value2)
                };

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let AccessibleCallback::PropertyChange(f) = cb {
                        f(source.clone(), property.clone(),old_value.clone(),new_value.clone());
                    }
                });
            }
            impl Jab {
                #[doc=concat!("添加", $doc,"监听器\n","`func` 一个监听器函数或闭包。")]
                pub fn add_on_property_change_listener(&self, func: impl Fn(AccessibleContextType, String,String,String) + Sync + Send + 'static) {
                    static set_property_change_fp: OnceLock<()> = OnceLock::new();
                    set_property_change_fp.get_or_init(|| self._lib.set_property_change_fp(cb_property_change));

                    let mut lock = $store.lock().unwrap();
                    lock.push(AccessibleCallback::PropertyChange(Box::new(func)));
                }
            }
        };

        (property_x_change,$lib:expr,$store:expr,$cb_name:ident,$func_name:ident,$type:path,$origin_name:ident,$doc:literal) => {
            extern "cdecl" fn $cb_name(vm_id: i32, event: JObject64, source: JObject64,old_value:*const u16,new_value: *const u16) {
                let (source,old_value,new_value) = unsafe {
                    let Some(lib) = $lib.get() else {
                        return;
                    };
                    let source = Arc::new(AccessibleContext::new(lib, vm_id, source));
                    let old_value2 = old_value.to_string_utf16();
                    let new_value2 = new_value.to_string_utf16();
                    lib.release_java_object(vm_id, old_value as JObject64);
                    lib.release_java_object(vm_id, new_value as JObject64);

                    lib.release_java_object(vm_id, event);
                    (source,old_value2,new_value2)
                };

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let $type(f) = cb {
                        f(source.clone(), old_value.clone(),new_value.clone());
                    }
                });
            }
            impl Jab {
                #[doc=concat!("添加", $doc,"监听器\n","`func` 一个监听器函数或闭包。")]
                pub fn $func_name(&self, func: impl Fn(AccessibleContextType, String,String) + Sync + Send + 'static) {
                    static $origin_name: OnceLock<()> = OnceLock::new();
                    $origin_name.get_or_init(|| self._lib.$origin_name($cb_name));

                    let mut lock = $store.lock().unwrap();
                    lock.push($type(Box::new(func)));
                }
            }
        };

        (property_caret_change,$lib:expr,$store:expr,$doc:literal) => {
            extern "cdecl" fn cb_property_caret_change(vm_id: i32, event: JObject64, source: JObject64,old_value:i32,new_value: i32) {
                let source = unsafe {
                    let Some(lib) = $lib.get() else {
                        return;
                    };
                    let source = Arc::new(AccessibleContext::new(lib, vm_id, source));

                    lib.release_java_object(vm_id, event);
                    source
                };

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let AccessibleCallback::PropertyCaretChange(f) = cb {
                        f(source.clone(), old_value,new_value);
                    }
                });
            }
            impl Jab {
                #[doc=concat!("添加", $doc,"监听器\n","`func` 一个监听器函数或闭包。")]
                pub fn add_on_property_caret_change_listener(&self, func: impl Fn(AccessibleContextType, i32,i32) + Sync + Send + 'static) {
                    static set_property_caret_change_fp: OnceLock<()> = OnceLock::new();
                    set_property_caret_change_fp.get_or_init(|| self._lib.set_property_caret_change_fp(cb_property_caret_change));

                    let mut lock = $store.lock().unwrap();
                    lock.push(AccessibleCallback::PropertyCaretChange(Box::new(func)));
                }
            }
        };

        (property_y_change,$lib:expr,$store:expr,$cb_name:ident,$func_name:ident,$type:path,$origin_name:ident,$doc:literal) => {
            extern "cdecl" fn $cb_name(vm_id: i32, event: JObject64, source: JObject64,old_value:JObject64,new_value: JObject64) {
                let (source,old_value,new_value) = unsafe {
                    let Some(lib) = $lib.get() else {
                        return;
                    };
                    let source = Arc::new(AccessibleContext::new(lib, vm_id, source));
                    let old_value = Arc::new(AccessibleContext::new(lib,vm_id,old_value));
                    let new_value = Arc::new(AccessibleContext::new(lib,vm_id,new_value));

                    lib.release_java_object(vm_id, event);
                    (source,old_value,new_value)
                };

                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let $type(f) = cb {
                        f(source.clone(), old_value.clone(),new_value.clone());
                    }
                });
            }
            impl Jab {
                #[doc=concat!("添加", $doc,"监听器\n","`func` 一个监听器函数或闭包。")]
                pub fn $func_name(&self, func: impl Fn(AccessibleContextType, AccessibleContextType, AccessibleContextType) + Sync + Send + 'static) {
                    static $origin_name: OnceLock<()> = OnceLock::new();
                    $origin_name.get_or_init(|| self._lib.$origin_name($cb_name));

                    let mut lock = $store.lock().unwrap();
                    lock.push($type(Box::new(func)));
                }
            }
        };

        (java_shutdown,$lib:expr,$store:expr,$doc:literal) => {
            extern "cdecl" fn cb_java_shutdown(vm_id: i32) {
                let lock = $store.lock().unwrap();
                lock.iter().for_each(move |cb| {
                    if let AccessibleCallback::JavaShutdown(f) = cb {
                        f(vm_id);
                    }
                });
            }
            impl Jab {
                #[doc=concat!("添加", $doc,"监听器\n","`func` 一个监听器函数或闭包。")]
                pub fn add_on_java_shutdown_listener(&self, func: impl Fn(i32) + Sync + Send + 'static) {
                    static set_java_shutdown_fp: OnceLock<()> = OnceLock::new();
                    set_java_shutdown_fp.get_or_init(|| self._lib.set_java_shutdown_fp(cb_java_shutdown));

                    let mut lock = $store.lock().unwrap();
                    lock.push(AccessibleCallback::JavaShutdown(Box::new(func)));
                }
            }
        };
}
