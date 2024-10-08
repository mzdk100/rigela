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

/*
演示如何加载默认插件并处理程序退出。
Com1组件添加到框架时，初始内部值是10，随着countdown系统运行10次后，组件的内部值变为0，然后退出程序。
*/

use rigela_framework::{
    basic::{AppRunner, Comp, Component},
    builtin::{DefaultPlugin, Terminator},
};

#[derive(Debug)]
struct Com1(u8);

impl Component for Com1 {}

fn countdown(mut a: Comp<Com1>, mut terminator: Comp<Terminator>) {
    println!("{:?}", a);
    if a.0 < 1 {
        terminator.terminate();
    } else {
        a.0 -= 1;
    }
}

fn main() {
    AppRunner::new()
        .add_plugin(DefaultPlugin)
        .add_component(Com1(10))
        .add_system(countdown)
        .run();
}
