#!/bin/bash
# Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by General Public License that can be found
# in the LICENSE file.

which wasm-pack || cargo install wasm-pack
wasm-pack build --target web
