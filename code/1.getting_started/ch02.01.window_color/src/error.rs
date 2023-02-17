// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] io::Error),

    #[error("window error")]
    Winit(#[from] winit::error::OsError),

    #[error("web error")]
    Web(String),
}

#[cfg(target_arch = "wasm32")]
impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&format!("{self:?}"))
    }
}

#[cfg(target_arch = "wasm32")]
impl From<JsValue> for Error {
    fn from(val: JsValue) -> Self {
        Self::Web(val.as_string().unwrap_or_default())
    }
}
