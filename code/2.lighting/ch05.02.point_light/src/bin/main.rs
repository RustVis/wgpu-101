// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use ch05_02_point_light::{run, Error};

fn main() -> Result<(), Error> {
    pollster::block_on(run())
}
