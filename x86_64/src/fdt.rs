// Copyright 2018 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fs::File;

use arch::android::create_android_fdt;
use cros_fdt::Error;
use cros_fdt::FdtWriter;

use crate::SetupData;
use crate::SetupDataType;

/// Creates a flattened device tree containing all of the parameters for the
/// kernel and returns it as `SetupData`.
///
/// # Arguments
///
/// * `android_fstab` - the File object for the android fstab
pub fn create_fdt(android_fstab: File) -> Result<SetupData, Error> {
    let mut fdt = FdtWriter::new(&[]);

    // The whole thing is put into one giant node with some top level properties
    let root_node = fdt.begin_node("")?;
    create_android_fdt(&mut fdt, android_fstab)?;
    fdt.end_node(root_node)?;

    let fdt_final = fdt.finish()?;

    Ok(SetupData {
        data: fdt_final,
        type_: SetupDataType::Dtb,
    })
}
