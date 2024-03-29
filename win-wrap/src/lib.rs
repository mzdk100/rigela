/*
 * Copyright (c) 2023. The RigelA open source project team and
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

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "com")]
pub mod com;
#[cfg(feature = "common")]
pub mod common;
#[cfg(feature = "control")]
pub mod control;
#[cfg(feature = "ext")]
pub mod ext;
#[cfg(feature = "graphic")]
pub mod graphic;
#[cfg(feature = "hook")]
pub mod hook;
#[cfg(feature = "input")]
pub mod input;
#[cfg(feature = "memory")]
pub mod memory;
#[cfg(feature = "message")]
pub mod message;
#[cfg(feature = "msaa")]
pub mod msaa;
#[cfg(feature = "pth")]
pub mod pdh;
#[cfg(feature = "registry")]
pub mod registry;
#[cfg(feature = "shell")]
pub mod shell;
#[cfg(feature = "threading")]
pub mod threading;
#[cfg(feature = "tts")]
pub mod tts;
#[cfg(feature = "uia")]
pub mod uia;
