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

//noinspection SpellCheckingInspection
#[cfg(target_arch = "x86")]
pub(crate) mod ibmeci;

#[cfg(target_arch = "x86")]
use crate::tts::ibmeci::Ibmeci;

//noinspection SpellCheckingInspection
#[cfg(target_arch = "x86")]
pub(crate) async fn create_ibmeci() -> Ibmeci {
    use log::error;
    use rigela_resources::clone_resource;
    use rigela_utils::{get_program_directory, SERVER_HOME_URI};

    const LIB_NAME: &str = "ibmeci.dll";
    let url = format!("{}/{}", SERVER_HOME_URI, LIB_NAME);

    let eci_path = get_program_directory().join(LIB_NAME);
    let file = clone_resource(url, eci_path.clone()).await;
    if let Err(e) = file {
        error!("{}", e);
        return Ibmeci::null();
    }
    Ibmeci::new(&eci_path.to_str().unwrap())
}
