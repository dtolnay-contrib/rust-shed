/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use once_cell::sync::Lazy;

#[cfg(any(fbcode_build, feature = "fb"))]
mod facebook;
#[cfg(any(fbcode_build, feature = "fb"))]
pub use facebook::get_env;
#[cfg(any(fbcode_build, feature = "fb"))]
pub use facebook::is_corp;
#[cfg(any(fbcode_build, feature = "fb"))]
pub use facebook::is_lab;
#[cfg(any(fbcode_build, feature = "fb"))]
pub use facebook::is_prod;
#[cfg(any(fbcode_build, feature = "fb"))]
pub use facebook::Env;

pub static IN_PROD: Lazy<bool> = Lazy::new(is_prod);
pub static IN_CORP: Lazy<bool> = Lazy::new(is_corp);
pub static IN_LAB: Lazy<bool> = Lazy::new(is_lab);

#[cfg(not(any(fbcode_build, feature = "fb")))]
pub fn get_env() -> u8 {
    0
}

#[cfg(not(any(fbcode_build, feature = "fb")))]
pub fn is_prod() -> bool {
    false
}

#[cfg(not(any(fbcode_build, feature = "fb")))]
pub fn is_corp() -> bool {
    false
}

#[cfg(not(any(fbcode_build, feature = "fb")))]
pub fn is_lab() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn fb_get_env() -> u8 {
    get_env() as u8
}

#[no_mangle]
pub extern "C" fn fb_is_prod() -> bool {
    *IN_PROD
}

#[no_mangle]
pub extern "C" fn fb_is_corp() -> bool {
    *IN_CORP
}

#[no_mangle]
pub extern "C" fn fb_is_lab() -> bool {
    *IN_LAB
}

#[no_mangle]
pub extern "C" fn fb_has_servicerouter() -> bool {
    *IN_PROD
}
