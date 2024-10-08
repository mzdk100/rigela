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

#[cfg(feature = "bass")]
pub mod bass;
#[cfg(feature = "common")]
pub mod common;
#[cfg(feature = "clip")]
pub mod clip;
#[cfg(feature = "color")]
pub mod color;
#[cfg(feature = "fs")]
pub mod fs;
//noinspection SpellCheckingInspection
#[cfg(all(feature = "ibmeci", target_arch = "x86"))]
pub mod ibmeci;
#[cfg(feature = "killer")]
pub mod killer;
#[cfg(feature = "library")]
pub mod library;
#[cfg(feature = "logger")]
pub mod logger;
#[cfg(feature = "pipe")]
pub mod pipe;
#[cfg(feature = "screen")]
pub mod screen;

//noinspection HttpUrlsUsage
pub const SERVER_HOME_URI: &str = "http://rigela.site/rigela";
