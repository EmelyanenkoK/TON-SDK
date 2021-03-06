/*
* Copyright 2018-2019 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.  You may obtain a copy of the
* License at: https://ton.dev/licenses
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

// Android React Native

#[cfg(target_os="android")]
extern crate jni;
#[cfg(target_os="android")]
mod android;
#[cfg(target_os="android")]
pub use self::android::*;

// IOS React Native

#[cfg(target_os="ios")]
extern crate libc;
#[cfg(target_os="ios")]
mod ios;
#[cfg(target_os="ios")]
pub use self::ios::*;
