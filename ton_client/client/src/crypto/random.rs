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

extern crate rand;
use rand::RngCore;

pub fn generate_bytes(len: usize) -> Vec<u8> {
    let mut rng = rand::rngs::OsRng::new().unwrap();
    let mut result: Vec<u8> = Vec::new();
    result.resize(len, 0);
    rng.fill_bytes(&mut result);
    result
}

