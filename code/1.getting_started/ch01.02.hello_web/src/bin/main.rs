// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use ch01_02_hello_web::run;

fn main() {
    pollster::block_on(run());
}
